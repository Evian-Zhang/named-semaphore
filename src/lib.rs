#![doc = include_str!("../README.md")]

mod error;
#[cfg(unix)]
mod unix;
#[cfg(windows)]
mod windows;

use std::time::Duration;

#[cfg(unix)]
use unix::RawNamedSemaphore;
#[cfg(windows)]
use windows::RawNamedSemaphore;

pub use error::Error;

type Result<T> = std::result::Result<T, Error>;

/// Named semaphore.
///
/// See lib level documentation for how to use this struct.
pub struct NamedSemaphore {
    raw_named_semaphore: RawNamedSemaphore,
}

impl NamedSemaphore {
    /// Create a named semaphore with name and initial value, or open it if there
    /// has already been a semaphore with the same name across system (in which case,
    /// the `name` and `initial_value` are ignored).
    ///
    /// In Linux, `name` should starts with "/" and is no longer than 250, and does not
    /// contain "/" after the prefix "/". `initial_value` should not greater than
    /// `SEM_VALUE_MAX`. The underlined implementation is
    /// [`sem_open`](https://www.man7.org/linux/man-pages/man3/sem_open.3.html).
    ///
    /// In Windows, `name` should be no longer than `MAX_PATH`. `initial_value` should
    /// fit in `i32` and not less than 0, the maximum count of the semaphore is set to the initial value.
    /// The underlying implementation is
    /// [`CreateSemaphore`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-createsemaphorea)
    ///
    /// # Notes
    ///
    /// The named semaphore will be closed when the `NamedSemaphore` drops.
    ///
    /// In Windows, if all accesses to named semaphore have been closed, the semaphore
    /// will be destroyed.
    ///
    /// In Linux, the named semaphore created will not be destroyed even if all processes accessing
    /// it has been terminated. The named semaphore can be destroyed by removing corresponding
    /// file in `/dev/shm`, or using [`sem_unlink`](https://www.man7.org/linux/man-pages/man3/sem_unlink.3.html),
    /// or restarting the operating system.
    pub fn create<T: AsRef<str>>(name: T, initial_value: u32) -> Result<Self> {
        let raw_named_semaphore = RawNamedSemaphore::create(name, initial_value)?;

        Ok(Self {
            raw_named_semaphore,
        })
    }

    /// Create a named semaphore with name and initial value, or open it if there
    /// has already been a semaphore with the same name across system (in which case,
    /// the `name` and `initial_value` are ignored).
    /// Max value is used on windows, where the semaphore can have an internal limit. It does not affect linux.
    ///
    /// In Linux, `name` should starts with "/" and is no longer than 250, and does not
    /// contain "/" after the prefix "/". `initial_value` should not greater than
    /// `SEM_VALUE_MAX`. The underlined implementation is
    /// [`sem_open`](https://www.man7.org/linux/man-pages/man3/sem_open.3.html).
    /// Max value does not have an effect on linux.
    ///
    /// In Windows, `name` should be no longer than `MAX_PATH`. `initial_value` and `max_value` should
    /// fit in `i32` and not less than 0. The underlying implementation is
    /// [`CreateSemaphore`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-createsemaphorea)
    ///
    /// # Notes
    ///
    /// The named semaphore will be closed when the `NamedSemaphore` drops.
    ///
    /// In Windows, if all accesses to named semaphore have been closed, the semaphore
    /// will be destroyed.
    ///
    /// In Linux, the named semaphore created will not be destroyed even if all processes accessing
    /// it has been terminated. The named semaphore can be destroyed by removing corresponding
    /// file in `/dev/shm`, or using [`sem_unlink`](https://www.man7.org/linux/man-pages/man3/sem_unlink.3.html),
    /// or restarting the operating system.
    pub fn create_with_max<T: AsRef<str>>(
        name: T,
        initial_value: u32,
        max_value: u32,
    ) -> Result<Self> {
        let raw_named_semaphore =
            RawNamedSemaphore::create_with_max(name, initial_value, max_value)?;

        Ok(Self {
            raw_named_semaphore,
        })
    }

    /// Wait for the semaphore and decrease it, block if current semaphore's count is 0.
    ///
    /// In Linux, the underlined implementation is [`sem_wait`](https://www.man7.org/linux/man-pages/man3/sem_wait.3.html).
    ///
    /// In Windows, the underlined implementation is
    /// [`WaitForSingleObject`](https://learn.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-waitforsingleobject).
    pub fn wait(&mut self) -> Result<()> {
        self.raw_named_semaphore.wait()
    }

    /// Wait for the semaphore and decrease it, block for `dur` if current semaphore's count is 0.
    ///
    /// In Linux, the underlined implementation is [`sem_timedwait`](https://www.man7.org/linux/man-pages/man3/sem_timedwait.3.html).
    ///
    /// In Windows, the underlined implementation is
    /// [`WaitForSingleObject`](https://learn.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-waitforsingleobject).
    ///
    /// Note that macOS did not provide such functionality, see <https://stackoverflow.com/q/641126/10005095>.
    pub fn timed_wait(&mut self, dur: Duration) -> Result<()> {
        self.raw_named_semaphore.timedwait(dur)
    }

    /// Wait for the semaphore and decrease it, raise [`Error::WouldBlock`] if current
    /// semaphore's count is 0.
    ///
    /// In Linux, the underlined implementation is [`sem_trywait`](https://www.man7.org/linux/man-pages/man3/sem_wait.3.html).
    ///
    /// In Windows, the underlined implementation is
    /// [`WaitForSingleObject`](https://learn.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-waitforsingleobject).
    pub fn try_wait(&mut self) -> Result<()> {
        self.raw_named_semaphore.try_wait()
    }

    /// Release the semaphore and increase it.
    ///
    /// In Linux, the underlined implementation is [`sem_post`](https://www.man7.org/linux/man-pages/man3/sem_post.3.html).
    ///
    /// In Windows, the underlined implementation is
    /// [`ReleaseSemaphore`](https://learn.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-releasesemaphore).
    pub fn post(&mut self) -> Result<()> {
        self.raw_named_semaphore.post()
    }

    /// A convenient method to wait-then-post the semaphore.
    pub fn wait_then_post<T, F: FnOnce() -> T>(&mut self, action: F) -> Result<T> {
        self.wait()?;
        let result = action();
        self.post()?;

        Ok(result)
    }
}
