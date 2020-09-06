use std::os::raw::c_char;

pub fn cstring<T>(s: T) -> std::ffi::CString where T: Into<Vec<u8>> {
    std::ffi::CString::new(s).expect("bad C string")
}

pub unsafe fn string_ref<'a>(s: *const c_char) -> &'a str {
    std::ffi::CStr::from_ptr(s).to_str().expect("bad utf-8 string")
}

//pub unsafe fn string_owned(s: *const c_char) -> String {
//    std::ffi::CStr::from_ptr(s).to_str().expect("bad utf-8 string").to_owned()
//}
