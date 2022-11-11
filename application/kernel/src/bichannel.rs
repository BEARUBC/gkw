use crossbeam::channel::Receiver;
use crossbeam::channel::Sender;

pub struct BiChannel<A, B>(Sender<A>, Receiver<B>);

impl<A, B> BiChannel<A, B> {
    pub fn deref(&self) -> (&Sender<A>, &Receiver<B>) {
        (&self.0, &self.1)
    }
}

pub fn bounded<A, B>(cap: usize) -> (BiChannel<A, B>, BiChannel<B, A>) {
    let (tx_1, rx_1) = crossbeam::channel::bounded(cap);
    let (tx_2, rx_2) = crossbeam::channel::bounded(cap);
    let channel_1 = BiChannel(tx_1, rx_2);
    let channel_2 = BiChannel(tx_2, rx_1);
    (channel_1, channel_2)
}
