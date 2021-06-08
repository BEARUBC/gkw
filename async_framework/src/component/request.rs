pub enum Request<M> {
    HandleMessage(M),

    #[allow(unused)]
    RunJob,
}
