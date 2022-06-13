use aya::Bpf;
use chrono::Local;
use log::info;

use furui_common::ConnectEvent;

use crate::handle::{handle_perf_array, to_str};

pub fn connect(bpf: &mut Bpf) -> Result<(), anyhow::Error> {
    handle_perf_array(
        bpf,
        "CONNECT_EVENTS",
        Box::new(|event: ConnectEvent| {
            let time = Local::now().format("%H:%M:%S").to_string();

            info!("{} PID {} {}", time, event.pid, to_str(event.comm));
        }),
    )?;

    Ok(())
}
