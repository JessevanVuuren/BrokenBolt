use core::panic::PanicMessage;
use crossterm::event::{Event, KeyEvent, read};
use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::Terminal;
use ratatui::crossterm::event::DisableMouseCapture;
use ratatui::crossterm::event::EnableMouseCapture;
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{EnterAlternateScreen, enable_raw_mode};
use ratatui::crossterm::terminal::{LeaveAlternateScreen, disable_raw_mode};
use ratatui::prelude::{Backend, CrosstermBackend};
use std::collections::{BTreeMap, HashMap};
use std::error::Error;
use std::process::exit;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex, mpsc};
use std::time::Duration;
use std::{io, thread};
use tokio_tungstenite::tungstenite::Message;

use broken_bolt::{App, Candle, CandleStick, Ch, Channel, Incoming, KraSoc, Kraken, OrderBook, OrderBookType, Socket, TickerType, ui};

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App, event: Receiver<State>) -> io::Result<bool> {
    loop {
        while let Ok(state) = event.try_recv() {
            match state {
                State::Input(key_event) => {
                    if key_event.kind == KeyEventKind::Press && key_event.code == KeyCode::Char('q') {
                        return Ok(true);
                    }
                }
                State::OrderBook(update) => app.orderbook.stream(update),
                State::Candles(update) => app.candle.web_stream(update),
            }
        }
        terminal.draw(|f| ui(f, app)).expect("failed to render UI");
    }
}

enum State {
    Input(KeyEvent),
    OrderBook(OrderBookType),
    Candles(KraSoc<CandleStick>),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stderr = io::stdout();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // ^ ui stuff
    let pair = "BTC/EUR";
    let interval = 1;

    let extra = ("interval".to_string(), serde_json::to_value(interval).unwrap());

    let ohlc_channel = Channel::new(Ch::OHLC, vec![pair], Some(HashMap::from([extra])));
    let orderbook_channel = Channel::new(Ch::BOOK, vec![pair], None);

    let mut web = Socket::new(vec![orderbook_channel, ohlc_channel]);

    web.start().await.expect("Error socket {}");
    web.subscribe_to_channels(false).await;

    let mut kraken = Kraken::from_env()?;

    let mut orderbook = OrderBook::new(&kraken, pair).await.expect("Failed to init orderbook");
    let mut candles = Candle::new(&kraken, pair, interval).await.expect("Failed to init candle");

    let (event_tx, event_rx) = mpsc::channel::<State>();
    let mut app = App::new(orderbook.clone(), candles.clone());

    let update_key = event_tx.clone();
    thread::spawn(move || read_user_input(update_key));

    let update_state = event_tx.clone();
    let main = tokio::spawn(async move {
        let mut msg = web.recv_msg.take().expect("msg");
        while let Some(data) = msg.recv().await {
            if (data.channel == "subscribe" || data.channel == "heartbeat" || data.channel == "status") {
                continue;
            }
            incoming(data, &mut web, &mut orderbook, &mut candles, &update_state).await
        }
    });

    let _ = run_app(&mut terminal, &mut app, event_rx);

    // v iu stuff

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    Ok(())
}

fn read_user_input(sender: Sender<State>) {
    loop {
        match read().unwrap() {
            Event::Key(key_event) => sender.send(State::Input(key_event)).unwrap(),
            _ => {}
        };
    }
}

async fn incoming(msg: Incoming, soc: &mut Socket, orderbook: &mut OrderBook, candles: &mut Candle, update_ui: &Sender<State>) {
    if msg.channel == "ticker" {
        let ticker: TickerType = serde_json::from_str(&msg.message.to_string()).unwrap();
    }

    if msg.channel == "book" {
        let ob_data: OrderBookType = serde_json::from_str(&msg.message.to_string()).unwrap();
        update_ui.send(State::OrderBook(ob_data.clone()));
    }

    if msg.channel == "ohlc" {
        let ohlc_data: KraSoc<CandleStick> = serde_json::from_str(&msg.message.to_string()).unwrap();
        update_ui.send(State::Candles(ohlc_data.clone()));
    }
}
