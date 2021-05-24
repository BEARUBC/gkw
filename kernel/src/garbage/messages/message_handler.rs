pub trait Handler<T> {
    fn handler(self: &Self) -> T;
}