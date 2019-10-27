use std::io::{Error, ErrorKind, Result};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::process::Command;

pub fn get() -> Result<IpAddr> {
    let output = Command::new("hostname").args(&["-i"]).output()?.stdout;
    let source = String::from_utf8(output)
        .map_err(|e| Error::new(ErrorKind::InvalidData, format!("{:?}", e)))?;

    let ips = source.trim().split(" ").collect::<Vec<_>>();
    let first = ips
        .first()
        .ok_or(Error::new(ErrorKind::NotFound, "Ip not found"))?;
    return if let Ok(addr) = first.parse::<Ipv4Addr>() {
        Ok(IpAddr::V4(addr))
    } else if let Ok(addr) = first.parse::<Ipv6Addr>() {
        Ok(IpAddr::V6(addr))
    } else {
        Err(Error::new(ErrorKind::NotFound, "Ip not found"))
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert!(get().is_ok());
    }
}
