// IpAdress enumerations
#[derive(Debug)]
enum IpAdress {
    V4,
    V6,
}

// Ip Address use the struct
#[derive(Debug)]
struct Ip {
    version: IpAdress,
    addr: String,
}

// use enums impl the ip
#[derive(Debug)]
pub enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

pub fn ip_struct(addr: String) {
    let ipv4 = Ip{
        version: IpAdress::V4,
        addr,
    };
    let ipv6 = Ip{version: IpAdress::V6, addr: "::1".to_string()};

    println!("The ip address use struct");
    println!("{:?}, {:?}", ipv4, ipv6);
}

