#[cfg(windows)]
mod ffi {
    use std::os::windows::raw::HANDLE;
    type PHANDLE = *mut HANDLE;
    type HRESULT = libc::c_long;
    #[repr(C)]
    struct UnicodeString {
        length: u16,
        maximum_length: u16,
        buffer: *const u16,
    }
    #[repr(C)]
    struct SecurityDescriptor {
        revision: u8,
        sbz1: u8,
        control: u32,
        owner: *const libc::c_void,
        group: *const libc::c_void,
        sacl: *const libc::c_void,
        dacl: *const libc::c_void,
    }
    #[repr(C)]
    struct ObjectAttributes {
        length: u32,
        root_directory: HANDLE,
        object_name: *const UnicodeString,
        attributes: u32,
        // We always set this to NULL.
        security_descriptor: *const SecurityDescriptor,
        security_quality_of_service: *const libc::c_void,
    }

    #[repr(C)]
    struct IoStatusBlock {
        dummy_union_name: union {
            status: u32,
            pointer: *mut libc::c_void,
        },
        information: *const libc::c_void,
    }

    extern "system" {
        fn NtCreateFile(
            file_handle: &mut HANDLE,
            desired_access: u32,
            object_attributes: &ObjectAttributes,
            io_status_block: &mut IoStatusBlock,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
