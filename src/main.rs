#![allow(warnings)]

use colored::Colorize;
use enums::{IpAddr, IpAdressKind};
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
        Some(x) => println!("{} {}", "Result:".green(), x),
        None => println!("{}", "Cannot devide by zero".red()),
    }
    match functions::divide_2(10.0, 2.0) {
        Err(e) => println!("{} {}", "Error:".red(), e.red()),
        Ok(res) => println!("{}{}", "Result :".green(), res.to_string().green()),
    }
    //
    let vector = vec![1, 2, 3, 4, 5];
    functions::get_elemet_of_vector(vector, 2);
    helpers::native_progress(100, 200, String::from("Testing progress bar"));
}
