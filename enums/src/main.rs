#[derive(Debug)]
enum IpAddrKind{
    V4,
    V6
}


struct Address{
    address :String,
    kind :IpAddrKind
}

impl Address{
    fn new(ip:&str)->Self{
        Address{
            address:ip.to_string(),
            kind:IpAddrKind::V4
        }
    }
}


fn main() {
    
    // route("22.3.4.3",IpAddrKind::V4);
    // let data= Address{
    //     address:String::from("22.3.4.3"),
    //     kind:IpAddrKind::V4
    // };

    let data =Address::new("113,22,44,4");
    route(data);

}

fn route(ip:Address){
    println!("ip is {} kind is  {:?} ",ip.address,ip.kind);
}

// fn route(ip:&str, kind : IpAddrKind){
//     println!("ip is {ip} kind is  {kind:?} ");
// }