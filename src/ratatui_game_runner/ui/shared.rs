use crate::ratatui_game_runner::tui::Frame;
use ratatui::{
    layout::{Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Style, Stylize},
    text::Span,
    widgets::{Block, BorderType, Borders},
};

pub(crate) fn control_span<'a>(control: &'a str, action: &'a str) -> [Span<'a>; 2] {
    [
        Span::raw(pad(control.into()))
            .bg(Color::White)
            .fg(Color::Black),
        Span::raw(pad(action.into())),
    ]
}

pub(crate) fn render_block(frame: &mut Frame, area: Rect, title: &str) -> Rect {
    render_styled_block(frame, area, title, Style::default())
}

pub(crate) fn render_styled_block(
    frame: &mut Frame,
    area: Rect,
    title: &str,
    style: Style,
) -> Rect {
    frame.render_widget(
        Block::default()
            .title(pad(title.into()))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(style)
            .border_style(style)
            .title_style(style),
        area,
    );

    area.inner(&Margin::new(2, 1))
}

pub(crate) fn centered_rect(width: u16, height: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length((r.height - height) / 2),
            Constraint::Length(height),
            Constraint::Length((r.height - height) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length((r.width - width) / 2),
            Constraint::Length(width),
            Constraint::Length((r.width - width) / 2),
        ])
        .split(popup_layout[1])[1]
}

pub(crate) fn intersperse<T: Clone>(vec: Vec<T>, separator: T) -> Vec<T> {
    let mut vec = vec
        .iter()
        .flat_map(|x| vec![x.clone(), separator.clone()])
        .collect::<Vec<T>>();

    vec.pop();

    vec
}

pub(crate) fn pad(string: String) -> String {
    format!(" {} ", string)
}
