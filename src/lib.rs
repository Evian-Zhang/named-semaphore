mod error;
#[cfg(unix)]
mod unix;
#[cfg(windows)]
mod windows;

#[cfg(unix)]
use unix::RawNamedSemaphore;
#[cfg(windows)]
use windows::RawNamedSemaphore;

pub use error::Error;

type Result<T> = std::result::Result<T, Error>;

pub struct NamedSemaphore {
    raw_named_semaphore: RawNamedSemaphore,
}

impl NamedSemaphore {
    pub fn create<T: AsRef<str>>(name: T, initial_value: u32) -> Result<Self> {
        let raw_named_semaphore = RawNamedSemaphore::create(name, initial_value)?;

        Ok(Self {
            raw_named_semaphore,
        })
    }

    pub fn wait(&mut self) -> Result<()> {
        self.raw_named_semaphore.wait()
    }

    pub fn try_wait(&mut self) -> Result<()> {
        self.raw_named_semaphore.try_wait()
    }

    pub fn post(&mut self) -> Result<()> {
        self.raw_named_semaphore.post()
    }

    pub fn wait_then_post<T, F: FnOnce() -> T>(&mut self, action: F) -> Result<T> {
        self.wait()?;
        let result = action();
        self.post()?;

        Ok(result)
    }
}
