use libc::{mount, umount, umount2, EINVAL, MS_BIND, MS_REC};
use std::ffi::CString;
use std::os::raw::c_char;

pub fn mount_bind(src_path: &str, dest_path: &str) -> Result<(), String> {
    let src_path = CString::new(src_path).expect("CString::new failed");
    let dest_path = CString::new(dest_path).expect("CString::new failed");

    // 检查目录是否已经被挂载，如果是的，先卸载
    let _ = unsafe { umount2(dest_path.as_ptr(), libc::MNT_DETACH) };

    // 挂载文件系统
    let result = unsafe {
        mount(
            src_path.as_ptr() as *const c_char,
            dest_path.as_ptr() as *const c_char,
            std::ptr::null(),
            MS_BIND | MS_REC,
            std::ptr::null(),
        )
    };

    if result != 0 {
        if result == -EINVAL {
            return Err(String::from("Invalid arguments provided."));
        } else {
            return Err(String::from("Failed to mount filesystem."));
        }
    }

    Ok(())
}

pub fn unmount(file_system: &str) {
    let path = CString::new(file_system).unwrap();
    let _result = unsafe { umount(path.as_ptr()) };
}
