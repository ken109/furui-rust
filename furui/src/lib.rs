use std::path::PathBuf;

use anyhow::anyhow;
use clap::Parser;
use thiserror::Error;
use tracing::error;
use tracing_core::Level;
use tracing_log::LogTracer;
use tracing_subscriber::FmtSubscriber;

use crate::{
    domain::Containers, ebpf::Loader, map::Maps, parse_policies::ParsePolicies, runtime::Runtime,
};

mod domain;
mod ebpf;
mod handle;
mod map;
mod parse_policies;
mod process;
mod runtime;

#[derive(clap::ValueEnum, PartialEq, Debug, Clone)]
pub enum ContainerRuntime {
    Docker,
    KubernetesCri,
}

#[derive(clap::ValueEnum, PartialEq, Debug, Clone)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

#[derive(clap::ValueEnum, PartialEq, Debug, Clone)]
pub enum LogFormat {
    Json,
    Text,
}

#[derive(Debug, Clone, Parser)]
pub struct Options {
    #[arg(long, short = 'e', value_enum, default_value = "docker")]
    pub container_engine: ContainerRuntime,

    pub policy_path: PathBuf,

    #[cfg_attr(debug_assertions, arg(long, value_enum, default_value = "debug"))]
    #[cfg_attr(not(debug_assertions), arg(long, value_enum, default_value = "info"))]
    pub log_level: LogLevel,

    #[arg(long, value_enum, default_value = "text")]
    pub log_fmt: LogFormat,
}

pub async unsafe fn start(opt: Options) -> anyhow::Result<()> {
    if libc::geteuid() != 0 {
        return Err(anyhow!("You must be root."));
    }

    setup_tracing(&opt)?;

    let container_engine = Runtime::new(&opt).await?;
    let containers = Containers::new();

    container_engine
        .add_running_containers_inspect(containers.clone())
        .await?;

    let policies = match ParsePolicies::new(opt.policy_path.clone()) {
        Ok(parsed_policies) => parsed_policies.to_policies(containers.clone()).await?,
        Err(err) => {
            return Err(err);
        }
    };

    let bpf = ebpf::load_bpf()?;
    let loader = Loader::new(bpf.clone());

    loader.attach_programs().await?;

    let processes = process::get_all(containers.clone()).await;

    let maps = Maps::new(bpf.clone());
    maps.policy.save(policies.clone()).await?;
    maps.container.save_id_with_ips(containers.clone()).await?;
    maps.process.save_all(&processes).await?;

    handle::perf_events(bpf.clone(), maps.clone(), &processes).await?;
    handle::container_events(
        loader.clone(),
        container_engine.clone(),
        maps.clone(),
        containers.clone(),
        policies.clone(),
    );
    handle::policy_events(
        opt.policy_path.clone(),
        maps.clone(),
        policies.clone(),
        containers.clone(),
    )?;

    Ok(())
}

pub fn cleanup() {
    ebpf::detach_programs();
}

#[derive(Error, Debug)]
enum SetupTracingError {
    #[error(transparent)]
    SetLogger(#[from] log::SetLoggerError),

    #[error(transparent)]
    SetGlobalDefault(#[from] tracing_core::dispatcher::SetGlobalDefaultError),
}

fn setup_tracing(opt: &Options) -> Result<(), SetupTracingError> {
    let (level_tracing, level_log) = match opt.log_level {
        LogLevel::Trace => (Level::TRACE, log::LevelFilter::Trace),
        LogLevel::Debug => (Level::DEBUG, log::LevelFilter::Debug),
        LogLevel::Info => (Level::INFO, log::LevelFilter::Info),
        LogLevel::Warn => (Level::WARN, log::LevelFilter::Warn),
        LogLevel::Error => (Level::ERROR, log::LevelFilter::Error),
    };

    let builder = FmtSubscriber::builder().with_max_level(level_tracing);
    match opt.log_fmt {
        LogFormat::Json => {
            let subscriber = builder.json().finish();
            tracing::subscriber::set_global_default(subscriber)?;
        }
        LogFormat::Text => {
            let subscriber = builder.finish();
            tracing::subscriber::set_global_default(subscriber)?;
        }
    };

    LogTracer::builder().with_max_level(level_log).init()?;

    Ok(())
}
