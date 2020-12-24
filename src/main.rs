#![warn(clippy::all, clippy::pedantic, clippy::cargo, clippy::nursery)]
#![allow(clippy::multiple_crate_versions)]
// TODO
#![allow(clippy::missing_errors_doc)]
// TODO
#![allow(dead_code)]

mod chain;
mod evm;
mod rpc;
mod server;

pub mod prelude {
    pub use crate::require;
    pub use anyhow::{anyhow, Context as _, Result as AnyResult};
    pub use futures::prelude::*;
    pub use hex_literal::hex;
    pub use itertools::Itertools as _;
    pub use rand::prelude::*;
    pub use rayon::prelude::*;
    pub use serde::{Deserialize, Serialize};
    pub use thiserror::Error;
    pub use tokio::prelude::*;
    pub use tokio_compat_02::FutureExt as Tokio2;
    pub use tracing::{debug, error, info, trace, warn};
    pub use zkp_u256::{Binary as _, One as _, Pow as _, Zero as _, U256};
}

use crate::prelude::*;
use once_cell::sync::OnceCell;
use rand_pcg::Mcg128Xsl64;
use std::sync::{Mutex, MutexGuard};
use structopt::StructOpt;
use tracing_subscriber::FmtSubscriber;

#[macro_export]
macro_rules! require {
    ($condition:expr, $err:expr) => {
        if !$condition {
            return Err($err.into());
        }
    };
}

#[derive(Debug, PartialEq, StructOpt)]
struct Options {
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    verbose: usize,

    /// Number of compute threads to use (defaults to number of cores)
    #[structopt(long)]
    threads: Option<usize>,

    /// Random seed for deterministic random number generation.
    /// If not specified a seed is periodically generated from OS entropy.
    #[structopt(long, parse(try_from_str = parse_hex_u64))]
    seed: Option<u64>,

    #[structopt(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, PartialEq, StructOpt)]
enum Command {
    /// Run an Ethereum JSON-RPC server
    Chain {
        /// Underlying JSON-RPC url to fork from
        #[structopt(long, default_value = "http://localhost:8545")]
        fork: String,
    },
}

fn parse_hex_u64(src: &str) -> Result<u64, std::num::ParseIntError> {
    u64::from_str_radix(src, 16)
}

static RNG: OnceCell<Mutex<Mcg128Xsl64>> = OnceCell::new();

pub fn rng() -> MutexGuard<'static, Mcg128Xsl64> {
    // RNG gets set in main before this function can be called.
    let mutex = unsafe { RNG.get_unchecked() };
    mutex.lock().expect("RNG mutex poisoned")
}

#[must_use]
pub fn random<T>() -> T
where
    rand::distributions::Standard: rand::distributions::Distribution<T>,
{
    rng().gen()
}

#[allow(clippy::cognitive_complexity)]
pub fn main() -> AnyResult<()> {
    // Parse CLI and handle help and version (which will stop the application).
    #[rustfmt::skip]
    let version = format!("\
        {version} {commit} ({commit_date})\n\
        {target} ({build_date})\n\
        {author}\n\
        {homepage}\n\
        {description}",
        version     = env!("CARGO_PKG_VERSION"),
        commit      = &env!("COMMIT_SHA")[..8],
        commit_date = env!("COMMIT_DATE"),
        author      = env!("CARGO_PKG_AUTHORS"),
        description = env!("CARGO_PKG_DESCRIPTION"),
        homepage    = env!("CARGO_PKG_HOMEPAGE"),
        target      = env!("TARGET"),
        build_date  = env!("BUILD_DATE"),
    );
    let matches = Options::clap().long_version(version.as_str()).get_matches();
    let options = Options::from_clap(&matches);

    // Initialize log output (prepend CLI verbosity to RUST_LOG)
    let log_cli = match options.verbose {
        0 => "info",
        1 => "sutro=debug",
        2 => "sutro=trace",
        3 => "sutro=trace,debug,hyper=info,tokio_reactor=info",
        4 => "sutro=trace,debug",
        _ => "trace",
    };
    let log_filter = std::env::var("RUST_LOG").map_or_else(
        |_| log_cli.to_string(),
        |log_env| format!("{},{}", log_cli, log_env),
    );
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(log_filter)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .context("setting default log subscriber")?;
    tracing_log::LogTracer::init().context("adding log compatibility layer")?;

    // Log version information
    info!(
        "{name} {version} {commit}",
        name = env!("CARGO_CRATE_NAME"),
        version = env!("CARGO_PKG_VERSION"),
        commit = &env!("COMMIT_SHA")[..8],
    );

    // Seed the random number generator
    let rng_seed = options
        .seed
        .unwrap_or_else(|| rand::rngs::OsRng::default().next_u64());
    info!("Using random seed {:16x}", rng_seed);
    let rng = Mcg128Xsl64::seed_from_u64(rng_seed);
    RNG.set(Mutex::new(rng)).expect("RNG already set.");

    // Configure Rayon thread pool
    if let Some(threads) = options.threads {
        rayon::ThreadPoolBuilder::new()
            .num_threads(threads)
            .build_global()
            .context("Failed to build thread pool.")?;
    }
    info!(
        "Using {} compute threads on {} cores",
        rayon::current_num_threads(),
        num_cpus::get()
    );

    // Launch Tokio runtime
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .context("Error creating Tokio runtime")?
        .block_on(server::async_main(options))
        .context("Error in main thread")?;

    // Terminate successfully
    info!("program terminating normally");
    Ok(())
}

#[cfg(test)]
pub mod test {
    pub mod prelude {
        pub use pretty_assertions::{assert_eq, assert_ne};
        pub use proptest::prelude::*;
        pub use tracing_test::traced_test;
    }

    use super::*;
    use crate::test::prelude::{assert_eq, *};

    #[test]
    fn parse_args() {
        let cmd = "hello --threads 5 -vvv --seed d5c7b134723a63bf -v";
        let options = Options::from_iter_safe(cmd.split(' ')).unwrap();
        assert_eq!(options, Options {
            verbose: 4,
            seed:    Some(0xd5c7_b134_723a_63bf),
            threads: Some(5),
            command: None,
        });
    }

    #[test]
    #[traced_test]
    fn test_with_log_output() {
        error!("logged on the error level");
        assert!(logs_contain("logged on the error level"));
    }

    #[tokio::test]
    #[traced_test]
    async fn async_test_with_log() {
        // Local log
        info!("This is being logged on the info level");

        // Log from a spawned task (which runs in a separate thread)
        tokio::spawn(async {
            warn!("This is being logged on the warn level from a spawned task");
        })
        .await
        .unwrap();

        // Ensure that `logs_contain` works as intended
        assert!(logs_contain("logged on the info level"));
        assert!(logs_contain("logged on the warn level"));
        assert!(!logs_contain("logged on the error level"));
    }
}

#[cfg(feature = "bench")]
pub mod bench {
    pub mod prelude {
        pub use criterion::{black_box, Criterion};
        pub use futures::executor::block_on;
    }

    #[allow(clippy::wildcard_imports)]
    use super::*;
    use crate::bench::prelude::*;

    #[cfg(feature = "bench")]
    pub fn main(c: &mut Criterion) {
        server::bench::group(c);
    }
}
