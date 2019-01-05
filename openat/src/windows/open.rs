extern crate winapi;
use winapi::um::securitybaseapi::GetTokenInformation;
use winapi::um::minwinbase::{
    LPVOID,
    PDWORD,
    DWORD,
    BOOL,
    BYTE,
};
use winapi::um::winnt::{
    TOKEN_INFORMATION_CLASS,
    HANDLE,
    LPVOID,
};
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

#[derive(Clone, Hash, Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Sid {
    storage: Vec<u32>,
}

#[derive(Clone, Hash, Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Acl {
    storage: Vec<u32>,
}

fn create_safe_acl(user_sid: Sid, group_sid: Sid, permissions: u16) {
    assert_eq!(permissions & ~0o777u16, 0, "Windows does not support SUID, SGID, or sticky bit");
    let mut acl_size = std::mem::size_of::<ACL>();
    for i in 0..=2 {
        if permissions & 7u16 << i {
            acl_size += std::mem::size_of::<ACCESS_ALLOWED_ACE> - 4;
        }
    }
    if permissions & 7 << 6 != 0 {
        acl_size += user.size_in_bytes()
    }
    if permissions & 7 << 3 != 0 {
        acl_size += group.size_in_bytes()
    }
    let mut res = Vec<u32>::with_capacity(acl_size / 4);
    unsafe {
        InitializeAcl(res.ptr_mut() as *mut _, acl_size, ACL_REVISION);
        if permissions & 7 << 6 != 0 {
            assert_ne!(AddAccessAllowedAce(res.as_mut_ptr() as *mut _, ACL_REVISION,
                                unix_perms_to_windows_access_mask(permissions & 7)
                                user.storage.as_ptr() as *const _), 0)
        }
        if permissions & 7 << 3 != 0 {
            assert_ne!(AddAccessAllowedAce(res.as_mut_ptr() as *mut _, ACL_REVISION,
                                unix_perms_to_windows_access_mask(permissions & 7)
                                group.storage.as_ptr() as *const _), 0)

        }

    }
impl Sid {
    fn new(buf: &[u32]) -> SID {




fn get_current_user() -> 
