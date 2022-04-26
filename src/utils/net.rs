use crate::service::errors::AppError;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

pub fn from_long_notation_to_ip_v6(ip: u128) -> Ipv6Addr {
    return Ipv6Addr::from(ip);
}

pub fn from_long_notation_i32_to_ip_v6(ip: i128) -> Ipv6Addr {
    let converted = ip as u128;
    from_long_notation_to_ip_v6(converted)
}

pub fn from_long_notation_i128_to_ip_v(ip: i128) -> Ipv6Addr {
    let converted = ip as u128;
    from_long_notation_to_ip_v6(converted)
}

pub fn from_long_notation_i64_to_ip_v6(ip: i128) -> Ipv6Addr {
    let converted = ip as u128;
    from_long_notation_to_ip_v6(converted)
}

pub fn to_long_notation_ip_v6(ip: Ipv6Addr) -> u128 {
    u128::from(ip)
}

pub fn from_long_notation_to_ip_v4(ip: u32) -> Ipv4Addr {
    return Ipv4Addr::from(ip);
}

pub fn from_long_notation_i32_to_ip_v4(ip: i32) -> Ipv4Addr {
    let ip_32 = ip as u32;
    from_long_notation_to_ip_v4(ip_32)
}

pub fn from_long_notation_i64_to_ip_v4(ip: i64) -> Ipv4Addr {
    let ip_32 = ip as u32;
    from_long_notation_to_ip_v4(ip_32)
}

pub fn to_long_notation_ip_v4(ip: Ipv4Addr) -> u32 {
    u32::from(ip)
}

pub fn from_ipv6(ip: Ipv6Addr) -> Result<i64, AppError> {
    let ip_as_u64 = u128::from(ip);
    return Ok(ip_as_u64 as i64);
}

pub fn from_ipv4(ip: Ipv4Addr) -> Result<i64, AppError> {
    let ip_as_u64 = u32::from(ip);
    return Ok(ip_as_u64 as i64);
}

pub fn from_ipv6_str(ip: String) -> Result<i64, AppError> {
    let ip_as_obj = Ipv6Addr::from_str(ip.as_str())?;
    let ip_as_u128 = u128::from(ip_as_obj);
    return Ok(ip_as_u128 as i64);
}

pub fn from_ipv4_str(ip: String) -> Result<i64, AppError> {
    let ip_as_obj = Ipv4Addr::from_str(ip.as_str())?;
    let ip_as_u64 = u32::from(ip_as_obj);
    return Ok(ip_as_u64 as i64);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ip_v4_to_u32() {
        let ip = Ipv4Addr::new(1, 1, 128, 0);
        let tr_ip = to_long_notation_ip_v4(ip);
        assert_eq!(tr_ip, 16875520);
    }

    #[test]
    fn test_u32_to_ipv4() {
        let tr_ip = from_long_notation_to_ip_v4(16875520);
        assert_eq!(tr_ip.to_string(), "1.1.128.0");
    }

    #[test]
    fn test_i32_to_ipv4() {
        let tr_ip = from_long_notation_i32_to_ip_v4(34608128);
        assert_eq!(tr_ip.to_string(), "2.16.20.0");
    }

    #[test]
    fn test_i64_to_ipv4() {
        let tr_ip = from_long_notation_i64_to_ip_v4(34615808);
        assert_eq!(tr_ip.to_string(), "2.16.50.0");
    }
}
