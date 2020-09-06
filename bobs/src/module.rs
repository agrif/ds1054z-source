pub trait ModuleInfo: Sized {
    const NAME: &'static str;
    const DESCRIPTION: &'static str;
    const AUTHOR: &'static str;
    fn load(registrar: &mut Registrar) -> Option<Self>;

    fn post_load(&mut self) {}
    fn set_locale(&mut self, _locale: &str) {}
    fn free_locale(&mut self) {}
}

pub struct Registrar {
    infos: Vec<Box<dyn crate::Register>>,
}

impl Registrar {
    pub unsafe fn new() -> Self {
        Registrar { infos: vec![] }
    }

    pub fn register<T>(&mut self, info: T)
    where
        T: crate::Register + 'static,
    {
        let mut boxed: Box<dyn crate::Register> = Box::new(info);
        unsafe {
            boxed.register();
        }
        self.infos.push(boxed);
    }
}

#[macro_export]
macro_rules! declare_module {
    ($t:ident) => {
        pub unsafe fn current_module() -> ::std::option::Option<*mut ::obs_sys::obs_module_t> {
            exports::OBS_MODULE
        }

        pub mod exports {
            use ::std::ffi::{CStr, CString};
            use ::std::option::Option;
            use ::std::option::Option::{None, Some};
            use ::std::os::raw::c_char;
            use $crate::ModuleInfo;

            pub(super) static mut OBS_MODULE: Option<*mut ::obs_sys::obs_module_t> = None;
            static mut OBS_MODULE_INFO: Option<(super::$t, $crate::Registrar)> = None;

            #[no_mangle]
            pub unsafe extern "C" fn obs_module_set_pointer(module: *mut ::obs_sys::obs_module_t) {
                $crate::log::init();
                OBS_MODULE = Some(module);
            }

            #[no_mangle]
            pub unsafe extern "C" fn obs_module_ver() -> ::std::primitive::u32 {
                $crate::log::init();
                ::obs_sys::LIBOBS_API_MAJOR_VER
            }

            #[no_mangle]
            pub unsafe extern "C" fn obs_module_name() -> *const c_char {
                $crate::log::init();
                static mut NAME: Option<CString> = None;
                NAME.get_or_insert_with(|| {
                    CString::new(<super::$t as ModuleInfo>::NAME).expect("invalid name string")
                })
                .as_ptr()
            }

            #[no_mangle]
            pub unsafe extern "C" fn obs_module_description() -> *const c_char {
                $crate::log::init();
                static mut DESC: Option<CString> = None;
                DESC.get_or_insert_with(|| {
                    CString::new(<super::$t as ModuleInfo>::DESCRIPTION)
                        .expect("invalid description string")
                })
                .as_ptr()
            }

            #[no_mangle]
            pub unsafe extern "C" fn obs_module_author() -> *const c_char {
                $crate::log::init();
                static mut AUTH: Option<CString> = None;
                AUTH.get_or_insert_with(|| {
                    CString::new(<super::$t as ModuleInfo>::AUTHOR).expect("invalid author string")
                })
                .as_ptr()
            }

            #[no_mangle]
            pub unsafe extern "C" fn obs_module_load() -> ::std::primitive::bool {
                $crate::log::init();
                let mut r = $crate::Registrar::new();
                if let None = OBS_MODULE_INFO {
                    OBS_MODULE_INFO = ModuleInfo::load(&mut r).map(|i| (i, r));
                }
                OBS_MODULE_INFO.is_some()
            }

            #[no_mangle]
            pub unsafe extern "C" fn obs_module_post_load() {
                $crate::log::init();
                if let Some((ref mut info, _)) = OBS_MODULE_INFO {
                    <super::$t as ModuleInfo>::post_load(info);
                }
            }

            #[no_mangle]
            pub unsafe extern "C" fn obs_module_unload() {
                $crate::log::init();
                OBS_MODULE_INFO = None;
            }

            #[no_mangle]
            pub unsafe extern "C" fn obs_module_set_locale(locale: *const c_char) {
                $crate::log::init();
                if let Some((ref mut info, _)) = OBS_MODULE_INFO {
                    let rustlocale = CStr::from_ptr(locale)
                        .to_str()
                        .expect("invalid utf-8 in string");
                    <super::$t as ModuleInfo>::set_locale(info, rustlocale);
                }
            }

            #[no_mangle]
            pub unsafe extern "C" fn obs_module_free_locale() {
                $crate::log::init();
                if let Some((ref mut info, _)) = OBS_MODULE_INFO {
                    <super::$t as ModuleInfo>::free_locale(info);
                }
            }
        }
    };
}
