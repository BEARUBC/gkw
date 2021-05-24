use std::future::Future;

pub enum JobType<M>
where
M: 'static + Send + Future, {
    Message(M),
    RunRequest,
}
