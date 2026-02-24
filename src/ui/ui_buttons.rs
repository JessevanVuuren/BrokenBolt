use std::sync::mpsc::Sender;

use crossterm::event::MouseEvent;
use ratatui::{
    Frame,
    layout::Rect,
    style::Color,
    widgets::{Block, Borders},
};

use crate::{
    Button, Message,
    ui::{
        globals::{BEAR_COLOR, BULL_COLOR, DULL_COLOR},
        utils::{abs_scale_rect, offset_rect},
    },
};

pub fn ui_buttons(button_area: Rect, frame: &mut Frame, mouse: &Option<MouseEvent>, update: Sender<Message>) {
    //
    ///////////////////////////  Candle type
    //

    let b_candle_type = Block::new().borders(Borders::ALL).title(" Candle type ");
    let candle_type = b_candle_type.inner(button_area);
    let button_width = candle_type.width / 2;

    let mut candle_stick = Button::new(button_width, 1, "Candle stick", &offset_rect(&candle_type, button_width * 0, 0)).bg(DULL_COLOR);
    let mut heiken_ashi = Button::new(button_width, 1, "Heiken Ashi ", &&offset_rect(&candle_type, button_width * 1, 0)).bg(DULL_COLOR);

    candle_stick.mouse(mouse);
    heiken_ashi.mouse(mouse);

    frame.render_widget(&candle_stick, candle_stick.rect);
    frame.render_widget(&heiken_ashi, heiken_ashi.rect);

    frame.render_widget(&b_candle_type, abs_scale_rect(&button_area, button_area.width, 3));

    //
    ///////////////////////////  Coin Type
    //

    let button_area = offset_rect(&button_area, 0, 3);
    let b_coin_pair = Block::new().borders(Borders::ALL).title(" Crypto pair ");
    let coin_pair = b_coin_pair.inner(button_area);
    let button_width = coin_pair.width / 2;

    let mut coin_pair_0 = Button::new(button_width, 1, "BTC/EUR", &offset_rect(&coin_pair, button_width * 0, 0)).bg(DULL_COLOR);
    let mut coin_pair_1 = Button::new(button_width, 1, "ETH/EUR", &&offset_rect(&coin_pair, button_width * 1, 0)).bg(DULL_COLOR);
    let mut coin_pair_2 = Button::new(button_width, 1, "SOL/EUR", &offset_rect(&coin_pair, button_width * 0, 1)).bg(DULL_COLOR);
    let mut coin_pair_3 = Button::new(button_width, 1, "XRP/EUR", &&offset_rect(&coin_pair, button_width * 1, 1)).bg(DULL_COLOR);
    let mut coin_pair_4 = Button::new(button_width, 1, "ADA/EUR", &offset_rect(&coin_pair, button_width * 0, 2)).bg(DULL_COLOR);
    let mut coin_pair_5 = Button::new(button_width, 1, "DOGE/EUR", &&offset_rect(&coin_pair, button_width * 1, 2)).bg(DULL_COLOR);

    coin_pair_0.callback({
        let update = update.clone();
        move || {
            update.send(Message::UpdateCandlesPair("BTC/EUR".into())).ok();
        }
    });

    coin_pair_1.callback({
        let update = update.clone();
        move || {
            update.send(Message::UpdateCandlesPair("ETH/EUR".into())).ok();
        }
    });

    coin_pair_2.callback({
        let update = update.clone();
        move || {
            update.send(Message::UpdateCandlesPair("SOL/EUR".into())).ok();
        }
    });

    coin_pair_3.callback({
        let update = update.clone();
        move || {
            update.send(Message::UpdateCandlesPair("XRP/EUR".into())).ok();
        }
    });

    coin_pair_4.callback({
        let update = update.clone();
        move || {
            update.send(Message::UpdateCandlesPair("ADA/EUR".into())).ok();
        }
    });

    coin_pair_5.callback({
        let update = update.clone();
        move || {
            update.send(Message::UpdateCandlesPair("DOGE/EUR".into())).ok();
        }
    });

    coin_pair_0.mouse(mouse);
    coin_pair_1.mouse(mouse);
    coin_pair_2.mouse(mouse);
    coin_pair_3.mouse(mouse);
    coin_pair_4.mouse(mouse);
    coin_pair_5.mouse(mouse);

    frame.render_widget(&coin_pair_0, coin_pair_0.rect);
    frame.render_widget(&coin_pair_1, coin_pair_1.rect);
    frame.render_widget(&coin_pair_2, coin_pair_2.rect);
    frame.render_widget(&coin_pair_3, coin_pair_3.rect);
    frame.render_widget(&coin_pair_4, coin_pair_4.rect);
    frame.render_widget(&coin_pair_5, coin_pair_5.rect);

    frame.render_widget(&b_coin_pair, abs_scale_rect(&button_area, button_area.width, 5));

    //
    /////////////////////////// time scale
    //

    let button_area = offset_rect(&button_area, 0, 5);
    let b_candle_time = Block::new().borders(Borders::ALL).title(" Time Scale ");
    let candle_time = b_candle_time.inner(button_area);
    let b_time_w = candle_time.width / 3;

    let mut m1 = Button::new(b_time_w, 1, "1m", &offset_rect(&candle_time, b_time_w * 0, 0)).bg(DULL_COLOR);
    let mut m5 = Button::new(b_time_w, 1, "5m", &&offset_rect(&candle_time, b_time_w * 1, 0)).bg(DULL_COLOR);
    let mut m15 = Button::new(b_time_w, 1, "15m", &offset_rect(&candle_time, b_time_w * 2, 0)).bg(DULL_COLOR);
    let mut m30 = Button::new(b_time_w, 1, "30m", &offset_rect(&candle_time, b_time_w * 0, 1)).bg(DULL_COLOR);
    let mut h1 = Button::new(b_time_w, 1, "1h", &offset_rect(&candle_time, b_time_w * 1, 1)).bg(DULL_COLOR);
    let mut h4 = Button::new(b_time_w, 1, "4h", &offset_rect(&candle_time, b_time_w * 2, 1)).bg(DULL_COLOR);
    let mut d1 = Button::new(b_time_w, 1, "1d", &offset_rect(&candle_time, b_time_w * 0, 2)).bg(DULL_COLOR);
    let mut w1 = Button::new(b_time_w, 1, "1w", &offset_rect(&candle_time, b_time_w * 1, 2)).bg(DULL_COLOR);
    let mut d15 = Button::new(b_time_w, 1, "15d", &offset_rect(&candle_time, b_time_w * 2, 2)).bg(DULL_COLOR);

    m1.callback({
        let update = update.clone();
        move || {
            update.send(Message::UpdateCandlesInterval(1)).ok();
        }
    });

    m5.callback({
        let update = update.clone();
        move || {
            update.send(Message::UpdateCandlesInterval(5)).ok();
        }
    });

    m15.callback({
        let update = update.clone();
        move || {
            update.send(Message::UpdateCandlesInterval(15)).ok();
        }
    });

    m30.callback({
        let update = update.clone();
        move || {
            update.send(Message::UpdateCandlesInterval(30)).ok();
        }
    });

    h1.callback({
        let update = update.clone();
        move || {
            update.send(Message::UpdateCandlesInterval(60)).ok();
        }
    });

    h4.callback({
        let update = update.clone();
        move || {
            update.send(Message::UpdateCandlesInterval(240)).ok();
        }
    });

    d1.callback({
        let update = update.clone();
        move || {
            update.send(Message::UpdateCandlesInterval(1440)).ok();
        }
    });

    w1.callback({
        let update = update.clone();
        move || {
            update.send(Message::UpdateCandlesInterval(10080)).ok();
        }
    });

    d15.callback({
        let update = update.clone();
        move || {
            update.send(Message::UpdateCandlesInterval(21600)).ok();
        }
    });

    m1.mouse(mouse);
    m5.mouse(mouse);
    m15.mouse(mouse);
    m30.mouse(mouse);
    h1.mouse(mouse);
    h4.mouse(mouse);
    d1.mouse(mouse);
    w1.mouse(mouse);
    d15.mouse(mouse);

    frame.render_widget(&m1, m1.rect);
    frame.render_widget(&m5, m5.rect);
    frame.render_widget(&m15, m15.rect);
    frame.render_widget(&m30, m30.rect);
    frame.render_widget(&h1, h1.rect);
    frame.render_widget(&h4, h4.rect);
    frame.render_widget(&d1, d1.rect);
    frame.render_widget(&w1, w1.rect);
    frame.render_widget(&d15, d15.rect);

    frame.render_widget(&b_candle_time, abs_scale_rect(&button_area, button_area.width, 5));

    //
    /////////////////////////// time scale
    //

    let button_area = offset_rect(&button_area, 0, 5);

    let b_button_area = Block::new().borders(Borders::ALL).title(" Execute order ");
    let b_order = b_button_area.inner(button_area);

    let button_width = b_order.width / 2;

    let mut buy = Button::new(button_width, 3, "BUY", &offset_rect(&b_order, 0, 0)).bg(BULL_COLOR);
    let mut sell = Button::new(button_width, 3, "SELL", &offset_rect(&b_order, button_width, 0)).bg(BEAR_COLOR);

    buy.mouse(mouse);
    sell.mouse(mouse);

    frame.render_widget(&buy, buy.rect);
    frame.render_widget(&sell, sell.rect);

    frame.render_widget(&b_button_area, abs_scale_rect(&button_area, button_area.width, 5));
}
