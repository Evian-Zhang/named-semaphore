# Named Semaphore

Named semaphore for Linux & Windows.

To use this crate, add the following to `Cargo.toml`:

```toml
[dependencies]
named-sem = "0.2"
```

## Background

Named semaphore is a process synchronize mechanism provided by OS and libc in Linux & Windows. By giving the semaphore a name, processes with appropriate permissions can share the same semaphore.

In Linux, we use the [POSIX semaphore](https://www.man7.org/linux/man-pages/man7/sem_overview.7.html) as implementation, and in Windows, we use [Semaphore Objects](https://learn.microsoft.com/en-us/windows/win32/sync/using-semaphore-objects).

## Examples

```rust
use named_sem::{NamedSemaphore, Error};

# fn do_heavy_things() {}
fn use_named_semaphore() -> Result<(), Error> {
    // In Linux, the semaphore's name should begin with "/"
    let mut semaphore = NamedSemaphore::create("/my-semaphore", 3)?;

    semaphore.wait_then_post(|| {
        do_heavy_things();
    })?;

    Ok(())
}
```

## Usage

A common usage for named semaphore is to control the process count across the system.

For example, we have four large directories, `A`, `B`, `C` and `D`, which needs to be compressed. While our computer's hardware only allows for two `7z` processes to run at the same time. So we can create a named semaphore with initial value 2, and require the semaphore before each `7z` run.

There is a small utility, `preempt-do`, in this repo. Compile it by

```shell
cargo build --release --features=commandline --bin preempt-do
```

Or you can install it by

```shell
cargo install named-sem --features=commandline --bin preempt-do
```

Then you can use the following instructions to do the things above:

```shell
preempt-do --name /my-semaphore --count 2 -- 7z a A.7z A
preempt-do --name /my-semaphore --count 2 -- 7z a B.7z B
preempt-do --name /my-semaphore --count 2 -- 7z a C.7z C
preempt-do --name /my-semaphore --count 2 -- 7z a D.7z D
```

This will allow there to be no more than two `7z` processes across the system.
