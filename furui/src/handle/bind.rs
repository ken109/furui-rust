use aya::Bpf;
use chrono::Local;
use tracing::info;

use furui_common::BindEvent;

use crate::handle::{handle_perf_array, to_str};

pub fn bind(bpf: &mut Bpf) -> anyhow::Result<()> {
    handle_perf_array(
        bpf,
        "BIND_EVENTS",
        Box::new(|event: BindEvent| {
            info!(
                container_id = to_str(event.container_id).as_str(),
                pid = event.pid,
                comm = to_str(event.comm).as_str(),
                protocol = format!("{}{}", event.protocol(), event.family()).as_str(),
                lport = event.lport,
            );
        }),
    )?;

    Ok(())
}
