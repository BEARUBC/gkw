use actix::prelude::*;


#[derive(Message)]
#[rtype(result = "usize")]
struct Ping(usize);

struct MonitorActor {
    count: usize,
}

impl Actor for MonitorActor {
    type Context = Context<Self>;
}

impl Handler<Ping> for MonitorActor {
    type Result = usize;

    fn handle(&mut self, msg: Ping, _ctx: &mut Context<Self>) -> Self::Result {
        self.count += msg.0;

        self.count
    }
}