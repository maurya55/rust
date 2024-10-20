
use auth_services::authenticate;
use auth_services::auth_utils::models::Credentials;
// use  auth_services::{authenticate,Credentials};


fn main() {
    println!("Hello, world!");
    let cred=Credentials{
        username:String::from("wap"),
        password:String::from("pass@123"),
    };

    authenticate(cred);


}
