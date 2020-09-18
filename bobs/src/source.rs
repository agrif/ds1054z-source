use crate::{ObsRawBox, ObsRawCounted, ObsRawWeak};
use std::ptr::NonNull;

#[derive(Debug)]
#[repr(C)]
pub struct Source(obs_sys::obs_source_t);

impl ObsRawBox for Source {
    type Raw = NonNull<obs_sys::obs_source_t>;

    unsafe fn from_raw(raw: Self::Raw) -> Box<Self> {
        Box::from_raw(std::mem::transmute(raw.as_ptr()))
    }

    unsafe fn as_raw(&self) -> Self::Raw {
        (&self.0).into()
    }
}

impl ObsRawCounted for Source {
    unsafe fn addref(&self) {
        obs_sys::obs_source_addref(self.as_raw().as_ptr());
    }

    unsafe fn release(&self) {
        obs_sys::obs_source_release(self.as_raw().as_ptr());
    }
}

impl ObsRawWeak for Source {
    type Weak = NonNull<obs_sys::obs_weak_source_t>;

    unsafe fn get_raw_weak(&self) -> Self::Weak {
        NonNull::new(obs_sys::obs_source_get_weak_source(self.as_raw().as_ptr()))
            .expect("null pointer")
    }

    unsafe fn get_raw_strong(weak: Self::Weak) -> Option<Box<Self>> {
        Some(Self::from_raw(NonNull::new(
            obs_sys::obs_weak_source_get_source(weak.as_ptr()),
        )?))
    }

    unsafe fn weak_addref(weak: Self::Weak) {
        obs_sys::obs_weak_source_addref(weak.as_ptr());
    }

    unsafe fn weak_release(weak: Self::Weak) {
        obs_sys::obs_weak_source_release(weak.as_ptr());
    }
}

impl Drop for Source {
    fn drop(&mut self) {
        unsafe { self.release() }
    }
}
