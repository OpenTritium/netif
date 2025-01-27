fn main(){
    for i in netif::up().unwrap() {
        println!("{:?}", i);
    }
}