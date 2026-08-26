pub mod models;
fn login(cred: models::Credentials) {
    // try to login
    crate::database::get_user();
}
