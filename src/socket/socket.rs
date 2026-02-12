use std::{thread, time::Duration};

use futures_util::{
    SinkExt, StreamExt, future, pin_mut,
    stream::{SplitSink, SplitStream},
};
use serde::Deserializer;
use serde_json::Value;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
    task::JoinHandle,
};
use tokio_tungstenite::{
    MaybeTlsStream, WebSocketStream, connect_async,
    tungstenite::{Error, Message, http::method},
};

use crate::socket::channels::{self, Channel};

pub const WEBSOCKET_URL: &str = "wss://ws.kraken.com/v2";

pub struct Socket {
    channels: Vec<Channel>,
    read: Option<JoinHandle<()>>,
    write: Option<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>,
    pub recv_err: Option<UnboundedReceiver<Error>>,
    pub recv_msg: Option<UnboundedReceiver<Incoming>>,
}

pub struct Incoming {
    pub message: Message,
    pub channel: String,
    pub pair: String,
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

    pub async fn start(&mut self) -> Result<(), Error> {
        let (stream, response) = connect_async(WEBSOCKET_URL).await?;

        let (write, mut read) = stream.split();
        let (send_err, recv_err) = mpsc::unbounded_channel();
        let (send_msg, recv_msg) = mpsc::unbounded_channel();

        let read = tokio::spawn(async move {
            Socket::handle_message(&mut read, send_err, send_msg).await;
        });

        self.read = Some(read);
        self.write = Some(write);
        self.recv_err = Some(recv_err);
        self.recv_msg = Some(recv_msg);

        Ok(())
    }

    async fn handle_message(read: &mut SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>, send_err: UnboundedSender<Error>, send_msg: UnboundedSender<Incoming>) {
        while let Some(msg) = read.next().await {
            match msg {
                Err(error) => {
                    let _ = send_err.send(error);
                }
                Ok(raw) => {
                    if let Ok(msg) = Socket::build_message(raw) {
                        let _ = send_msg.send(msg);
                    }
                }
            }
        }
    }

    fn build_message(message: Message) -> Result<Incoming, std::io::Error> {
        let object: Value = serde_json::from_str(&message.to_string())?;

        let mut channel = String::new();
        let mut pair = String::new();

        if let (Some(c), Some(p)) = (object.pointer("/method").and_then(Value::as_str), object.pointer("/result/symbol").and_then(Value::as_str)) {
            channel = c.to_owned();
            pair = p.to_owned();
        } else if let (Some(c), Some(p)) = (object.pointer("/channel").and_then(Value::as_str), object.pointer("/data/0/symbol").and_then(Value::as_str)) {
            channel = c.to_owned();
            pair = p.to_owned();
        } else if let Some(c) = object.pointer("/channel").and_then(Value::as_str) {
            channel = c.to_owned();
        }

        Ok(Incoming {
            message,
            channel,
            pair,
        })
    }

    pub async fn send(&mut self, message: &str) {
        if let Some(mut write) = self.write.as_mut() {
            write.send(Message::Text(message.into())).await;
        }
    }

    pub async fn subscribe_to_channels(&mut self, log:bool) {
        let channels: Vec<String> = self.channels.iter().map(|c| c.subscription()).collect();
        for channel in channels {
            if log {
                println!("Subscribe -> {}", channel);
            }
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
