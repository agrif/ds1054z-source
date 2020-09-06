use crate::string::{cstring, string_ref};
use std::ptr::NonNull;

#[derive(Debug)]
pub struct Data(NonNull<obs_sys::obs_data_t>);

impl Data {
    pub unsafe fn from_raw_owned(raw: NonNull<obs_sys::obs_data_t>) -> Self {
        Data(raw)
    }

    pub unsafe fn from_raw_unowned(raw: NonNull<obs_sys::obs_data_t>) -> Self {
        obs_sys::obs_data_addref(raw.as_ptr());
        Data(raw)
    }

    pub unsafe fn as_raw(&mut self) -> NonNull<obs_sys::obs_data_t> {
        self.0
    }

    pub unsafe fn into_raw(self) -> NonNull<obs_sys::obs_data_t> {
        let raw = self.0;
        std::mem::forget(self);
        raw
    }

    pub fn create() -> Self {
        Self(unsafe { NonNull::new(obs_sys::obs_data_create()).expect("pointer is null") })
    }

    pub fn create_from_json(json_string: &str) -> Option<Self> {
        let cjson = cstring(json_string);
        unsafe {
            Some(Self(NonNull::new(obs_sys::obs_data_create_from_json(
                cjson.as_ptr(),
            ))?))
        }
    }

    pub fn create_from_json_file<P>(json_file: P) -> Option<Self>
    where
        P: AsRef<std::path::Path>,
    {
        let cpath = cstring(json_file.as_ref().to_str().expect("bad path"));
        unsafe {
            Some(Self(NonNull::new(
                obs_sys::obs_data_create_from_json_file(cpath.as_ptr()),
            )?))
        }
    }

    pub fn create_from_json_file_safe<P>(json_file: P, backup_ext: &str) -> Option<Self>
    where
        P: AsRef<std::path::Path>,
    {
        let cpath = cstring(json_file.as_ref().to_str().expect("bad path"));
        let cext = cstring(backup_ext);
        unsafe {
            Some(Self(NonNull::new(
                obs_sys::obs_data_create_from_json_file_safe(cpath.as_ptr(), cext.as_ptr()),
            )?))
        }
    }

    pub fn get_json(&mut self) -> &str {
        // mut self because this re-uses an internal buffer
        unsafe { string_ref(obs_sys::obs_data_get_json(self.0.as_ptr())) }
    }

    pub fn save_json<P>(&self, file: P)
    where
        P: AsRef<std::path::Path>,
    {
        let cpath = cstring(file.as_ref().to_str().expect("bad path"));
        unsafe {
            obs_sys::obs_data_save_json(self.0.as_ptr(), cpath.as_ptr());
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
                self.0.as_ptr(),
                cpath.as_ptr(),
                ctext.as_ptr(),
                cbext.as_ptr(),
            );
        }
    }

    pub fn apply(&mut self, apply_data: &Data) {
        unsafe {
            obs_sys::obs_data_apply(self.0.as_ptr(), apply_data.0.as_ptr());
        }
    }

    pub fn erase(&mut self, name: &str) {
        let cname = cstring(name);
        unsafe {
            obs_sys::obs_data_erase(self.0.as_ptr(), cname.as_ptr());
        }
    }

    pub fn clear(&mut self) {
        unsafe {
            obs_sys::obs_data_clear(self.0.as_ptr());
        }
    }

    pub fn set_string(&mut self, name: &str, val: &str) {
        let cname = cstring(name);
        let cval = cstring(val);
        unsafe {
            obs_sys::obs_data_set_string(self.0.as_ptr(), cname.as_ptr(), cval.as_ptr());
        }
    }

    pub fn set_int(&mut self, name: &str, val: i64) {
        let cname = cstring(name);
        unsafe {
            obs_sys::obs_data_set_int(
                self.0.as_ptr(),
                cname.as_ptr(),
                val as std::os::raw::c_longlong,
            );
        }
    }

    pub fn set_double(&mut self, name: &str, val: f64) {
        let cname = cstring(name);
        unsafe {
            obs_sys::obs_data_set_double(self.0.as_ptr(), cname.as_ptr(), val);
        }
    }

    pub fn set_bool(&mut self, name: &str, val: bool) {
        let cname = cstring(name);
        unsafe {
            obs_sys::obs_data_set_bool(self.0.as_ptr(), cname.as_ptr(), val);
        }
    }

    pub fn set_obj(&mut self, name: &str, val: &Data) {
        let cname = cstring(name);
        unsafe {
            obs_sys::obs_data_set_obj(self.0.as_ptr(), cname.as_ptr(), val.0.as_ptr());
        }
    }

    // FIXME set_array

    pub fn get_string(&self, name: &str) -> &str {
        let cname = cstring(name);
        unsafe {
            string_ref(obs_sys::obs_data_get_string(
                self.0.as_ptr(),
                cname.as_ptr(),
            ))
        }
    }

    pub fn get_int(&self, name: &str) -> i64 {
        let cname = cstring(name);
        unsafe { obs_sys::obs_data_get_int(self.0.as_ptr(), cname.as_ptr()) as i64 }
    }

    pub fn get_double(&self, name: &str) -> f64 {
        let cname = cstring(name);
        unsafe { obs_sys::obs_data_get_double(self.0.as_ptr(), cname.as_ptr()) }
    }

    pub fn get_bool(&self, name: &str) -> bool {
        let cname = cstring(name);
        unsafe { obs_sys::obs_data_get_bool(self.0.as_ptr(), cname.as_ptr()) }
    }

    pub fn get_obj(&self, name: &str) -> Option<Data> {
        let cname = cstring(name);
        unsafe {
            Some(Self(NonNull::new(obs_sys::obs_data_get_obj(
                self.0.as_ptr(),
                cname.as_ptr(),
            ))?))
        }
    }

    // FIXME get_array

    pub fn set_default_string(&mut self, name: &str, val: &str) {
        let cname = cstring(name);
        let cval = cstring(val);
        unsafe {
            obs_sys::obs_data_set_default_string(self.0.as_ptr(), cname.as_ptr(), cval.as_ptr());
        }
    }

    pub fn get_default_string(&self, name: &str) -> &str {
        let cname = cstring(name);
        unsafe {
            string_ref(obs_sys::obs_data_get_default_string(
                self.0.as_ptr(),
                cname.as_ptr(),
            ))
        }
    }

    pub fn set_default_int(&mut self, name: &str, val: i64) {
        let cname = cstring(name);
        unsafe {
            obs_sys::obs_data_set_default_int(
                self.0.as_ptr(),
                cname.as_ptr(),
                val as std::os::raw::c_longlong,
            );
        }
    }

    pub fn get_default_int(&self, name: &str) -> i64 {
        let cname = cstring(name);
        unsafe { obs_sys::obs_data_get_default_int(self.0.as_ptr(), cname.as_ptr()) as i64 }
    }

    pub fn set_default_double(&mut self, name: &str, val: f64) {
        let cname = cstring(name);
        unsafe {
            obs_sys::obs_data_set_default_double(self.0.as_ptr(), cname.as_ptr(), val);
        }
    }

    pub fn get_default_double(&self, name: &str) -> f64 {
        let cname = cstring(name);
        unsafe { obs_sys::obs_data_get_default_double(self.0.as_ptr(), cname.as_ptr()) }
    }

    pub fn set_default_bool(&mut self, name: &str, val: bool) {
        let cname = cstring(name);
        unsafe {
            obs_sys::obs_data_set_default_bool(self.0.as_ptr(), cname.as_ptr(), val);
        }
    }

    pub fn get_default_bool(&self, name: &str) -> bool {
        let cname = cstring(name);
        unsafe { obs_sys::obs_data_get_default_bool(self.0.as_ptr(), cname.as_ptr()) }
    }

    pub fn set_default_obj(&mut self, name: &str, obj: &Data) {
        let cname = cstring(name);
        unsafe {
            obs_sys::obs_data_set_default_obj(self.0.as_ptr(), cname.as_ptr(), obj.0.as_ptr());
        }
    }

    pub fn get_default_obj(&self, name: &str) -> Option<Data> {
        let cname = cstring(name);
        unsafe {
            Some(Self(NonNull::new(obs_sys::obs_data_get_default_obj(
                self.0.as_ptr(),
                cname.as_ptr(),
            ))?))
        }
    }

    // FIXME autoselect
}

impl Drop for Data {
    fn drop(&mut self) {
        unsafe {
            obs_sys::obs_data_release(self.0.as_ptr());
        }
    }
}

impl Clone for Data {
    fn clone(&self) -> Self {
        unsafe { Self::from_raw_unowned(self.0) }
    }
}
