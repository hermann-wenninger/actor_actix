#[cfg(feature = "actix")]
use actix_web::{dev::Payload, FromRequest, HttpRequest,};

#[cfg(feature = "actix")]
use futures::future::{Ready, ok, err};

#[cfg(feature = "actix")]
use crate::errors::{NanoServiceError, NanoServiceErrorStatus};


pub struct HeaderToken {
    pub message: String,
}


#[cfg(feature = "actix")]
impl FromRequest for HeaderToken {
    type Error = NanoServiceError;
    type Future = Ready<Result<HeaderToken, NanoServiceError>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
    
    let raw_data = match req.headers().get("token") {
    Some(data) => data,
    None => { return err(NanoServiceError {
        status: NanoServiceErrorStatus::Unauthorized,
        message: "token not in header under key 'token'".to_string()})}};
    }




}

