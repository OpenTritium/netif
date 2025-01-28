fn main() {
    for i in netif::up().unwrap() {
        if i.is_ipv6() {
            println!("IPv6: {:?}", i);
        }
    }
}
