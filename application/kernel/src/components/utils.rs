#[cfg(feature = "tcp_data")]
use std::io::Read;
#[cfg(feature = "tcp_data")]
use std::net::TcpListener;
#[cfg(feature = "tcp_data")]
use std::str::FromStr;
#[cfg(feature = "tcp_data")]
use std::thread::spawn;
#[cfg(feature = "tcp_data")]
use std::thread::JoinHandle;

#[cfg(feature = "tcp_data")]
use anyhow::Error;
#[cfg(feature = "tcp_data")]
use anyhow::Result;

#[cfg(feature = "tcp_data")]
use crate::components::TCP_BUFFER_CAPACITY;
#[cfg(feature = "tcp_data")]
use crate::wait::Wait;

#[cfg(feature = "tcp_data")]
pub fn run_tcp<S, F, T>(
    host: S,
    port: u16,
    mut parser: F,
    pause: Option<Wait<bool>>,
) -> Result<JoinHandle<()>>
where
    S: AsRef<str>,
    F: 'static + FnMut(T) -> Result<()> + Send,
    T: FromStr,
    Error: From<T::Err>,
{
    let host = host.as_ref();
    let addr = format!("{}:{}", host, port);
    let listener = TcpListener::bind(addr)?;
    let handle = spawn(move || {
        listener
            .incoming()
            .filter_map(|stream| {
                let mut stream = stream.ok()?;
                let mut buffer = [0u8; TCP_BUFFER_CAPACITY];
                loop {
                    let bytes_read = stream.read(&mut buffer).unwrap();
                    let did_wait = pause
                        .clone()
                        .and_then(|pause| pause.wait(false).ok())
                        .unwrap_or_default();
                    match (did_wait, bytes_read) {
                        (true, _) => continue,
                        (_, 0) => break None,
                        _ if bytes_read == TCP_BUFFER_CAPACITY => break None,
                        _ => {
                            let last = buffer[bytes_read - 1] as char;
                            let buffer = &buffer[0..(match last {
                                '\n' => bytes_read - 1,
                                _ => bytes_read,
                            })];
                            String::from_utf8_lossy(buffer)
                                .parse::<T>()
                                .ok()
                                .and_then(|data| parser(data).ok());
                        },
                    }
                }
            })
            .for_each(|()| ());
    });
    Ok(handle)
}
