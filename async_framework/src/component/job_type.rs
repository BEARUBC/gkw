use std::future::Future;

pub(crate) enum JobType<M>
where
M: 'static + Send + Future, {
    Message(M),

    #[allow(unused)]
    RunRequest,
}
