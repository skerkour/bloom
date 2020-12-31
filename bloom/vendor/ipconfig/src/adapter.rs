#![allow(clippy::cast_ptr_alignment)]

use std;
use std::ffi::CStr;
use std::net::IpAddr;

use crate::error::*;
use socket2;
use widestring::WideCString;
use winapi::shared::winerror::{ERROR_SUCCESS, ERROR_BUFFER_OVERFLOW};
use winapi::shared::ws2def::AF_UNSPEC;
use winapi::shared::ws2def::SOCKADDR;

use crate::bindings::*;

/// Represent an operational status of the adapter
/// See IP_ADAPTER_ADDRESSES docs for more details
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OperStatus {
    IfOperStatusUp = 1,
    IfOperStatusDown = 2,
    IfOperStatusTesting = 3,
    IfOperStatusUnknown = 4,
    IfOperStatusDormant = 5,
    IfOperStatusNotPresent = 6,
    IfOperStatusLowerLayerDown = 7,
}

/// Represent an interface type
/// See IANA docs on iftype for more details
/// https://www.iana.org/assignments/ianaiftype-mib/ianaiftype-mib
/// Note that we only support a subset of the IANA interface
/// types and in case the adapter has an unsupported type,
/// `IfType::Unsupported` is used. `IfType::Other`
/// is different from `IfType::Unsupported`, as the former
/// one is defined by the IANA itself.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IfType {
    Other = 1,
    EthernetCsmacd = 6,
    Iso88025Tokenring = 9,
    Ppp = 23,
    SoftwareLoopback = 24,
    Atm = 37,
    Ieee80211 = 71,
    Tunnel = 131,
    Ieee1394 = 144,
    Unsupported,
    /// This enum may grow additional variants, so this makes sure clients
    /// don't count on exhaustive matching. (Otherwise, adding a new variant
    /// could break existing code.)
    #[doc(hidden)]
    __Nonexhaustive,
}

/// Represent an adapter.
#[derive(Debug)]
pub struct Adapter {
    adapter_name: String,
    ip_addresses: Vec<IpAddr>,
    prefixes: Vec<(IpAddr, u32)>,
    gateways: Vec<IpAddr>,
    dns_servers: Vec<IpAddr>,
    description: String,
    friendly_name: String,
    physical_address: Option<Vec<u8>>,
    receive_link_speed: u64,
    transmit_link_speed: u64,
    oper_status: OperStatus,
    if_type: IfType,
    ipv6_if_index: u32,
}

impl Adapter {
    /// Get the adapter's name
    pub fn adapter_name(&self) -> &str {
        &self.adapter_name
    }
    /// Get the adapter's ip addresses (unicast ip addresses)
    pub fn ip_addresses(&self) -> &[IpAddr] {
        &self.ip_addresses
    }
    /// Get the adapter's prefixes. Returns a list of tuples (IpAddr, u32),
    /// where first element is a subnet address, e.g. 192.168.1.0
    /// and second element is prefix length, e.g. 24
    pub fn prefixes(&self) -> &[(IpAddr, u32)] {
        &self.prefixes
    }
    /// Get the adapter's gateways
    pub fn gateways(&self) -> &[IpAddr] {
        &self.gateways
    }
    /// Get the adapter's dns servers (the preferred dns server is first)
    pub fn dns_servers(&self) -> &[IpAddr] {
        &self.dns_servers
    }
    /// Get the adapter's description
    pub fn description(&self) -> &str {
        &self.description
    }
    /// Get the adapter's friendly name
    pub fn friendly_name(&self) -> &str {
        &self.friendly_name
    }
    /// Get the adapter's physical (MAC) address
    pub fn physical_address(&self) -> Option<&[u8]> {
        self.physical_address.as_ref().map(std::vec::Vec::as_slice)
    }

    /// Get the adapter Recieve Link Speed (bits per second)
    pub fn receive_link_speed(&self) -> u64 {
        self.receive_link_speed
    }

    /// Get the Trasnmit Link Speed (bits per second)
    pub fn transmit_link_speed(&self) -> u64 {
        self.transmit_link_speed
    }

    /// Check if the adapter is up (OperStatus is IfOperStatusUp)
    pub fn oper_status(&self) -> OperStatus {
        self.oper_status
    }

    /// Get the interface type
    pub fn if_type(&self) -> IfType {
        self.if_type
    }

    /// Get the IPv6 interface index.
    ///
    /// The return value can be used as an IPv6 scope id for link-local
    /// addresses.
    pub fn ipv6_if_index(&self) -> u32 {
        self.ipv6_if_index
    }
}

/// Get all the network adapters on this machine.
pub fn get_adapters() -> Result<Vec<Adapter>> {
    unsafe {
        // Preallocate 16K per Microsoft recommendation, see Remarks section
        // https://docs.microsoft.com/en-us/windows/desktop/api/iphlpapi/nf-iphlpapi-getadaptersaddresses
        let mut buf_len: ULONG = 16384;
        let mut adapters_addresses_buffer = Vec::new();

        let mut result = ERROR_BUFFER_OVERFLOW;
        while result == ERROR_BUFFER_OVERFLOW {
            adapters_addresses_buffer.resize(buf_len as usize, 0);

            result = GetAdaptersAddresses(
                AF_UNSPEC as u32,
                0x0080 | 0x0010, //GAA_FLAG_INCLUDE_GATEWAYS | GAA_FLAG_INCLUDE_PREFIX,
                std::ptr::null_mut(),
                adapters_addresses_buffer.as_mut_ptr() as PIP_ADAPTER_ADDRESSES,
                &mut buf_len as *mut ULONG,
            );
        }

        if result != ERROR_SUCCESS {
            return Err(Error {
                kind: ErrorKind::Os(result),
            });
        }

        let mut adapters = vec![];
        let mut adapter_addresses_ptr = adapters_addresses_buffer.as_mut_ptr() as PIP_ADAPTER_ADDRESSES;

        while !adapter_addresses_ptr.is_null() {
            adapters.push(get_adapter(adapter_addresses_ptr)?);
            adapter_addresses_ptr = (*adapter_addresses_ptr).Next;
        }

        Ok(adapters)
    }
}

unsafe fn get_adapter(adapter_addresses_ptr: PIP_ADAPTER_ADDRESSES) -> Result<Adapter> {
    let adapter_addresses = &*adapter_addresses_ptr;
    let adapter_name = CStr::from_ptr(adapter_addresses.AdapterName)
        .to_str()?
        .to_owned();
    let dns_servers = get_dns_servers(adapter_addresses.FirstDnsServerAddress)?;
    let gateways = get_gateways(adapter_addresses.FirstGatewayAddress)?;
    let prefixes = get_prefixes(adapter_addresses.FirstPrefix)?;
    let unicast_addresses = get_unicast_addresses(adapter_addresses.FirstUnicastAddress)?;
    let receive_link_speed: u64 = adapter_addresses.ReceiveLinkSpeed;
    let transmit_link_speed: u64 = adapter_addresses.TransmitLinkSpeed;
    let oper_status = match adapter_addresses.OperStatus {
        1 => OperStatus::IfOperStatusUp,
        2 => OperStatus::IfOperStatusDown,
        3 => OperStatus::IfOperStatusTesting,
        4 => OperStatus::IfOperStatusUnknown,
        5 => OperStatus::IfOperStatusDormant,
        6 => OperStatus::IfOperStatusNotPresent,
        7 => OperStatus::IfOperStatusLowerLayerDown,
        v => {
            panic!("unexpected OperStatus value: {}", v);
        }
    };
    let if_type = match adapter_addresses.IfType {
        1 => IfType::Other,
        6 => IfType::EthernetCsmacd,
        9 => IfType::Iso88025Tokenring,
        23 => IfType::Ppp,
        24 => IfType::SoftwareLoopback,
        37 => IfType::Atm,
        71 => IfType::Ieee80211,
        131 => IfType::Tunnel,
        144 => IfType::Ieee1394,
        _ => IfType::Unsupported,
    };
    let ipv6_if_index = adapter_addresses.Ipv6IfIndex;

    let description = WideCString::from_ptr_str(adapter_addresses.Description).to_string()?;
    let friendly_name = WideCString::from_ptr_str(adapter_addresses.FriendlyName).to_string()?;
    let physical_address = if adapter_addresses.PhysicalAddressLength == 0 {
        None
    } else {
        Some(
            adapter_addresses.PhysicalAddress[..adapter_addresses.PhysicalAddressLength as usize]
                .to_vec(),
        )
    };
    Ok(Adapter {
        adapter_name,
        ip_addresses: unicast_addresses,
        prefixes,
        gateways,
        dns_servers,
        description,
        friendly_name,
        physical_address,
        receive_link_speed,
        transmit_link_speed,
        oper_status,
        if_type,
        ipv6_if_index,
    })
}

unsafe fn socket_address_to_ipaddr(socket_address: &SOCKET_ADDRESS) -> IpAddr {
    let sockaddr = socket2::SockAddr::from_raw_parts(
        socket_address.lpSockaddr as *const SOCKADDR,
        socket_address.iSockaddrLength,
    );

    // Could be either ipv4 or ipv6
    sockaddr
        .as_inet()
        .map(|s| IpAddr::V4(*s.ip()))
        .unwrap_or_else(|| IpAddr::V6(*sockaddr.as_inet6().unwrap().ip()))
}

unsafe fn get_dns_servers(
    mut dns_server_ptr: PIP_ADAPTER_DNS_SERVER_ADDRESS_XP,
) -> Result<Vec<IpAddr>> {
    let mut dns_servers = vec![];

    while !dns_server_ptr.is_null() {
        let dns_server = &*dns_server_ptr;
        let ipaddr = socket_address_to_ipaddr(&dns_server.Address);
        dns_servers.push(ipaddr);

        dns_server_ptr = dns_server.Next;
    }

    Ok(dns_servers)
}

unsafe fn get_gateways(mut gateway_ptr: PIP_ADAPTER_GATEWAY_ADDRESS_LH) -> Result<Vec<IpAddr>> {
    let mut gateways = vec![];

    while !gateway_ptr.is_null() {
        let gateway = &*gateway_ptr;
        let ipaddr = socket_address_to_ipaddr(&gateway.Address);
        gateways.push(ipaddr);

        gateway_ptr = gateway.Next;
    }

    Ok(gateways)
}

unsafe fn get_unicast_addresses(
    mut unicast_addresses_ptr: PIP_ADAPTER_UNICAST_ADDRESS_LH,
) -> Result<Vec<IpAddr>> {
    let mut unicast_addresses = vec![];

    while !unicast_addresses_ptr.is_null() {
        let unicast_address = &*unicast_addresses_ptr;
        let ipaddr = socket_address_to_ipaddr(&unicast_address.Address);
        unicast_addresses.push(ipaddr);

        unicast_addresses_ptr = unicast_address.Next;
    }

    Ok(unicast_addresses)
}

unsafe fn get_prefixes(mut prefixes_ptr: PIP_ADAPTER_PREFIX_XP) -> Result<Vec<(IpAddr, u32)>> {
    let mut prefixes = vec![];

    while !prefixes_ptr.is_null() {
        let prefix = &*prefixes_ptr;
        let ipaddr = socket_address_to_ipaddr(&prefix.Address);
        prefixes.push((ipaddr, prefix.PrefixLength));

        prefixes_ptr = prefix.Next;
    }

    Ok(prefixes)
}
