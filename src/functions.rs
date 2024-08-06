use crate::enums::{IpAddr, IpAdressKind};

pub fn route_kind(ip_kind: IpAdressKind) {
    println!("showing IP adress of kind {:?}", ip_kind);
}
pub fn route_ip(ip_addr: IpAddr) {
    println!("Home adress is {:?}", ip_addr);
}
pub fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}
pub fn divide_2(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(numerator / denominator)
    }
}
pub fn get_elemet_of_vector(vec: Vec<i32>, index: i32) {
    if index >= 0 && (index as usize) < vec.len() {
        println!("Element at index {}: {}", index, vec[index as usize]);
    } else {
        println!("Index out of bounds");
    }
}
