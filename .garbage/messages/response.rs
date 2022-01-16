#[allow(unused)]
pub enum Response<T> {
    Accepted(T),
    Rejected(Rejected),
}

#[allow(unused)]
pub enum Rejected {
    EventLoopTooFull,
    InvalidState,
    Other,
}
