use lazy_static::lazy_static;

pub const HOST: &'static str = "http://localhost:8000";

lazy_static! {
    pub static ref SIGN_UP_URL: String = format!("{}/user/register", HOST);
}