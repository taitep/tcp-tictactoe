use std::{
    net::TcpListener,
    sync::mpsc::{Receiver, Sender, channel},
    thread
};

use anyhow::Result;

struct BidirectionalChannelEnd<S, R> {
    rx: Receiver<R>,
    tx: Sender<S>,
}

impl<A, B> BidirectionalChannelEnd<A, B> {
    fn new_pair() -> (BidirectionalChannelEnd<A, B>, BidirectionalChannelEnd<B, A>) {
        let (txa, rxa) = channel::<A>();
        let (txb, rxb) = channel::<B>();
        (
            BidirectionalChannelEnd {
                rx: rxb,
                tx: txa,
            },
            BidirectionalChannelEnd {
                rx: rxa,
                tx: txb,
            },
        )
    }
}

fn game_manager(client_receiver: Receiver<BidirectionalChannelEnd<(), ()>>) {

}

fn handle_client(channel: BidirectionalChannelEnd<(), ()>) {

}

fn main() -> Result<()> {
    let listener = TcpListener::bind("0.0.0.0:7777")?;
    let (send_client, get_client) = channel::<BidirectionalChannelEnd<(), ()>>();

    thread::spawn(move || {
        game_manager(get_client);
    });

    loop {
        let (client, addr) = listener.accept()?;
        println!("Client {addr} has connected, adding to queue...");

        let (server_channel_end, client_channel_end) = BidirectionalChannelEnd::<(), ()>::new_pair();
        send_client.send(server_channel_end).expect("Unhandlable internal error");

        thread::spawn(move || {
            handle_client(client_channel_end);
        });
    }
}
