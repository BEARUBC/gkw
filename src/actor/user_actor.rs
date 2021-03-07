/* external crates */

/* external uses */
use actix::prelude::*;

/* internal mods */

/* internal uses */

pub(crate) struct UserActor;

impl UserActor {
    #[allow(unused)]
    pub fn new() -> Self {
        return UserActor;
    }
}

impl Actor for UserActor {
    type Context = Context<Self>;

    fn started(&mut self, _: &mut Context<Self>) {
        println!("user actor has started");
    }
 
    fn stopped(&mut self, _: &mut Context<Self>) {
        println!("user actor has stopped");
    }
}
