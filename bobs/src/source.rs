use crate::string::cstring;
use std::ffi::{c_void, CString};
use std::os::raw::c_char;
use std::ptr::NonNull;

pub trait Source: Sized {
    const ID: &'static str;
    const NAME: &'static str;
    const TYPE: SourceType = SourceType::Input;
    const ICON_TYPE: IconType = IconType::Unknown;

    // const fn restrictions need this to be a function
    fn output_flags() -> SourceFlags {
        SourceFlags::empty()
    }

    fn create(settings: &crate::Data, source: *mut obs_sys::obs_source_t) -> Self;

    fn get_properties(&mut self) -> crate::Properties {
        crate::Properties::create()
    }
    fn get_defaults(_settings: &mut crate::Data) {}
    fn update(&mut self, _settings: &crate::Data) {}

    fn info() -> SourceInfo<Self> {
        SourceInfo::new()
    }
}

#[derive(Debug, Clone)]
pub struct SourceInfo<T> {
    raw: obs_sys::obs_source_info,
    _marker: std::marker::PhantomData<fn() -> T>,
}

impl<T> SourceInfo<T>
where
    T: Source,
{
    pub unsafe fn into_raw(self) -> obs_sys::obs_source_info {
        self.raw
    }

    pub unsafe fn from_raw(raw: obs_sys::obs_source_info) -> Self {
        SourceInfo {
            _marker: std::marker::PhantomData,
            raw,
        }
    }

    fn new() -> Self {
        static mut ID: Option<CString> = None;
        let cid = unsafe { ID.get_or_insert_with(|| cstring(T::ID)).as_ptr() };
        SourceInfo {
            _marker: std::marker::PhantomData,
            raw: obs_sys::obs_source_info {
                id: cid,
                type_: T::TYPE.into_raw(),
                output_flags: T::output_flags().into_raw(),
                icon_type: T::ICON_TYPE.into_raw(),

                get_name: Some(Self::get_name),
                create: Some(Self::create),
                destroy: Some(Self::destroy),
                get_properties: Some(Self::get_properties),
                get_defaults: Some(Self::get_defaults),
                update: Some(Self::update),

                ..Default::default()
            },
        }
    }

    unsafe extern "C" fn get_name(_type_data: *mut c_void) -> *const c_char {
        static mut NAME: Option<CString> = None;
        NAME.get_or_insert_with(|| cstring(T::NAME)).as_ptr()
    }

    unsafe extern "C" fn create(
        settings: *mut obs_sys::obs_data_t,
        source: *mut obs_sys::obs_source_t,
    ) -> *mut c_void {
        let settings = crate::Data::from_raw_unowned(NonNull::new(settings).expect("null pointer"));
        let src = Box::new(T::create(&settings, source));
        Box::into_raw(src) as *mut c_void
    }

    unsafe extern "C" fn destroy(data: *mut c_void) {
        let src = Box::from_raw(data as *mut T);
        std::mem::drop(src);
    }

    unsafe extern "C" fn get_properties(data: *mut c_void) -> *mut obs_sys::obs_properties_t {
        let mut src = Box::from_raw(data as *mut T);
        let r = src.get_properties();
        std::mem::forget(src);
        r.into_raw().as_ptr()
    }

    unsafe extern "C" fn get_defaults(settings: *mut obs_sys::obs_data_t) {
        let mut settings =
            crate::Data::from_raw_unowned(NonNull::new(settings).expect("null pointer"));
        T::get_defaults(&mut settings);
    }

    unsafe extern "C" fn update(data: *mut c_void, settings: *mut obs_sys::obs_data_t) {
        let mut src = Box::from_raw(data as *mut T);
        let settings = crate::Data::from_raw_unowned(NonNull::new(settings).expect("null pointer"));
        src.update(&settings);
        std::mem::forget(src);
    }
}

impl<T> crate::Register for SourceInfo<T> {
    unsafe fn register(&mut self) {
        obs_sys::obs_register_source_s(
            &mut self.raw,
            std::mem::size_of::<obs_sys::obs_source_info>() as obs_sys::size_t,
        );
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum SourceType {
    Input,
    Filter,
    Transition,
}

impl SourceType {
    pub fn into_raw(self) -> obs_sys::obs_source_type {
        match self {
            SourceType::Input => obs_sys::obs_source_type_OBS_SOURCE_TYPE_INPUT,
            SourceType::Filter => obs_sys::obs_source_type_OBS_SOURCE_TYPE_FILTER,
            SourceType::Transition => obs_sys::obs_source_type_OBS_SOURCE_TYPE_TRANSITION,
        }
    }

    pub fn from_raw(raw: obs_sys::obs_source_type) -> Option<Self> {
        match raw {
            obs_sys::obs_source_type_OBS_SOURCE_TYPE_INPUT => Some(SourceType::Input),
            obs_sys::obs_source_type_OBS_SOURCE_TYPE_FILTER => Some(SourceType::Filter),
            obs_sys::obs_source_type_OBS_SOURCE_TYPE_TRANSITION => Some(SourceType::Transition),
            _ => None,
        }
    }
}

bitflags::bitflags! {
    pub struct SourceFlags: u32 {
        const VIDEO = obs_sys::OBS_SOURCE_VIDEO;
        const AUDIO = obs_sys::OBS_SOURCE_AUDIO;
        const ASYNC = obs_sys::OBS_SOURCE_ASYNC;
        const ASYNC_VIDEO = obs_sys::OBS_SOURCE_ASYNC_VIDEO;
        const CUSTOM_DRAW = obs_sys::OBS_SOURCE_CUSTOM_DRAW;
        const INTERACTION = obs_sys::OBS_SOURCE_INTERACTION;
        const COMPOSITE = obs_sys::OBS_SOURCE_COMPOSITE;
        const DO_NOT_DUPLICATE = obs_sys::OBS_SOURCE_DO_NOT_DUPLICATE;
        const DEPRECATED = obs_sys::OBS_SOURCE_DEPRECATED;
        const DO_NOT_SELF_MONITOR = obs_sys::OBS_SOURCE_DO_NOT_SELF_MONITOR;
    }
}

impl SourceFlags {
    pub fn into_raw(self) -> u32 {
        self.bits()
    }

    pub fn from_raw(raw: u32) -> Option<Self> {
        Self::from_bits(raw)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum IconType {
    AudioInput,
    AudioOutput,
    Browser,
    Camera,
    Color,
    Custom,
    DesktopCapture,
    GameCapture,
    Image,
    Media,
    Slideshow,
    Text,
    Unknown,
    WindowCapture,
}

impl IconType {
    pub fn into_raw(self) -> obs_sys::obs_icon_type {
        match self {
            IconType::AudioInput => obs_sys::obs_icon_type_OBS_ICON_TYPE_AUDIO_INPUT,
            IconType::AudioOutput => obs_sys::obs_icon_type_OBS_ICON_TYPE_AUDIO_OUTPUT,
            IconType::Browser => obs_sys::obs_icon_type_OBS_ICON_TYPE_BROWSER,
            IconType::Camera => obs_sys::obs_icon_type_OBS_ICON_TYPE_CAMERA,
            IconType::Color => obs_sys::obs_icon_type_OBS_ICON_TYPE_COLOR,
            IconType::Custom => obs_sys::obs_icon_type_OBS_ICON_TYPE_CUSTOM,
            IconType::DesktopCapture => obs_sys::obs_icon_type_OBS_ICON_TYPE_DESKTOP_CAPTURE,
            IconType::GameCapture => obs_sys::obs_icon_type_OBS_ICON_TYPE_GAME_CAPTURE,
            IconType::Image => obs_sys::obs_icon_type_OBS_ICON_TYPE_IMAGE,
            IconType::Media => obs_sys::obs_icon_type_OBS_ICON_TYPE_MEDIA,
            IconType::Slideshow => obs_sys::obs_icon_type_OBS_ICON_TYPE_SLIDESHOW,
            IconType::Text => obs_sys::obs_icon_type_OBS_ICON_TYPE_TEXT,
            IconType::Unknown => obs_sys::obs_icon_type_OBS_ICON_TYPE_UNKNOWN,
            IconType::WindowCapture => obs_sys::obs_icon_type_OBS_ICON_TYPE_WINDOW_CAPTURE,
        }
    }

    pub fn from_raw(raw: obs_sys::obs_icon_type) -> Option<Self> {
        match raw {
            obs_sys::obs_icon_type_OBS_ICON_TYPE_AUDIO_INPUT => Some(IconType::AudioInput),
            obs_sys::obs_icon_type_OBS_ICON_TYPE_AUDIO_OUTPUT => Some(IconType::AudioOutput),
            obs_sys::obs_icon_type_OBS_ICON_TYPE_BROWSER => Some(IconType::Browser),
            obs_sys::obs_icon_type_OBS_ICON_TYPE_CAMERA => Some(IconType::Camera),
            obs_sys::obs_icon_type_OBS_ICON_TYPE_COLOR => Some(IconType::Color),
            obs_sys::obs_icon_type_OBS_ICON_TYPE_CUSTOM => Some(IconType::Custom),
            obs_sys::obs_icon_type_OBS_ICON_TYPE_DESKTOP_CAPTURE => Some(IconType::DesktopCapture),
            obs_sys::obs_icon_type_OBS_ICON_TYPE_GAME_CAPTURE => Some(IconType::GameCapture),
            obs_sys::obs_icon_type_OBS_ICON_TYPE_IMAGE => Some(IconType::Image),
            obs_sys::obs_icon_type_OBS_ICON_TYPE_MEDIA => Some(IconType::Media),
            obs_sys::obs_icon_type_OBS_ICON_TYPE_SLIDESHOW => Some(IconType::Slideshow),
            obs_sys::obs_icon_type_OBS_ICON_TYPE_TEXT => Some(IconType::Text),
            obs_sys::obs_icon_type_OBS_ICON_TYPE_UNKNOWN => Some(IconType::Unknown),
            obs_sys::obs_icon_type_OBS_ICON_TYPE_WINDOW_CAPTURE => Some(IconType::WindowCapture),
            _ => None,
        }
    }
}
