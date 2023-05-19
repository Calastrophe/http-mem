use libc::{iovec, process_vm_readv, process_vm_writev};
use log::error;
use std::ffi::c_void;

// Linux implementation to read host OS memory.

pub fn reader(pid: i32, addr: *mut c_void, type_size: usize) -> Result<Vec<u8>, ()> {
    let mut bytes_read: Vec<u8> = Vec::with_capacity(type_size);
    let local_iov = iovec {
        iov_base: bytes_read.as_mut_ptr().cast(),
        iov_len: type_size,
    };
    let remote_iov = iovec {
        iov_base: addr,
        iov_len: type_size,
    };

    let result = unsafe {
        bytes_read.set_len(type_size);
        process_vm_readv(pid, &local_iov as _, 1, &remote_iov as _, 1, 0)
    };

    match result {
        -1 => {
            error!(
                "The error thrown last by the host OS was {:?}",
                std::io::Error::last_os_error()
            );
            Err(())
        }
        _ => Ok(bytes_read),
    }
}

pub fn writer(pid: i32, addr: *mut c_void, value: &mut Vec<u8>) -> Result<isize, ()> {
    let local_iov = iovec {
        iov_base: value.as_mut_ptr().cast(),
        iov_len: value.len(),
    };
    let remote_iov = iovec {
        iov_base: addr,
        iov_len: value.len(),
    };

    let result = unsafe { process_vm_writev(pid, &local_iov as _, 1, &remote_iov as _, 1, 0) };

    match result {
        -1 => {
            error!(
                "The error thrown last by the host OS was {:?}",
                std::io::Error::last_os_error()
            );
            Err(())
        }
        _ => Ok(result),
    }
}

// TODO: Windows implmentation that uses DBVM?
