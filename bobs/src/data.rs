use crate::string::{cstring, string_ref};
use crate::{ObsRawBox, ObsRawCounted};
use std::ptr::NonNull;

#[derive(Debug)]
#[repr(C)]
pub struct Data(obs_sys::obs_data_t);

impl ObsRawBox for Data {
    type Raw = NonNull<obs_sys::obs_data_t>;

    unsafe fn from_raw(raw: Self::Raw) -> Box<Self> {
        Box::from_raw(std::mem::transmute(raw.as_ptr()))
    }

    unsafe fn as_raw(&self) -> Self::Raw {
        (&self.0).into()
    }
}

impl ObsRawCounted for Data {
    unsafe fn addref(&self) {
        obs_sys::obs_data_addref(self.as_raw().as_ptr());
    }

    unsafe fn release(&self) {
        obs_sys::obs_data_release(self.as_raw().as_ptr());
    }
}

impl Drop for Data {
    fn drop(&mut self) {
        unsafe { self.release() }
    }
}

impl Data {
    pub fn create() -> Box<Self> {
        unsafe {
            Self::from_raw(NonNull::new(obs_sys::obs_data_create()).expect("pointer is null"))
        }
    }

    pub fn create_from_json(json_string: &str) -> Option<Box<Self>> {
        let cjson = cstring(json_string);
        unsafe {
            Some(Self::from_raw(NonNull::new(
                obs_sys::obs_data_create_from_json(cjson.as_ptr()),
            )?))
        }
    }

    pub fn create_from_json_file<P>(json_file: P) -> Option<Box<Self>>
    where
        P: AsRef<std::path::Path>,
    {
        let cpath = cstring(json_file.as_ref().to_str().expect("bad path"));
        unsafe {
            Some(Self::from_raw(NonNull::new(
                obs_sys::obs_data_create_from_json_file(cpath.as_ptr()),
            )?))
        }
    }

    pub fn create_from_json_file_safe<P>(json_file: P, backup_ext: &str) -> Option<Box<Self>>
    where
        P: AsRef<std::path::Path>,
    {
        let cpath = cstring(json_file.as_ref().to_str().expect("bad path"));
        let cext = cstring(backup_ext);
        unsafe {
            Some(Self::from_raw(NonNull::new(
                obs_sys::obs_data_create_from_json_file_safe(cpath.as_ptr(), cext.as_ptr()),
            )?))
        }
    }

    pub fn get_json(&mut self) -> &str {
        // mut self because this re-uses an internal buffer
        unsafe { string_ref(obs_sys::obs_data_get_json(self.as_raw().as_ptr())) }
    }

    pub fn save_json<P>(&self, file: P)
    where
        P: AsRef<std::path::Path>,
    {
        let cpath = cstring(file.as_ref().to_str().expect("bad path"));
        unsafe {
            obs_sys::obs_data_save_json(self.as_raw().as_ptr(), cpath.as_ptr());
        }
    }

    pub fn save_json_safe<P>(&self, file: P, temp_ext: &str, backup_ext: &str)
    where
        P: AsRef<std::path::Path>,
    {
        let cpath = cstring(file.as_ref().to_str().expect("bad path"));
        let ctext = cstring(temp_ext);
        let cbext = cstring(backup_ext);
        unsafe {
            obs_sys::obs_data_save_json_safe(
                self.as_raw().as_ptr(),
                cpath.as_ptr(),
                ctext.as_ptr(),
                cbext.as_ptr(),
            );
        }
    }

    pub fn apply(&mut self, apply_data: &Data) {
        unsafe {
            obs_sys::obs_data_apply(self.as_raw().as_ptr(), apply_data.as_raw().as_ptr());
        }
    }

    pub fn erase(&mut self, name: &str) {
        let cname = cstring(name);
        unsafe {
            obs_sys::obs_data_erase(self.as_raw().as_ptr(), cname.as_ptr());
        }
    }

    pub fn clear(&mut self) {
        unsafe {
            obs_sys::obs_data_clear(self.as_raw().as_ptr());
        }
    }

    pub fn set_string(&mut self, name: &str, val: &str) {
        let cname = cstring(name);
        let cval = cstring(val);
        unsafe {
            obs_sys::obs_data_set_string(self.as_raw().as_ptr(), cname.as_ptr(), cval.as_ptr());
        }
    }

    pub fn set_int(&mut self, name: &str, val: i64) {
        let cname = cstring(name);
        unsafe {
            obs_sys::obs_data_set_int(
                self.as_raw().as_ptr(),
                cname.as_ptr(),
                val as std::os::raw::c_longlong,
            );
        }
    }

    pub fn set_double(&mut self, name: &str, val: f64) {
        let cname = cstring(name);
        unsafe {
            obs_sys::obs_data_set_double(self.as_raw().as_ptr(), cname.as_ptr(), val);
        }
    }

    pub fn set_bool(&mut self, name: &str, val: bool) {
        let cname = cstring(name);
        unsafe {
            obs_sys::obs_data_set_bool(self.as_raw().as_ptr(), cname.as_ptr(), val);
        }
    }

    pub fn set_obj(&mut self, name: &str, val: &Data) {
        let cname = cstring(name);
        unsafe {
            obs_sys::obs_data_set_obj(
                self.as_raw().as_ptr(),
                cname.as_ptr(),
                val.as_raw().as_ptr(),
            );
        }
    }

    // FIXME set_array

    pub fn get_string(&self, name: &str) -> &str {
        let cname = cstring(name);
        unsafe {
            string_ref(obs_sys::obs_data_get_string(
                self.as_raw().as_ptr(),
                cname.as_ptr(),
            ))
        }
    }

    pub fn get_int(&self, name: &str) -> i64 {
        let cname = cstring(name);
        unsafe { obs_sys::obs_data_get_int(self.as_raw().as_ptr(), cname.as_ptr()) as i64 }
    }

    pub fn get_double(&self, name: &str) -> f64 {
        let cname = cstring(name);
        unsafe { obs_sys::obs_data_get_double(self.as_raw().as_ptr(), cname.as_ptr()) }
    }

    pub fn get_bool(&self, name: &str) -> bool {
        let cname = cstring(name);
        unsafe { obs_sys::obs_data_get_bool(self.as_raw().as_ptr(), cname.as_ptr()) }
    }

    pub fn get_obj(&self, name: &str) -> Option<Box<Self>> {
        let cname = cstring(name);
        unsafe {
            Some(Self::from_raw(NonNull::new(obs_sys::obs_data_get_obj(
                self.as_raw().as_ptr(),
                cname.as_ptr(),
            ))?))
        }
    }

    // FIXME get_array

    pub fn set_default_string(&mut self, name: &str, val: &str) {
        let cname = cstring(name);
        let cval = cstring(val);
        unsafe {
            obs_sys::obs_data_set_default_string(
                self.as_raw().as_ptr(),
                cname.as_ptr(),
                cval.as_ptr(),
            );
        }
    }

    pub fn get_default_string(&self, name: &str) -> &str {
        let cname = cstring(name);
        unsafe {
            string_ref(obs_sys::obs_data_get_default_string(
                self.as_raw().as_ptr(),
                cname.as_ptr(),
            ))
        }
    }

    pub fn set_default_int(&mut self, name: &str, val: i64) {
        let cname = cstring(name);
        unsafe {
            obs_sys::obs_data_set_default_int(
                self.as_raw().as_ptr(),
                cname.as_ptr(),
                val as std::os::raw::c_longlong,
            );
        }
    }

    pub fn get_default_int(&self, name: &str) -> i64 {
        let cname = cstring(name);
        unsafe { obs_sys::obs_data_get_default_int(self.as_raw().as_ptr(), cname.as_ptr()) as i64 }
    }

    pub fn set_default_double(&mut self, name: &str, val: f64) {
        let cname = cstring(name);
        unsafe {
            obs_sys::obs_data_set_default_double(self.as_raw().as_ptr(), cname.as_ptr(), val);
        }
    }

    pub fn get_default_double(&self, name: &str) -> f64 {
        let cname = cstring(name);
        unsafe { obs_sys::obs_data_get_default_double(self.as_raw().as_ptr(), cname.as_ptr()) }
    }

    pub fn set_default_bool(&mut self, name: &str, val: bool) {
        let cname = cstring(name);
        unsafe {
            obs_sys::obs_data_set_default_bool(self.as_raw().as_ptr(), cname.as_ptr(), val);
        }
    }

    pub fn get_default_bool(&self, name: &str) -> bool {
        let cname = cstring(name);
        unsafe { obs_sys::obs_data_get_default_bool(self.as_raw().as_ptr(), cname.as_ptr()) }
    }

    pub fn set_default_obj(&mut self, name: &str, obj: &Data) {
        let cname = cstring(name);
        unsafe {
            obs_sys::obs_data_set_default_obj(
                self.as_raw().as_ptr(),
                cname.as_ptr(),
                obj.as_raw().as_ptr(),
            );
        }
    }

    pub fn get_default_obj(&self, name: &str) -> Option<Box<Self>> {
        let cname = cstring(name);
        unsafe {
            Some(Self::from_raw(NonNull::new(
                obs_sys::obs_data_get_default_obj(self.as_raw().as_ptr(), cname.as_ptr()),
            )?))
        }
    }

    // FIXME autoselect
}
