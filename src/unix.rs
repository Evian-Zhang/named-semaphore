use std::ffi::CString;

use libc::{c_char, sem_t, O_CREAT, SEM_FAILED, S_IRWXG, S_IRWXO, S_IRWXU};

use crate::Error;

use super::Result;

pub(crate) struct RawNamedSemaphore {
    raw_ptr: *mut sem_t,
}

impl RawNamedSemaphore {
    // NOTE:
    //
    // The `name` should starts with "/"
    pub(crate) fn create<T: AsRef<str>>(name: T, initial_value: u32) -> Result<Self> {
        let name = name.as_ref();
        let name = CString::new(name.as_bytes()).map_err(|_| Error::InvalidCharacter)?;
        let raw_ptr = unsafe {
            libc::sem_open(
                name.as_ptr() as *const c_char,
                O_CREAT,
                (S_IRWXU | S_IRWXG | S_IRWXO) as libc::c_uint,
                initial_value,
            )
        };

        if raw_ptr == SEM_FAILED {
            return Err(Error::CreateFailed(std::io::Error::last_os_error()));
        }

        Ok(Self { raw_ptr })
    }

    pub(crate) fn wait(&mut self) -> Result<()> {
        if unsafe { libc::sem_wait(self.raw_ptr) } == -1 {
            return Err(Error::WaitFailed(std::io::Error::last_os_error()));
        }
        Ok(())
    }

    pub(crate) fn try_wait(&mut self) -> Result<()> {
        if unsafe { libc::sem_trywait(self.raw_ptr) } == -1 {
            let last_error = std::io::Error::last_os_error();
            let error = if last_error.kind() == std::io::ErrorKind::WouldBlock {
                Error::WouldBlock
            } else {
                Error::WaitFailed(last_error)
            };
            return Err(error);
        }
        Ok(())
    }

    pub(crate) fn post(&mut self) -> Result<()> {
        if unsafe { libc::sem_post(self.raw_ptr) } == -1 {
            return Err(Error::PostFailed(std::io::Error::last_os_error()));
        }
        Ok(())
    }
}

impl Drop for RawNamedSemaphore {
    fn drop(&mut self) {
        // From the NOTES section:
        //
        // > All open named semaphores are automatically closed on process termination, or upon execve(2).
        //
        // We don't care if this failed.
        unsafe {
            libc::sem_close(self.raw_ptr);
        }
    }
}
