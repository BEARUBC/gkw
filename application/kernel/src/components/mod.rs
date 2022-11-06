mod bms;
mod emg;
mod kernel;
mod utils;

#[cfg(feature = "simulation")]
use std::io::Read;
#[cfg(feature = "simulation")]
use std::net::TcpListener;
#[cfg(feature = "simulation")]
use std::net::TcpStream;
#[cfg(feature = "simulation")]
use std::result;
#[cfg(feature = "simulation")]
use std::thread::spawn;

use anyhow::Result;
use crossbeam::channel::bounded;
use crossbeam::channel::Receiver;
use crossbeam::channel::Sender;

use crate::components::bms::Bms;
use crate::components::emg::Emg;
use crate::components::kernel::Kernel;
use crate::config::Config;
use crate::wait::Wait;

const MESSAGE_CAPACITY: usize = 32;
const RESPONSE_CAPACITY: usize = 16;
#[cfg(feature = "simulation")]
const BUFFER_CAPACITY: usize = 32;

trait Component {
    fn run(self, _: &Config) -> Result<()>;
}

trait ForwardingComponent: Component {
    type Message: 'static + Send + Sync;

    fn tx(&self) -> &Sender<Self::Message>;

    fn send(&self, message: Self::Message) -> Result<()> {
        self.tx().send(message)?;
        Ok(())
    }
}

trait BackPressuredForwardingComponent: ForwardingComponent {
    const WAIT_CONDITION: Self::WaitCondition;

    type Response: Into<Option<Wait<Self::WaitCondition>>>;
    type WaitCondition: PartialEq;

    fn rx(&self) -> &Receiver<Self::Response>;

    fn read_last(&self, block: bool) -> Result<bool> {
        let rx = self.rx();
        let len = rx.len();
        if len > 1 {
            let range = 0..(len - 1);
            range.for_each(|_| {
                rx.recv().ok();
            });
        };
        let wait = match block {
            true => rx.recv()?.into(),
            false => rx.try_recv().ok().map(Self::Response::into).flatten(),
        };
        let did_wait = match wait {
            Some(wait) => {
                wait.wait(Self::WAIT_CONDITION)?;
                true
            },
            None => false,
        };
        Ok(did_wait)
    }

    fn send_and_apply_pressure(&self, message: Self::Message) -> Result<()> {
        let did_wait = self.read_last(false)?;
        if !did_wait {
            self.send(message)?;
            self.read_last(true)?;
        };
        Ok(())
    }
}

#[cfg(feature = "simulation")]
trait TcpComponent: 'static + ForwardingComponent + Sized + Send {
    fn tcp_config<'a>(_: &'a Config) -> (&'a str, &'a u16);

    fn stream(&self, mut stream: TcpStream, buffer: &mut [u8; BUFFER_CAPACITY]) -> Result<()> {
        loop {
            let bytes_read = stream.read(buffer)?;
            match bytes_read {
                0 => break,
                _ if bytes_read > BUFFER_CAPACITY => break,
                _ => {
                    let last = buffer[bytes_read - 1] as char;
                    let buffer = &buffer[0..(match last {
                        '\n' => bytes_read - 1,
                        _ => bytes_read,
                    })];
                    self.handle(buffer).ok();
                },
            }
        }
        Ok(())
    }

    fn handle(&self, _: &[u8]) -> Result<()>;

    fn run_tcp(self, config: &Config) -> Result<()> {
        let (host, port) = Self::tcp_config(config);
        let addr = format!("{}:{}", host, port);
        let listener = TcpListener::bind(addr)?;
        let mut buffer = [0u8; BUFFER_CAPACITY];
        spawn(move || {
            listener
                .incoming()
                .filter_map(result::Result::ok)
                .for_each(|stream| {
                    self.stream(stream, &mut buffer).ok();
                });
        });
        Ok(())
    }
}

pub(super) fn run(config: Config) -> Result<()> {
    let (kernel_tx, kernel_rx) = bounded(MESSAGE_CAPACITY);
    let (emg_tx, emg_rx) = bounded(RESPONSE_CAPACITY);
    let kernel = Kernel {
        emg: emg_tx,
        rx: kernel_rx,
    };
    let emg = Emg {
        tx: kernel_tx.clone(),
        rx: emg_rx,
    };
    let bms = Bms { tx: kernel_tx };
    emg.run(&config)?;
    bms.run(&config)?;
    kernel.run(&config)?;
    Ok(())
}
