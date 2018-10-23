extern crate futures;
//extern crate bincode;
extern crate tower_service;

#[cfg(test)]
extern crate tokio;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate async_bincode;

extern crate tokio_tower;

#[derive(Serialize, Deserialize)]
pub struct Request;

#[derive(Serialize, Deserialize)]
pub struct Response;

impl From<Request> for Response {
    fn from(_: Request) -> Response {
        Response
    }
}

mod pipeline;
