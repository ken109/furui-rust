use aya_ebpf::{bindings::TC_ACT_OK, cty::c_long, programs::TcContext};
use furui_common::{EthProtocol, IpProtocol};

use crate::{
    helpers::{ntohs, ETH_HDR_LEN, IPV6_HDR_LEN, IP_HDR_LEN},
    vmlinux::{ethhdr, iphdr, ipv6hdr, tcphdr, udphdr},
};

pub(crate) const NEIGHBOR_SOLICITAION: u8 = 135;
pub(crate) const NEIGHBOR_ADVERTISEMENT: u8 = 136;

#[inline]
pub(crate) fn eth_protocol(ctx: &TcContext) -> Result<EthProtocol, c_long> {
    let eth = ctx.load::<ethhdr>(0)?;

    Ok(EthProtocol::from_eth(ntohs(eth.h_proto)))
}

#[inline]
pub(crate) fn ip_protocol(ctx: &TcContext) -> Result<IpProtocol, c_long> {
    match eth_protocol(ctx)? {
        EthProtocol::IP => {
            let iph = ctx.load::<iphdr>(ETH_HDR_LEN)?;

            Ok(IpProtocol::new(iph.protocol))
        }
        EthProtocol::IPv6 => {
            let iph = ctx.load::<ipv6hdr>(ETH_HDR_LEN)?;

            Ok(IpProtocol::new(iph.nexthdr))
        }
        EthProtocol::Other => Err(TC_ACT_OK as c_long),
    }
}

#[inline]
pub(crate) unsafe fn get_port(ctx: &TcContext) -> Result<(u16, u16), c_long> {
    let ip_hdr_len = match eth_protocol(ctx)? {
        EthProtocol::IP => IP_HDR_LEN,
        EthProtocol::IPv6 => IPV6_HDR_LEN,
        EthProtocol::Other => return Err(TC_ACT_OK as c_long),
    };

    return match ip_protocol(ctx)? {
        IpProtocol::TCP => {
            let tcph = ctx.load::<tcphdr>(ETH_HDR_LEN + ip_hdr_len)?;
            Ok((ntohs(tcph.source), ntohs(tcph.dest)))
        }
        IpProtocol::UDP => {
            let udph = ctx.load::<udphdr>(ETH_HDR_LEN + ip_hdr_len)?;
            Ok((ntohs(udph.source), ntohs(udph.dest)))
        }
        _ => Err(TC_ACT_OK as c_long),
    };
}
