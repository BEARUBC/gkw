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
use std::ops::Range;
#[cfg(feature = "simulation")]
use std::result;
#[cfg(feature = "simulation")]
use std::thread::spawn;

use anyhow::Result;
use crossbeam::channel::bounded;
use crossbeam::channel::Receiver;
use crossbeam::channel::Sender;
#[cfg(feature = "simulation")]
use log::warn;

use crate::components::bms::Bms;
use crate::components::emg::Emg;
use crate::components::kernel::Kernel;
use crate::config::Config;
use crate::wait::Wait;

#[cfg(feature = "simulation")]
const TCP_BUFFER_CAPACITY: usize = 32;

trait Component {
    fn run(self, _: &Config) -> Result<()>;
}

trait ForwardingComponent: Component {
    const DESTINATION_BUFFER_CAPACITY: usize;
    const DESTINATION_BUFFER_CAPACITY_WARNING_INTERVAL: Range<usize>;

    type Message: 'static + Send + Sync;

    fn tx(&self) -> &Sender<Self::Message>;

    fn send(&self, message: Self::Message) -> Result<()> {
        let tx = self.tx();
        #[cfg(feature = "simulation")]
        utils::buffer_check(
            tx,
            Self::DESTINATION_BUFFER_CAPACITY,
            Self::DESTINATION_BUFFER_CAPACITY_WARNING_INTERVAL,
        );
        let is_full = tx.is_full();
        match is_full {
            true => {
                #[cfg(feature = "simulation")]
                warn!("Buffer full. Dropping message.");
            },
            false => {
                tx.send(message)?;
            },
        };
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
            false => rx.try_recv().ok().and_then(Self::Response::into),
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
    fn tcp_config(_: &Config) -> (&str, &u16);

    fn stream(&self, mut stream: TcpStream, buffer: &mut [u8; TCP_BUFFER_CAPACITY]) -> Result<()> {
        loop {
            let bytes_read = stream.read(buffer)?;
            match bytes_read {
                0 => break,
                _ if bytes_read > TCP_BUFFER_CAPACITY => break,
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
        let mut buffer = [0u8; TCP_BUFFER_CAPACITY];
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
    let (kernel_tx, kernel_rx) = bounded(kernel::MESSAGE_CAPACITY);
    let (emg_tx, emg_rx) = bounded(emg::MESSAGE_CAPACITY);
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
