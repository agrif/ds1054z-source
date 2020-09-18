use crate::string::cstring;
use crate::ObsRawBox;
use std::ptr::NonNull;

#[derive(Debug)]
#[repr(C)]
pub struct Properties(obs_sys::obs_properties_t);

impl ObsRawBox for Properties {
    type Raw = NonNull<obs_sys::obs_properties_t>;

    unsafe fn from_raw(raw: Self::Raw) -> Box<Properties> {
        Box::from_raw(std::mem::transmute(raw.as_ptr()))
    }

    unsafe fn as_raw(&self) -> Self::Raw {
        (&self.0).into()
    }
}

impl Drop for Properties {
    fn drop(&mut self) {
        unsafe {
            obs_sys::obs_properties_destroy(self.as_raw().as_ptr());
        }
    }
}

impl Properties {
    pub fn create() -> Box<Self> {
        unsafe {
            Self::from_raw(NonNull::new(obs_sys::obs_properties_create()).expect("pointer is null"))
        }
    }

    pub fn add_bool(&mut self, name: &str, description: &str) -> Box<Property> {
        let cname = cstring(name);
        let cdesc = cstring(description);
        unsafe {
            Property::from_raw(
                NonNull::new(obs_sys::obs_properties_add_bool(
                    self.as_raw().as_ptr(),
                    cname.as_ptr(),
                    cdesc.as_ptr(),
                ))
                .expect("pointer is null"),
            )
        }
    }

    pub fn add_text(&mut self, name: &str, description: &str, type_: TextType) -> Box<Property> {
        let cname = cstring(name);
        let cdesc = cstring(description);
        unsafe {
            Property::from_raw(
                NonNull::new(obs_sys::obs_properties_add_text(
                    self.as_raw().as_ptr(),
                    cname.as_ptr(),
                    cdesc.as_ptr(),
                    type_.into_raw(),
                ))
                .expect("pointer is null"),
            )
        }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct Property(obs_sys::obs_property_t);

impl ObsRawBox for Property {
    type Raw = NonNull<obs_sys::obs_property_t>;

    unsafe fn from_raw(raw: Self::Raw) -> Box<Property> {
        Box::from_raw(std::mem::transmute(raw.as_ptr()))
    }

    unsafe fn as_raw(&self) -> Self::Raw {
        (&self.0).into()
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
