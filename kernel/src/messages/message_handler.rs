pub trait Handler {
    fn handler(self: &Self) -> ();
}