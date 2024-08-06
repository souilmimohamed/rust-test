use crate::enums::{IpAddr, IpAdressKind, Option, Result};

pub fn route_kind(ip_kind: IpAdressKind) {
    println!("showing IP adress of kind {:?}", ip_kind);
}
pub fn route_ip(ip_addr: IpAddr) {
    println!("Home adress is {:?}", ip_addr);
}
pub fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        Option::None
    } else {
        Option::Some(numerator / denominator)
    }
}
pub fn divide_2(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Result::Err(String::from("Cannot divide by zero"))
    } else {
        Result::Ok(numerator / denominator)
    }
}
