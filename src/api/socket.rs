use std::{thread, time::Duration};

use futures_util::{SinkExt, StreamExt, future, pin_mut, stream::SplitSink};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::mpsc::{self, UnboundedReceiver},
    task::JoinHandle,
};
use tokio_tungstenite::{
    MaybeTlsStream, WebSocketStream, connect_async,
    tungstenite::{Error, Message},
};

use crate::api::channels::Channel;

pub struct Socket {
    channels: Vec<Channel>,
    read: Option<JoinHandle<()>>,
    write: Option<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>,
    pub recv_err: Option<UnboundedReceiver<Error>>,
    pub recv_msg: Option<UnboundedReceiver<Message>>,
}

impl Socket {
    pub fn new(channels: Vec<Channel>) -> Self {
        Self {
            channels,
            read: None,
            write: None,
            recv_err: None,
            recv_msg: None,
        }
    }

    pub fn print_channels(&mut self) {
        for channel in self.channels.iter_mut() {
            println!("{}", channel.subscription());
        }
    }

    pub async fn start(&mut self, url: &'static str) -> Result<(), Error> {
        let (stream, response) = connect_async(url).await?;
        println!("WebSocket successfully connected to: {url}");

        let (write, mut read) = stream.split();
        let (send_err, recv_err) = mpsc::unbounded_channel();
        let (send_msg, recv_msg) = mpsc::unbounded_channel();

        let read = tokio::spawn(async move {
            while let Some(msg) = read.next().await {
                match msg {
                    Err(error) => send_err.send(error).unwrap(),
                    Ok(message) => send_msg.send(message).unwrap(),
                }
            }
        });

        self.read = Some(read);
        self.write = Some(write);
        self.recv_err = Some(recv_err);
        self.recv_msg = Some(recv_msg);

        Ok(())
    }

    pub async fn send(&mut self, message: &str) {
        if let Some(mut write) = self.write.as_mut() {
            println!("sending");
            write.send(Message::Text(message.into())).await;
        }
    }

    pub async fn subscribe_to_channels(&mut self) {
        let channels: Vec<String> = self.channels.iter().map(|c| c.subscription()).collect();
        for channel in channels {
            println!("Subscribe -> {}", channel);
            self.send(&channel).await;
        }
    }

    pub async fn stop(&mut self) {
        if let Some(read) = self.read.as_mut() {
            read.abort();
        }

        self.read = None;
    }
}
