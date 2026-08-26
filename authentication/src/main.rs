use authentication::auth_utils::models::Credentials;
use authentication::authenticate;
use authentication::util;

fn main() {
    let cred = Credentials {
        username: String::from("abdullah dev"),
        password: String::from("admin123"),
    };
    authenticate(cred);
    
}
