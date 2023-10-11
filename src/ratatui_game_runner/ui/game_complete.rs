use crate::{
    game::{CompleteGameStatus, GameStatus},
    ratatui_game_runner::{
        app::{App, CurrentView, PlayAgain},
        tui::Frame,
        ui::shared,
    },
};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Clear, Paragraph},
};

pub(crate) fn render(app: &mut App, frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(frame.size());

    render_complete_game_dialog(app, frame, shared::centered_rect(50, 9, chunks[0]));

    render_controls(frame, chunks[1]);
}

fn render_complete_game_dialog(app: &mut App, frame: &mut Frame, area: Rect) {
    frame.render_widget(Clear, area);

    let GameStatus::Complete(complete_game_status) = &app.current_game.status() else {
        unreachable!("this dialog is only shown when game is complete");
    };

    let (title, color) = match complete_game_status {
        CompleteGameStatus::Won => ("You Won!", Color::Green),
        CompleteGameStatus::Lost => ("You Lost...", Color::Red),
    };

    let inner_block = shared::render_styled_block(frame, area, title, Style::default().fg(color))
        .inner(&Margin::new(1, 1));

    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Length(1),
        ])
        .split(inner_block);

    let horizontal_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(vertical_chunks[2]);

    frame.render_widget(
        Paragraph::new(format!("{}", app.current_game.letters().join(" ")))
            .alignment(Alignment::Center),
        vertical_chunks[0],
    );

    frame.render_widget(
        Paragraph::new("Would you like to play again?").alignment(Alignment::Center),
        vertical_chunks[1],
    );

    let CurrentView::GameComplete(game_complete_state) = &app.current_view else {
        unreachable!("this dialog is only shown when the current view is GameComplete");
    };

    let selected_style = Style::default().bg(color).fg(Color::Black);

    frame.render_widget(
        Paragraph::new("Yes").alignment(Alignment::Center).style(
            match game_complete_state.currently_selected {
                PlayAgain::Yes => selected_style,
                PlayAgain::No => Style::default(),
            },
        ),
        horizontal_chunks[0],
    );
    frame.render_widget(
        Paragraph::new("No").alignment(Alignment::Center).style(
            match game_complete_state.currently_selected {
                PlayAgain::Yes => Style::default(),
                PlayAgain::No => selected_style,
            },
        ),
        horizontal_chunks[1],
    );
}

fn render_controls(frame: &mut Frame, area: Rect) {
    frame.render_widget(Clear, area);

    let spans: Vec<Span> = vec![
        shared::control_span("Esc/Ctrl-C", "Exit"),
        shared::control_span("←/→", "Move"),
        shared::control_span("Enter", "Select"),
    ]
    .iter()
    .flatten()
    .map(|x| x.clone())
    .collect();

    frame.render_widget(
        Paragraph::new(Line::from(spans)).alignment(Alignment::Center),
        area,
    );
}
