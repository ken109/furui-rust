use aya_bpf_cty::c_char;

use furui_macros::{SearchIcmpPolicyKey, SearchPolicyKey};

use crate::{
    EthProtocol, IcmpPolicyKey, IcmpVersion, IpProtocol, PolicyKey, TcAction, CONTAINER_ID_LEN,
    IPV6_LEN, TASK_COMM_LEN,
};

#[derive(Copy, Clone, SearchPolicyKey)]
#[repr(C)]
pub struct IngressEvent {
    pub container_id: [c_char; CONTAINER_ID_LEN],
    #[search_key(remote_ip = 0)]
    pub saddr: u32,
    pub daddr: u32,
    #[search_key(remote_port = 0)]
    pub sport: u16,
    #[search_key(local_port = 0)]
    pub dport: u16,
    pub family: EthProtocol,
    #[search_key(protocol = IpProtocol::default())]
    pub protocol: IpProtocol,
    pub action: TcAction,
    pub comm: [u8; TASK_COMM_LEN],
}

#[cfg(feature = "user")]
impl IngressEvent {
    pub fn src_addr(&self) -> String {
        std::net::Ipv4Addr::from(self.saddr).to_string()
    }

    pub fn dst_addr(&self) -> String {
        std::net::Ipv4Addr::from(self.daddr).to_string()
    }
}

#[derive(Copy, Clone, SearchPolicyKey)]
#[repr(C)]
pub struct Ingress6Event {
    pub container_id: [c_char; CONTAINER_ID_LEN],
    #[search_key(remote_ipv6 = [0; IPV6_LEN])]
    pub saddr: [u8; IPV6_LEN],
    pub daddr: [u8; IPV6_LEN],
    #[search_key(remote_port = 0)]
    pub sport: u16,
    #[search_key(local_port = 0)]
    pub dport: u16,
    pub family: EthProtocol,
    #[search_key(protocol = IpProtocol::default())]
    pub protocol: IpProtocol,
    pub action: TcAction,
    pub comm: [u8; TASK_COMM_LEN],
}

#[cfg(feature = "user")]
impl Ingress6Event {
    pub fn src_addr(&self) -> String {
        std::net::Ipv6Addr::from(self.saddr).to_string()
    }

    pub fn dst_addr(&self) -> String {
        std::net::Ipv6Addr::from(self.daddr).to_string()
    }
}

#[derive(Copy, Clone, SearchIcmpPolicyKey)]
#[repr(C)]
pub struct IngressIcmpEvent {
    pub container_id: [c_char; CONTAINER_ID_LEN],
    #[search_key(remote_ip = 0)]
    pub saddr: u32,
    pub daddr: u32,
    pub family: EthProtocol,
    pub protocol: IpProtocol,
    pub version: IcmpVersion,
    #[search_key(type_ = 255)]
    pub type_: u8,
    #[search_key(code = 255)]
    pub code: u8,
    pub action: TcAction,
}

#[cfg(feature = "user")]
impl IngressIcmpEvent {
    pub fn src_addr(&self) -> String {
        std::net::Ipv4Addr::from(self.saddr).to_string()
    }

    pub fn dst_addr(&self) -> String {
        std::net::Ipv4Addr::from(self.daddr).to_string()
    }
}

#[derive(Copy, Clone, SearchIcmpPolicyKey)]
#[repr(C)]
pub struct Ingress6IcmpEvent {
    pub container_id: [c_char; CONTAINER_ID_LEN],
    #[search_key(remote_ipv6 = [0; IPV6_LEN])]
    pub saddr: [u8; IPV6_LEN],
    pub daddr: [u8; IPV6_LEN],
    pub family: EthProtocol,
    pub protocol: IpProtocol,
    pub version: IcmpVersion,
    #[search_key(type_ = 255)]
    pub type_: u8,
    #[search_key(code = 255)]
    pub code: u8,
    pub action: TcAction,
}

#[cfg(feature = "user")]
impl Ingress6IcmpEvent {
    pub fn src_addr(&self) -> String {
        std::net::Ipv6Addr::from(self.saddr).to_string()
    }

    pub fn dst_addr(&self) -> String {
        std::net::Ipv6Addr::from(self.daddr).to_string()
    }
}
