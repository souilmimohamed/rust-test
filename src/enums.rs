#[derive(Debug)]
pub enum IpAdressKind {
    V4,
    V6,
}
#[derive(Debug)]
pub enum IpAddr {
    V4(String),
    V6(String),
}

// error handling enums
pub enum Option<T> {
    Some(T),
    None,
}
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}
