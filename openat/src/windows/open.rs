use super::ffi::{_UNICODE_STRING, OBJECT_ATTRIBUTES, HANDLE};
fn vec_to_unicode_string(buf: &[u16]) -> _UNICODE_STRING {
    _UNICODE_STRING {
        Length: buf.len(),
        MaximumLength: buf.len(),
        Buffer: buf.as_ptr(),
    }
}
macro_rules! assert_same_size {
    (t1:ty, t2:ty) => {
        if false {
            unsafe {
            ::std::mem::forget::<$t2>(::std::mem::transmute::<$t1, $t2>(::std::mem::uninitialized::<$t1>()))
            }
        }
    }
}

// fn create_object_attributes(buf: &[u16], hnd: Option<NonNull<HANDLE>>) -> OBJECT_ATTRIBUTES {
 //   if 

