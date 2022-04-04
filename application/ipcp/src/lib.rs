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
pub struct IpcpRequest<P = String, B = Value> {
    pub id: Uuid,

    #[derivative(Hash = "ignore")]
    pub flow_type: FlowType,

    #[derivative(Hash = "ignore")]
    pub method: Method,

    #[derivative(Hash = "ignore")]
    pub path: P,

    #[derivative(Hash = "ignore")]
    pub body: B,
}

impl<'a, P, B> TryFrom<&'a [u8]> for IpcpRequest<P, B>
where
    P: Deserialize<'a>,
    B: Deserialize<'a>,
{
    type Error = gkw_utils::Error;

    fn try_from(data: &'a [u8]) -> gkw_utils::Result<Self> {
        serde_json::from_slice(data).map_err(gkw_utils::Error::from)
    }
}

impl<P, B> TryFrom<IpcpRequest<P, B>> for Vec<u8>
where
    P: Serialize,
    B: Serialize,
{
    type Error = gkw_utils::Error;

    fn try_from(req: IpcpRequest<P, B>) -> gkw_utils::Result<Self> {
        serde_json::to_vec(&req).map_err(gkw_utils::Error::from)
    }
}

#[derive(Derivative, Deserialize, Serialize, Clone, Debug)]
#[derivative(Hash)]
pub struct IpcpResponse<B = Value> {
    pub id: Uuid,

    #[derivative(Hash = "ignore")]
    pub request_id: Uuid,

    #[derivative(Hash = "ignore")]
    pub sequence_number: u16,

    #[derivative(Hash = "ignore")]
    pub body: B,
}

impl<'a, B> TryFrom<&'a [u8]> for IpcpResponse<B>
where
    B: Deserialize<'a>,
{
    type Error = gkw_utils::Error;

    fn try_from(data: &'a [u8]) -> gkw_utils::Result<Self> {
        serde_json::from_slice(data).map_err(gkw_utils::Error::from)
    }
}

impl<B> TryFrom<IpcpResponse<B>> for Vec<u8>
where
    B: Serialize,
{
    type Error = gkw_utils::Error;

    fn try_from(req: IpcpResponse<B>) -> gkw_utils::Result<Self> {
        serde_json::to_vec(&req).map_err(gkw_utils::Error::from)
    }
}
