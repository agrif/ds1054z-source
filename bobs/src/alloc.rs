use std::convert::TryInto;

#[derive(Debug)]
pub struct ObsAllocator;

#[global_allocator]
static GLOBAL: ObsAllocator = ObsAllocator;

unsafe fn alloc_helper<F, P>(layout: std::alloc::Layout, alloc: F) -> *mut u8
where
    F: FnOnce(usize) -> *mut P,
{
    let p = alloc(layout.size() + layout.align()) as *mut u8;
    if p.is_null() {
        return p;
    }

    // shift to alignment
    let diff = ((!(p as usize)) & (layout.align() - 1)) + 1;
    let diff8: u8 = diff.try_into().expect("alignment too large");
    let p = p.offset(diff as isize);
    *p.offset(-1) = diff8;
    p
}

unsafe fn ptr_base<P>(p: *mut u8) -> *mut P {
    let diff = *p.offset(-1) as isize;
    p.offset(-diff) as *mut P
}

unsafe impl std::alloc::GlobalAlloc for ObsAllocator {
    unsafe fn alloc(&self, layout: std::alloc::Layout) -> *mut u8 {
        alloc_helper(layout, |s| obs_sys::bmalloc(s as obs_sys::size_t))
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: std::alloc::Layout) {
        obs_sys::bfree(ptr_base(ptr))
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: std::alloc::Layout, new_size: usize) -> *mut u8 {
        let newlayout = std::alloc::Layout::from_size_align_unchecked(new_size, layout.align());
        alloc_helper(newlayout, |s| {
            obs_sys::brealloc(ptr_base(ptr), s as obs_sys::size_t)
        })
    }
}
