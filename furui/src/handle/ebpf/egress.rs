use std::sync::Arc;

use aya::Ebpf;
use furui_common::{Egress6Event, Egress6IcmpEvent, EgressEvent, EgressIcmpEvent};
use tokio::sync::Mutex;
use tracing::info;

use crate::handle::ebpf::handle_perf_array;

pub async fn egress(bpf: Arc<Mutex<Ebpf>>) -> anyhow::Result<()> {
    let args = Arc::new(Mutex::new(()));

    handle_perf_array(
        bpf.clone(),
        "EGRESS_EVENTS",
        args.clone(),
        |event: EgressEvent, _| async move {
            info!(
                event = "egress",
                action = event.action.to_string(),
                container_id = event.container_id().as_str(),
                comm = event.comm().as_str(),
                family = event.family.to_string(),
                protocol = event.protocol.to_string(),
                source_addr = event.src_addr().as_str(),
                source_port = event.sport,
                destination_addr = event.dst_addr().as_str(),
                destination_port = event.dport,
            );
        },
    )
    .await?;

    handle_perf_array(
        bpf.clone(),
        "EGRESS_ICMP_EVENTS",
        args.clone(),
        |event: EgressIcmpEvent, _| async move {
            info!(
                event = "egress",
                action = event.action.to_string(),
                container_id = event.container_id().as_str(),
                family = event.family.to_string(),
                protocol = event.protocol.to_string(),
                source_addr = event.src_addr().as_str(),
                destination_addr = event.dst_addr().as_str(),
                version = event.version.to_string(),
                "type" = event.type_,
                code = event.code,
            );
        },
    )
    .await?;

    handle_perf_array(
        bpf.clone(),
        "EGRESS6_EVENTS",
        args.clone(),
        |event: Egress6Event, _| async move {
            info!(
                event = "egress",
                action = event.action.to_string(),
                container_id = event.container_id().as_str(),
                comm = event.comm().as_str(),
                family = event.family.to_string(),
                protocol = event.protocol.to_string(),
                source_addr = event.src_addr().as_str(),
                source_port = event.sport,
                destination_addr = event.dst_addr().as_str(),
                destination_port = event.dport,
            );
        },
    )
    .await?;

    handle_perf_array(
        bpf,
        "EGRESS6_ICMP_EVENTS",
        args.clone(),
        |event: Egress6IcmpEvent, _| async move {
            info!(
                event = "egress",
                action = event.action.to_string(),
                container_id = event.container_id().as_str(),
                family = event.family.to_string(),
                protocol = event.protocol.to_string(),
                source_addr = event.src_addr().as_str(),
                destination_addr = event.dst_addr().as_str(),
                version = event.version.to_string(),
                "type" = event.type_,
                code = event.code,
            );
        },
    )
    .await?;

    Ok(())
}
