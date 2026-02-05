enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
enum IpAddrKindV2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

// std lib way of doing an enum
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    route(four);
    route(six);
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let home = IpAddrKindV2::V4(127, 0, 0, 1);
    let loopback = IpAddrKindV2::V6("::1".to_string());
    println!("{:#?}\n{:#?}", home, loopback);

    let _some_number = Some(5);
    let _some_char = Some('e');
    let _absent_number: Option<i32> = None;
}

fn route(ip: IpAddrKind) -> IpAddrKind {
    ip
}
