#[cfg(feature = "simulation")]
use std::io::Read;
#[cfg(feature = "simulation")]
use std::net::TcpListener;
#[cfg(feature = "simulation")]
use std::net::TcpStream;
#[cfg(feature = "simulation")]
use std::str::FromStr;
#[cfg(feature = "simulation")]
use std::thread::spawn;
#[cfg(feature = "simulation")]
use std::thread::JoinHandle;

#[cfg(feature = "simulation")]
use anyhow::Error;
#[cfg(feature = "simulation")]
use anyhow::Result;

#[cfg(feature = "simulation")]
use crate::components::TCP_BUFFER_CAPACITY;
#[cfg(feature = "simulation")]
use crate::wait::Wait;

#[cfg(feature = "simulation")]
pub fn run_tcp<S, F>(host: S, port: u16, mut parser: F) -> Result<JoinHandle<()>>
where
    S: AsRef<str>,
    F: 'static + FnMut(TcpStream) -> Result<()> + Send,
{
    let host = host.as_ref();
    let addr = format!("{}:{}", host, port);
    let listener = TcpListener::bind(addr)?;
    let handle = spawn(move || {
        listener
            .incoming()
            .filter_map(|stream| {
                let stream = stream.ok()?;
                parser(stream).ok()
            })
            .for_each(|()| ());
    });
    Ok(handle)
}

#[cfg(feature = "simulation")]
pub fn parser<F, T>(
    mut handle: F,
    active: Option<Wait<bool>>,
) -> impl FnMut(TcpStream) -> Result<()>
where
    F: FnMut(T) -> Result<()>,
    T: FromStr,
    Error: From<T::Err>,
{
    move |mut stream: TcpStream| {
        let mut buffer = [0u8; TCP_BUFFER_CAPACITY];
        loop {
            let bytes_read = stream.read(&mut buffer).unwrap();
            let did_wait = active
                .clone()
                .and_then(|wait| wait.wait(false).ok())
                .unwrap_or_default();
            match (did_wait, bytes_read) {
                (true, _) => continue,
                (_, 0) => break,
                _ if bytes_read == TCP_BUFFER_CAPACITY => break,
                _ => {
                    let last = buffer[bytes_read - 1] as char;
                    let buffer = &buffer[0..(match last {
                        '\n' => bytes_read - 1,
                        _ => bytes_read,
                    })];
                    String::from_utf8_lossy(buffer)
                        .parse::<T>()
                        .ok()
                        .and_then(|data| handle(data).ok());
                },
            }
        }
        Ok(())
    }
}
