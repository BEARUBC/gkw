use std::future::Future;

pub enum JobType<M>
where
M: 'static + Future + Send, {
    Message(M),
    RunRequest,
}
