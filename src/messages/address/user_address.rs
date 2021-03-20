/* external crates */

/* external uses */
use actix::prelude::*;

/* internal mods */

/* internal uses */
use crate::{
    messages::response::Response,
    actor::user_actor::UserActor,
};

#[derive(Message)]
#[rtype(result = "Response<bool>")]
pub struct UserAddress(pub Addr<UserActor>);
