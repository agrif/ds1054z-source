use smol_timeout::TimeoutExt;
use std::sync::mpsc;
use std::time::{Duration, Instant};

#[derive(Debug)]
struct ThreadSafePtr<T>(*mut T);
unsafe impl<T> Send for ThreadSafePtr<T> {}

#[derive(Debug)]
pub struct ScopeSource {
    channel: mpsc::Sender<Message>,
    thread: Option<std::thread::JoinHandle<()>>,
}

#[derive(Debug, Default)]
struct Settings {
    address: String,
    blank: bool,
}

#[derive(Debug)]
enum Message {
    End,
    Update(Settings),
}

impl bobs::Source for ScopeSource {
    const ID: &'static str = "ds1054z";
    const NAME: &'static str = "Rigol DS1054Z";
    const ICON_TYPE: bobs::IconType = bobs::IconType::WindowCapture;

    fn output_flags() -> bobs::SourceFlags {
        bobs::SourceFlags::ASYNC_VIDEO
    }

    fn create(settings: &bobs::Data, source: *mut obs_sys::obs_source_t) -> Self {
        let (tx, rx) = std::sync::mpsc::channel();
        let source = ThreadSafePtr(source);
        let thread = std::thread::spawn(move || smol::block_on(Self::video_thread(rx, source.0)));
        let mut src = ScopeSource {
            thread: Some(thread),
            channel: tx,
        };
         src.update(settings);
        src
    }

    fn get_properties(&mut self) -> bobs::Properties {
        let mut props = bobs::Properties::create();
        props.add_text("address", "Oscilloscope address", bobs::TextType::Default);
        props.add_bool("blank", "Blank when disconnected.");
        props
    }

    fn get_defaults(settings: &mut bobs::Data) {
        settings.set_default_string("address", "ds1054z.local:555");
        settings.set_default_bool("blank", true);
    }

    fn update(&mut self, settings: &bobs::Data) {
        let settings = Settings {
            address: settings.get_string("address").to_owned(),
            blank: settings.get_bool("blank"),
        };
        self.channel
            .send(Message::Update(settings))
            .expect("could not update settings");
    }
}

impl ScopeSource {
    async fn video_thread(channel: mpsc::Receiver<Message>, source: *mut obs_sys::obs_source_t) {
        // set up state
        let mut settings: Settings = Default::default();
        let mut scope = None;

        // set up frame
        let mut frame = obs_sys::obs_source_frame {
            format: obs_sys::video_format_VIDEO_FORMAT_RGBA,
            width: 800,
            height: 480,
            ..Default::default()
        };
        frame.linesize[0] = 4 * frame.width;
        let mut data: Vec<u8> = vec![0xff; (4 * frame.width * frame.height) as usize];
        frame.data[0] = data.as_slice().as_ptr() as *mut u8;

        loop {
            // wait until this time at end of loop...
            let loop_end = Instant::now() + Duration::from_millis(100);

            // attempt to connect
            if let None = scope {
                if settings.address != "" {
                    if let Some(Ok(s)) = ds1054z::Scope::connect(&settings.address)
                        .timeout(Duration::from_millis(1000))
                        .await
                    {
                        log::info!("connected to {}", settings.address);
                        scope = Some(s);
                    }
                }
            }

            // grab a frame
            if let Some(mut s) = scope.take() {
                let bmpr = s.grab_screen().timeout(Duration::from_millis(2000)).await;
                if let Some(Ok(bmp)) = bmpr {
                    // expand frame, if needed
                    let newsize = (4 * bmp.width() * bmp.height()) as usize;
                    if data.len() < newsize {
                        data.resize(newsize, 0xff);
                    }
                    frame.linesize[0] = 4 * bmp.width();
                    frame.width = bmp.width();
                    frame.height = bmp.height();

                    // fill frame
                    for (i, v) in bmp.data().iter().enumerate() {
                        let dst = (i / 3) + i;
                        data[dst] = *v;
                    }

                    // present texture
                    unsafe {
                        obs_sys::obs_source_output_video(source, &frame);
                    }

                    // re-use scope
                    scope = Some(s);
                } else {
                    log::info!("disconnected from {}", settings.address);
                }
            } else if settings.blank {
                // fill frame with black
                for (i, v) in data.iter_mut().enumerate() {
                    if i % 4 != 3 {
                        *v = 0;
                    }
                }

                // present texture
                unsafe {
                    obs_sys::obs_source_output_video(source, &frame);
                }
            }

            // look for end-thread message
            match channel.try_recv() {
                Ok(Message::End) => return,
                Ok(Message::Update(s)) => {
                    if scope.is_some() {
                        log::info!("disconnected from {}", settings.address);
                    }
                    settings = s;
                    scope = None;
                }
                Err(mpsc::TryRecvError::Disconnected) => return,
                Err(mpsc::TryRecvError::Empty) => (),
            }

            // don't busy-loop
            let now = Instant::now();
            if now < loop_end {
                std::thread::sleep(loop_end - now);
            }
        }
    }
}

impl Drop for ScopeSource {
    fn drop(&mut self) {
        if let Some(thread) = self.thread.take() {
            // send end-thread message
            self.channel
                .send(Message::End)
                .expect("could not end thread");
            thread.join().expect("could not join thread");
        }
    }
}
