extern crate serde;

use derivative::*;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use serde_repr::Deserialize_repr;
use serde_repr::Serialize_repr;
use strum_macros::Display;
use uuid::Uuid;

#[allow(non_camel_case_types)]
#[derive(Deserialize_repr, Serialize_repr, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[repr(u8)]
pub enum FlowType {
    packet = 0,
    stream = 1,
}

#[derive(Display, Deserialize, Serialize, Clone, Debug)]
pub enum Method {
    GET,
    SEND,
}

#[derive(Derivative, Deserialize, Serialize, Clone, Debug)]
#[derivative(Hash)]
pub struct IpcpRequest<P = String>
where
    P: ToString,
{
    id: Uuid,

    #[derivative(Hash = "ignore")]
    flow_type: FlowType,

    #[derivative(Hash = "ignore")]
    method: Method,

    #[derivative(Hash = "ignore")]
    path: P,

    #[derivative(Hash = "ignore")]
    body: Value,
}

#[derive(Derivative, Deserialize, Serialize, Clone, Debug)]
#[derivative(Hash)]
pub struct IpcpResponse {
    id: Uuid,

    #[derivative(Hash = "ignore")]
    request_id: Uuid,

    #[derivative(Hash = "ignore")]
    sequence_number: u16,

    #[derivative(Hash = "ignore")]
    body: Value,
}
