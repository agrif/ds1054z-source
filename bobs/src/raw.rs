pub trait ObsRawBox {
    type Raw;

    // from_raw *transfers* ownership
    unsafe fn from_raw(raw: Self::Raw) -> Box<Self>;
    unsafe fn as_raw(&self) -> Self::Raw;

    // into_raw *transfers* ownership
    unsafe fn into_raw(self: Box<Self>) -> Self::Raw {
        let raw = self.as_raw();
        std::mem::forget(self);
        raw
    }
}

pub trait ObsRawCounted: ObsRawBox {
    unsafe fn addref(&self);
    unsafe fn release(&self);

    unsafe fn clone(&self) -> Box<Self> {
        self.addref();
        Self::from_raw(self.as_raw())
    }

    unsafe fn from_raw_owned(raw: Self::Raw) -> Box<Self> {
        Self::from_raw(raw)
    }

    unsafe fn from_raw_unowned(raw: Self::Raw) -> Box<Self> {
        let s = Self::from_raw(raw);
        s.addref();
        s
    }
}

pub trait ObsRawWeak: ObsRawCounted {
    type Weak: Clone;
    unsafe fn get_raw_weak(&self) -> Self::Weak;
    unsafe fn get_raw_strong(weak: Self::Weak) -> Option<Box<Self>>;
    unsafe fn weak_addref(weak: Self::Weak);
    unsafe fn weak_release(weak: Self::Weak);

    fn downgrade(&self) -> Weak<Self> {
        Weak(unsafe { self.get_raw_weak() })
    }
}

#[derive(Debug)]
pub struct Weak<T: ObsRawWeak + ?Sized>(T::Weak);

impl<T> Weak<T>
where
    T: ObsRawWeak,
{
    pub unsafe fn upgrade(&self) -> Option<Box<T>> {
        let weak = self.0.clone();
        <T as ObsRawWeak>::get_raw_strong(weak)
    }
}

impl<T> Clone for Weak<T>
where
    T: ObsRawWeak,
{
    fn clone(&self) -> Self {
        unsafe {
            <T as ObsRawWeak>::weak_addref(self.0.clone());
        }
        Weak(self.0.clone())
    }
}

impl<T: ?Sized> Drop for Weak<T>
where
    T: ObsRawWeak,
{
    fn drop(&mut self) {
        unsafe {
            <T as ObsRawWeak>::weak_release(self.0.clone());
        }
    }
}
