#![allow(dead_code)]

use crate::MacAddressError;
use nix::{ifaddrs::*, sys::socket::SockAddr};

/// Uses the `getifaddrs` call to retrieve a list of network interfaces on the
/// host device and returns the first MAC address listed that isn't
/// local-loopback or if a name was specified, that name.
pub fn get_mac(name: Option<&str>) -> Result<Option<[u8; 6]>, MacAddressError> {
    let ifiter = getifaddrs()?;

    for interface in ifiter {
        if let Some(SockAddr::Link(link)) = interface.address {
            let bytes = link.addr();

            if let Some(name) = name {
                if interface.interface_name == name {
                    return Ok(Some(bytes));
                }
            } else if bytes.iter().any(|&x| x != 0) {
                return Ok(Some(bytes));
            }
        }
    }

    Ok(None)
}

/// Returns a list of MAC addresses for all network devices.
///
/// If no devices are found, it returns an empty Vec.
pub fn get_all_macs() -> Result<Vec<[u8; 6]>, MacAddressError> {
    let ifiter = getifaddrs()?;
    let mut mac_addresses: Vec<[u8; 6]> = Vec::new();

    for interface in ifiter {
        if let Some(SockAddr::Link(link)) = interface.address {

            let bytes = link.addr();

            if bytes.iter().any(|&x| x != 0) {
                mac_addresses.push(bytes);
            }
        }
    }

    Ok(mac_addresses)
}

pub fn get_ifname(mac: &[u8; 6]) -> Result<Option<String>, MacAddressError> {
    let ifiter = getifaddrs()?;

    for interface in ifiter {
        if let Some(SockAddr::Link(link)) = interface.address {
            let bytes = link.addr();

            if &bytes == mac {
                return Ok(Some(interface.interface_name));
            }
        }
    }

    Ok(None)
}
