#![allow(warnings)]

use enums::{IpAddr, IpAdressKind, Option, Result};
use structs::User;
mod enums;
mod functions;
mod helpers;
mod structs;
fn main() {
    //
    let mut user = User {
        active: true,
        username: String::from("Souimi"),
        email: String::from("email@email.com"),
        sign_in_count: 0,
    };
    println!("Showing user :{:#?}", user);
    //
    functions::route_kind(IpAdressKind::V4);
    functions::route_kind(IpAdressKind::V6);
    //
    functions::route_ip(IpAddr::V4(String::from("172.0.0.1")));
    functions::route_ip(IpAddr::V6(String::from("::1")));
    //
    match functions::divide(10.0, 0.0) {
        Option::Some(x) => println!("Result: {}", x),
        Option::None => println!("Cannot devide by zero"),
    }
    match functions::divide_2(10.0, 0.0) {
        Result::Err(e) => println!("Error: {}", e),
        Result::Ok(res) => println!("Result :{}", res),
    }
    //
    //helpers::native_progress(100, 200, String::from("Testing progress bar"));
}
