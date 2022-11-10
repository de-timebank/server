pub mod auth;
pub mod rating;
pub mod service_request;
pub mod user;

pub type Result<T> = std::result::Result<T, tonic::Status>;

#[allow(unused)]
pub mod error_messages {
    pub const INVALID_PAYLOAD: &str = "INVALID PAYLOAD";
    pub const UNKNOWN: &str = "AN ERROR HAS OCCURED";
    pub const ALREADY_EXISTS: &str = "ITEM ALREADY EXISTS";
    pub const MISSING_ARGUMENT: &str = "EXPECTED ARGUMENT MISSING";
    pub const TOO_MANY_REQUESTS: &str = "TOO MANY REQUESTS";
}
