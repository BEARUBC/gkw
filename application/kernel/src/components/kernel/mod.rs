mod grip;
mod parser;

#[cfg(feature = "tcp_edge")]
use std::thread::spawn;

use anyhow::Result;

use crate::components::kernel::grip::GripType;
#[cfg(feature = "tcp_edge")]
use crate::components::utils::create_tcp_runner;
use crate::components::Component;
#[cfg(feature = "tcp_edge")]
use crate::config::Components;
use crate::config::Config;
#[cfg(feature = "tcp_edge")]
use crate::config::TcpComponent;
use crate::wait::Wait;

struct State;

pub(super) struct Kernel {
    pub(super) pause: Wait<bool>,
}

#[cfg(not(feature = "tcp_edge"))]
impl Component for Kernel {
    fn run(self, _: &Config) -> Result<()> {
        todo!()
    }
}

#[cfg(feature = "tcp_edge")]
impl Component for Kernel {
    fn run(
        self,
        Config {
            components: Components { emg, fsr, .. },
            ..
        }: &Config,
    ) -> Result<()> {
        self.launch_emg(emg)?;
        // {
        //     let TcpComponent { host, port } = emg;
        //     let mut grip_type_cache = GripType::default();
        //     #[cfg(feature = "pseudo_analytics")]
        //     let parser = move |data: f64| {
        //         let grip_type = GripType::from(data);
        //         if grip_type_cache != grip_type {
        //             #[cfg(not(release))]
        //             println!("grip type: {:#?}", grip_type);
        //             grip_type_cache = grip_type;
        //         };
        //         Ok(())
        //     };
        //     #[cfg(not(feature = "pseudo_analytics"))]
        //     let parser = |_: f64| Ok(());
        //     let runner = create_tcp_runner(host, *port, parser, Some(self.pause.clone()))?;
        //     spawn(runner);
        // };
        {
            let TcpComponent { host, port } = fsr;
            let parser = |_: f64| Ok(());
            let runner = create_tcp_runner(host, *port, parser, Some(self.pause))?;
            spawn(runner);
        };
        Ok(())
    }
}

impl Kernel {
    fn launch_emg(&self, TcpComponent { host, port }: &TcpComponent) -> Result<()> {
        let mut grip_type_cache = GripType::default();
        #[cfg(feature = "pseudo_analytics")]
        let parser = move |data: f64| {
            let grip_type = GripType::from(data);
            if grip_type_cache != grip_type {
                #[cfg(not(release))]
                println!("grip type: {:#?}", grip_type);
                grip_type_cache = grip_type;
            };
            Ok(())
        };
        #[cfg(not(feature = "pseudo_analytics"))]
        let parser = |_: f64| Ok(());
        let runner = create_tcp_runner(host, *port, parser, Some(self.pause.clone()))?;
        spawn(runner);
        Ok(())
    }

    fn launch_fsr(&self, TcpComponent { host, port }: &TcpComponent) -> Result<()> {
        let parser = |_: f64| Ok(());
        let runner = create_tcp_runner(host, *port, parser, Some(self.pause.clone()))?;
        spawn(runner);
        Ok(())
    }
}
