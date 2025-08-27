#![feature(local_waker)]

mod r#async;
pub use crate::r#async::http_request::AsyncHttpRequest;

mod utils;
pub use crate::utils::channel::{channel, Receiver, Sender};
pub use crate::utils::error::GenericError;
pub use crate::utils::responses::{
    error_route, not_found_route, preflight_route, serve_route,
};