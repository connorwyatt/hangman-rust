use crate::game::GuessStatus;
use crate::{
    game::allowed_letters::ALLOWED_LETTER_RANGE,
    ratatui_game_runner::{app::App, tui::Frame},
};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Modifier, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
};

pub(crate) fn render(app: &mut App, frame: &mut Frame) {
    let chunks = Layout::default()
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(1),
        ])
        .split(frame.size());

    render_header(app, frame, chunks[0]);

    render_current_game_and_guesses(app, frame, chunks[1]);

    render_controls(frame, chunks[2]);
}

fn render_header(app: &mut App, frame: &mut Frame, area: Rect) {
    let inner_rect = render_block(frame, area, "Hangman");

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(20), Constraint::Min(20)])
        .split(inner_rect);

    frame.render_widget(Paragraph::new("Welcome to Hangman!"), chunks[0]);
    frame.render_widget(
        Paragraph::new(format!(
            "Played: {}, Won: {}, Lost: {}",
            app.games_played, app.games_won, app.games_lost
        ))
        .alignment(Alignment::Right),
        chunks[1],
    );
}

fn render_current_game_and_guesses(app: &mut App, frame: &mut Frame, area: Rect) {
    let horizontal_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(area);

    render_current_game_and_remaining_letters(app, frame, horizontal_chunks[0]);

    render_lives_and_guesses(app, frame, horizontal_chunks[1]);
}

fn render_current_game_and_remaining_letters(app: &mut App, frame: &mut Frame, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(5), Constraint::Length(5)])
        .split(area);

    render_current_game(app, frame, chunks[0]);

    render_remaining_letters(app, frame, chunks[1]);
}

fn render_current_game(app: &mut App, frame: &mut Frame, area: Rect) {
    let inner_rect = render_block(frame, area, "Current Game").inner(&Margin::new(1, 1));

    let current_word_state = format!("{}", app.game.blanked_out_letters().join(" "));

    render_current_game_state(frame, inner_rect, current_word_state);
}

fn render_current_game_state(frame: &mut Frame, area: Rect, current_word_state: String) {
    let current_word_state_length = current_word_state
        .len()
        .try_into()
        .expect("word length should never be greater than u16 length");

    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min((area.height.checked_sub(1).unwrap_or(0)) / 2),
            Constraint::Length(1),
            Constraint::Min((area.height.checked_sub(1).unwrap_or(0)) / 2),
        ])
        .split(area);

    let horizontal_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(
                (area
                    .width
                    .checked_sub(current_word_state_length)
                    .unwrap_or(0))
                    / 2,
            ),
            Constraint::Length(current_word_state_length),
            Constraint::Min(
                (area
                    .width
                    .checked_sub(current_word_state_length)
                    .unwrap_or(0))
                    / 2,
            ),
        ])
        .split(vertical_chunks[1]);

    frame.render_widget(Paragraph::new(current_word_state), horizontal_chunks[1]);
}

fn render_remaining_letters(app: &App, frame: &mut Frame, area: Rect) {
    let inner_rect = render_block(frame, area, "Remaining Letters").inner(&Margin::new(1, 1));

    let guesses = app.game.guesses();

    let remaining_letters_spans = intersperse(
        ALLOWED_LETTER_RANGE
            .map(|x| x.to_string())
            .filter(|x| !guesses.iter().any(|(guess, _)| x == guess))
            .map(|x| Span::raw(x).add_modifier(Modifier::BOLD))
            .collect::<Vec<Span>>(),
        Span::raw(" "),
    );

    frame.render_widget(
        Paragraph::new(Line::from(remaining_letters_spans)).alignment(Alignment::Center),
        inner_rect,
    );
}

fn render_lives_and_guesses(app: &mut App, frame: &mut Frame, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(4), Constraint::Min(5)])
        .split(area);

    render_lives(app, frame, chunks[0]);

    render_guesses(app, frame, chunks[1]);
}

fn render_lives(app: &mut App, frame: &mut Frame, area: Rect) {
    let inner_rect = render_block(frame, area, "Lives");

    let lives_remaining = app.game.lives_remaining();

    let lives_remaining_span = Span::raw(format!("{}", lives_remaining));
    let lives_remaining_span = match lives_remaining {
        0 => lives_remaining_span.fg(Color::Red),
        1..=5 => lives_remaining_span.fg(Color::Yellow),
        _ => lives_remaining_span.fg(Color::Green),
    };

    frame.render_widget(
        Paragraph::new(vec![
            Line::from(lives_remaining_span.add_modifier(Modifier::BOLD)),
            Line::from(
                Span::raw("lives remaining")
                    .add_modifier(Modifier::DIM)
                    .add_modifier(Modifier::ITALIC),
            ),
        ])
        .alignment(Alignment::Center),
        inner_rect,
    );
}

fn render_guesses(app: &mut App, frame: &mut Frame, area: Rect) {
    let inner_rect = render_block(frame, area, "Guesses");

    let guesses = app.game.guesses();

    let guessed_letters_lines = guesses
        .iter()
        .map(|(x, status)| {
            let span = Span::raw(format!(
                "{} {}",
                x,
                match status {
                    GuessStatus::Correct => "\u{2713}",
                    GuessStatus::Incorrect => "\u{2717}",
                },
            ))
            .add_modifier(Modifier::BOLD);

            Line::from(match status {
                GuessStatus::Correct => span.fg(Color::Green),
                GuessStatus::Incorrect => span.fg(Color::Red),
            })
        })
        .collect::<Vec<Line>>();

    frame.render_widget(
        Paragraph::new(guessed_letters_lines).alignment(Alignment::Center),
        inner_rect,
    );
}

fn render_controls(frame: &mut Frame, area: Rect) {
    let spans: Vec<Span> = vec![
        control_span("Esc/Ctrl-C", "Exit"),
        control_span("A-Z", "Make Guess"),
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

fn control_span<'a>(control: &'a str, action: &'a str) -> [Span<'a>; 2] {
    [
        Span::raw(pad(control.into()))
            .bg(Color::White)
            .fg(Color::Black),
        Span::raw(pad(action.into())),
    ]
}

fn render_block(frame: &mut Frame, area: Rect, title: &str) -> Rect {
    frame.render_widget(
        Block::default()
            .title(pad(title.into()))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
        area,
    );

    area.inner(&Margin::new(2, 1))
}

fn intersperse<T: Clone>(vec: Vec<T>, separator: T) -> Vec<T> {
    let mut vec = vec
        .iter()
        .flat_map(|x| vec![x.clone(), separator.clone()])
        .collect::<Vec<T>>();

    vec.pop();

    vec
}

fn pad(string: String) -> String {
    format!(" {} ", string)
}
