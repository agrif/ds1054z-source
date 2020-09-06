use crate::string::cstring;
use std::ptr::NonNull;

#[derive(Debug)]
pub struct Properties(NonNull<obs_sys::obs_properties_t>);

impl Properties {
    pub unsafe fn from_raw(raw: NonNull<obs_sys::obs_properties_t>) -> Self {
        Properties(raw)
    }

    pub unsafe fn into_raw(self) -> NonNull<obs_sys::obs_properties_t> {
        let raw = self.0;
        std::mem::forget(self);
        raw
    }

    pub fn create() -> Self {
        Properties(unsafe {
            NonNull::new(obs_sys::obs_properties_create()).expect("pointer is null")
        })
    }

    pub fn add_bool(&mut self, name: &str, description: &str) -> Property {
        let cname = cstring(name);
        let cdesc = cstring(description);
        unsafe {
            Property::from_raw(
                NonNull::new(obs_sys::obs_properties_add_bool(
                    self.0.as_ptr(),
                    cname.as_ptr(),
                    cdesc.as_ptr(),
                ))
                .expect("pointer is null"),
            )
        }
    }

    pub fn add_text(&mut self, name: &str, description: &str, type_: TextType) -> Property {
        let cname = cstring(name);
        let cdesc = cstring(description);
        unsafe {
            Property::from_raw(
                NonNull::new(obs_sys::obs_properties_add_text(
                    self.0.as_ptr(),
                    cname.as_ptr(),
                    cdesc.as_ptr(),
                    type_.into_raw(),
                ))
                .expect("pointer is null"),
            )
        }
    }
}

impl Drop for Properties {
    fn drop(&mut self) {
        unsafe {
            obs_sys::obs_properties_destroy(self.0.as_ptr());
        }
    }
}

#[derive(Debug)]
pub struct Property<'a>(&'a mut obs_sys::obs_property_t);

impl<'a> Property<'a> {
    pub unsafe fn from_raw(raw: NonNull<obs_sys::obs_property_t>) -> Self {
        Property(&mut *raw.as_ptr())
    }

    pub unsafe fn into_raw(self) -> NonNull<obs_sys::obs_property_t> {
        let raw = self.0.into();
        std::mem::forget(self);
        raw
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum TextType {
    Default,
    Multiline,
    Password,
}

impl TextType {
    pub fn into_raw(self) -> obs_sys::obs_text_type {
        match self {
            TextType::Default => obs_sys::obs_text_type_OBS_TEXT_DEFAULT,
            TextType::Multiline => obs_sys::obs_text_type_OBS_TEXT_MULTILINE,
            TextType::Password => obs_sys::obs_text_type_OBS_TEXT_PASSWORD,
        }
    }

    pub fn from_raw(raw: obs_sys::obs_text_type) -> Option<Self> {
        match raw {
            obs_sys::obs_text_type_OBS_TEXT_DEFAULT => Some(TextType::Default),
            obs_sys::obs_text_type_OBS_TEXT_MULTILINE => Some(TextType::Multiline),
            obs_sys::obs_text_type_OBS_TEXT_PASSWORD => Some(TextType::Password),
            _ => None,
        }
    }
}
