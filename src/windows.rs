use std::time::Duration;

use windows::{
    core::{HSTRING, PCWSTR},
    Win32::{
        Foundation::{
            CloseHandle, GetLastError, HANDLE, WAIT_ABANDONED, WAIT_FAILED, WAIT_OBJECT_0,
            WAIT_TIMEOUT,
        },
        System::Threading::{CreateSemaphoreW, ReleaseSemaphore, WaitForSingleObject, INFINITE},
    },
};

use super::{Error, Result};

pub(crate) struct RawNamedSemaphore {
    handle: HANDLE,
}

impl RawNamedSemaphore {
    // NOTE:
    //
    // `initial_value` should not exceed `i32::MAX`
    pub(crate) fn create<T: AsRef<str>>(name: T, initial_value: u32) -> Result<Self> {
        let name = name.as_ref();
        let name = HSTRING::from(name);
        let name = PCWSTR(name.as_ptr());
        let Ok(initial_value) = i32::try_from(initial_value) else {
            return Err(Error::InappropriateCount);
        };
        let handle = unsafe { CreateSemaphoreW(None, initial_value, initial_value, name) }
            .map_err(|error| Error::CreateFailed(error))?;

        Ok(Self { handle })
    }

    pub(crate) fn wait(&mut self) -> Result<()> {
        let wait_event = unsafe { WaitForSingleObject(self.handle, INFINITE) };
        if wait_event == WAIT_FAILED {
            if let Err(last_error) = unsafe { GetLastError() } {
                return Err(Error::WaitFailed(last_error));
            } else {
                return Err(Error::Unexpected);
            }
        }

        Ok(())
    }

    pub(crate) fn timedwait(&mut self, dur: Duration) -> Result<()> {
        let Ok(wait_timeout) = u32::try_from(dur.as_millis()) else {
            return Err(Error::InvalidWaitTimeout);
        };
        let wait_event = unsafe { WaitForSingleObject(self.handle, wait_timeout) };
        match wait_event {
            WAIT_OBJECT_0 | WAIT_ABANDONED => Ok(()),
            WAIT_FAILED => {
                if let Err(last_error) = unsafe { GetLastError() } {
                    Err(Error::WaitFailed(last_error))
                } else {
                    Err(Error::Unexpected)
                }
            }
            WAIT_TIMEOUT => Err(Error::WaitTimeout),
            _ => Err(Error::Unexpected),
        }
    }

    pub(crate) fn try_wait(&mut self) -> Result<()> {
        let wait_event = unsafe { WaitForSingleObject(self.handle, 0) };
        match wait_event {
            WAIT_OBJECT_0 | WAIT_ABANDONED => Ok(()),
            WAIT_FAILED => {
                if let Err(last_error) = unsafe { GetLastError() } {
                    Err(Error::WaitFailed(last_error))
                } else {
                    Err(Error::Unexpected)
                }
            }
            WAIT_TIMEOUT => Err(Error::WouldBlock),
            _ => Err(Error::Unexpected),
        }
    }

    pub(crate) fn post(&mut self) -> Result<()> {
        unsafe { ReleaseSemaphore(self.handle, 1, None) }.map_err(|error| Error::PostFailed(error))
    }
}

impl Drop for RawNamedSemaphore {
    // From the REMARKS section:
    //
    // > Use the CloseHandle function to close the handle. The system closes the handle automatically when the process terminates. The semaphore object is destroyed when its last handle has been closed.
    //
    // We don't care if this failed.
    fn drop(&mut self) {
        let _ = unsafe { CloseHandle(self.handle) };
    }
}
