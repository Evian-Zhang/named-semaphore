mod error;
#[cfg(unix)]
mod unix;

#[cfg(unix)]
use unix::RawNamedSemaphore;

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
}
