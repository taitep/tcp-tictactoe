use std::{net::TcpListener, sync::mpsc::{Receiver, Sender, channel}};

struct BidirectionalChannelEnd<T> {
    rx: Receiver<T>,
    tx: Sender<T>,
}

impl<T> BidirectionalChannelEnd<T> {
    fn new_pair() -> (Self, Self) {
        let (txa, rxa) = channel::<T>();
        let (txb, rxb) = channel::<T>();
        (
            Self {
                rx: rxa,
                tx: txb,
            },
            Self {
                rx: rxb,
                tx: txa,
            },
        )
    }
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7777");
}
