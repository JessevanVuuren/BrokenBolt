use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Cell, Clear, List, ListItem, Paragraph, Row, Table, Wrap},
};

use crate::ui::app::App;

pub fn ui(frame: &mut Frame, app: &App) {
    let orderbook = Layout::default().direction(Direction::Vertical).constraints([Constraint::Percentage(50), Constraint::Percentage(50)]).split(frame.area());

    let orderbook_block = Block::default().borders(Borders::ALL).style(Style::default());


    let orderbook_table_l = Layout::vertical([Constraint::Percentage(100)]).split(orderbook[0]);

    let widths = [Constraint::Percentage(50), Constraint::Percentage(50)];


    let headers = Row::new(vec!["price", "quantity"]);
    let rows = app.orderbook.asks.iter().enumerate().map(|(i, t)| {
        let color = match i % 2 {
            0 => Color::Blue,
            _ => Color::Red,
        };
        Row::new(vec![Cell::from(t.0.to_string()), Cell::from(t.1.to_string())]).style(Style::new().bg(color))
    });

    let table = Table::new(rows, widths).block(orderbook_block).header(headers);
    frame.render_widget(table, orderbook[0]);
}
