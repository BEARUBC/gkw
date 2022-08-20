use std::process::Child;
use std::process::ChildStderr;
use std::process::ChildStdin;
use std::process::ChildStdout;
use std::process::Command;
use std::process::Stdio;
use python_integration::{Analytics};

use log::info;

/// Initialize logging.
///
/// For `not(release)` builds, `log-level=trace` will be executed.
/// For `release` builds, `log-level=info` will be executed.
///
/// To learn more about logging and the various levels, visit: https://docs.rs/env_logger/latest/env_logger/.
///
/// In short, the logging precedence is as follows:
/// 1. `error`
/// 2. `warn`
/// 3. `info`
/// 4. `debug`
/// 5. `trace`
pub fn init_logging() {
    #[cfg(not(release))]
    std::env::set_var("RUST_LOG", "trace");
    #[cfg(release)]
    std::env::set_var("RUST_LOG", "info");

    env_logger::init();
}

pub struct GkwSubProcess {
    pub child: Child,
    pub stdin: ChildStdin,
    pub stdout: ChildStdout,
    pub stderr: ChildStderr,
}

impl GkwSubProcess {
    fn new(
        name: &'static str,
        program: &'static str,
        args: &'static [&'static str],
    ) -> gkw_utils::Result<Self> {
        fn format_error<S>(err_msg: S) -> gkw_utils::Error
        where
            S: Into<String>,
        {
            gkw_utils::Error::new(gkw_utils::ErrorCode::unable_to_initialize, Some(err_msg))
        }

        let mut child = Command::new(program)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|_| {
                format_error(format!(
                    "Something went wrong while trying to initialize the {} sub-process.",
                    name
                ))
            })?;

        let stdin = child.stdin.take().ok_or_else(|| format_error(format!("Something went wrong while trying to grab stdin of the newly created {} sub-process.", name)))?;
        let stdout = child.stdout.take().ok_or_else(|| format_error(format!("Something went wrong while trying to grab stdout of the newly created {} sub-process.", name)))?;
        let stderr = child.stderr.take().ok_or_else(|| format_error(format!("Something went wrong while trying to grab stderr of the newly created {} sub-process.", name)))?;

        Ok(Self {
            child,
            stdin,
            stdout,
            stderr,
        })
    }
}

/// Initialization sequence.
///
/// Serves to --apart from other things-- initialize instances of lazy-statics across GKW.
pub async fn init() -> gkw_utils::Result<()> {
    info!("Initializing GKW...");

    // #[allow(unused)]
    // let analytics = GkwSubProcess::new("Analytics", "python", &["<path-to-python-script>"])?;

    // #[allow(unused)]
    // let emg = GkwSubProcess::new("EMG", "<emg-executable>", &["<path-to-emg-binary>"])?;

    let analytics_res = Analytics::new("./application/python_integration/python/wrapper.py");
    let analytics = if let Ok(analytics) = analytics_res{
        analytics
    } else {
        panic!("Failed to start wrapper");
    };

    info!("GKW initialization done.");

    Ok(())
}
