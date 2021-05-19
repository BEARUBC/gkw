use std::future::Future;

pub(crate) enum Wrapper<M>
where
M: 'static + Send,
M: Future, {
    MessageWrapper(M),

    #[allow(unused)]
    RunRequest,
}
