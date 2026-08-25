#![allow(dead_code, unused_variables)]
// iss code ko organized karne k leye ham code ko module me tor de ge
// pub struct Credentials {
//     pub username: String,
//     pub password: String,
// }
// enum Status {
//     Connected,
//     Interrupted,
// }
// fn connect_to_database() -> Status {
//     Status::Connected
// }
// fn get_user(){
//     // fetch the user from db and return
// }
// fn login(cred:Credentials){
//     // try to login
//     get_user();
// }
// pub fn authenticate(cred: Credentials) {
//      if let Status::Connected=connect_to_database(){

//      }
// }

// ye code ham module me tod rahe hai organize k leye
mod database {
    pub enum Status {
        Connected,
        Interrupted,
    }
    pub fn connect_to_database() -> Status {
        Status::Connected
    }
    pub fn get_user() {
        // fetch the user from db and return
    }
}
pub mod auth_utils {
    fn login(cred: models::Credentials) {
        // try to login
        crate::database::get_user();
    }
    pub mod models {
        pub struct Credentials {
            pub username: String,
            pub password: String,
        }
    }
}
pub fn authenticate(cred: auth_utils::models::Credentials) {
    if let database::Status::Connected = database::connect_to_database() {}
}
