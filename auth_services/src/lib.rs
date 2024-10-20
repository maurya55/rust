#![allow(dead_code,unused_variables)]





mod database;

//  mod database{
//    pub enum  Status {
//          Connected,
//          Interrupted
// }

// pub fn connect_to_database()->Status  {
//     Status::Connected
// }

// pub fn get_user(){

// }
// }

pub mod auth_utils;
// pub mod auth_utils{
//     // use crate::database;



//     pub fn login(cred:models::Credentials){
//     //  super::database::get_user();
//      crate::database::get_user();
//     }

//     pub mod models{
//         pub struct Credentials{
//             pub username:String,
//             pub password:String
//         }
//     }
    
// }


// mod util;



use database::{Status,connect_to_database};



pub fn authenticate (cred:auth_utils::models::Credentials){
    // if let database::Status::Connected= database::connect_to_database(){
    if let Status::Connected= connect_to_database(){
        auth_utils::login(cred);
    }
}