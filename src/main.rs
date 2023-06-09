mod game;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use game::{Game, PossibleMoves, PossiblePlayers};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Clear, Paragraph},
    Frame, Terminal,
};
use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let res = run_app(&mut terminal);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

const NUMBER_OF_ROWS: u8 = 4;
fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let mut game = Game::new(NUMBER_OF_ROWS);
    loop {
        terminal.draw(|f| render(f, &game))?;

        if let Event::Key(key) = event::read()? {
            // Universal keys
            match key.code {
                KeyCode::Char('q') | KeyCode::Char('Q') => return Ok(()),
                KeyCode::Char('r') | KeyCode::Char('R') => game = Game::new(NUMBER_OF_ROWS),
                _ => {}
            }

            // Game keys
            if key.kind == KeyEventKind::Press && !game.is_finished {
                match key.code {
                    KeyCode::Up => game.make_move(PossibleMoves::Up),
                    KeyCode::Down => game.make_move(PossibleMoves::Down),
                    KeyCode::Left => game.make_move(PossibleMoves::Left),
                    KeyCode::Right => game.make_move(PossibleMoves::Right),
                    KeyCode::Enter => {
                        game.make_move(PossibleMoves::Select);
                        game.check_win_conditions();
                    }
                    KeyCode::Char('p') | KeyCode::Char('P') => game.next_player(),
                    _ => (),
                }
            }
        }
    }
}

fn render<B: Backend>(f: &mut Frame<B>, game: &Game) {
    let f_size = f.size();

    // Screen container
    let game_container_constraints = [
        Constraint::Percentage(2),
        Constraint::Percentage(98),
        Constraint::Percentage(2),
    ];
    let game_container_verticla_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(game_container_constraints.as_ref())
        .split(f_size);
    let game_container_horizontal_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(game_container_constraints.as_ref())
        .split(game_container_verticla_chunks[1]);
    let game_border = Block::default().borders(Borders::ALL);
    f.render_widget(game_border, game_container_horizontal_chunks[1]);

    // Game container
    let game_layout_container = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(75), Constraint::Percentage(15)])
        .split(game_container_horizontal_chunks[1]);

    let matches_rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints(game.matches_vertical_container_constraints.as_ref())
        .split(game_layout_container[0]);
    for i in 0..game.matches_number_of_rows {
        let current_row = &game.matches[i];
        let row_len = current_row.len();
        let number_of_spaces = row_len + 1;
        let mut horizontal_container_constrains: Vec<Constraint> =
            Vec::with_capacity(number_of_spaces + 2);

        horizontal_container_constrains.push(Constraint::Percentage(5));

        let constraint = 90 / number_of_spaces;
        for _i in 0..row_len {
            horizontal_container_constrains.push(Constraint::Percentage(constraint as u16));
            horizontal_container_constrains.push(Constraint::Min(2));
        }
        horizontal_container_constrains.push(Constraint::Percentage(constraint as u16));

        horizontal_container_constrains.push(Constraint::Percentage(5));

        let matches_columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(horizontal_container_constrains)
            .split(matches_rows[i + 1]);
        for j in 0..row_len {
            if !current_row[j] {
                continue;
            }

            let mut stick = Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White));
            if game.pointing_to_match.row == i && game.pointing_to_match.column == j {
                stick = stick.style(
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                );
            }
            f.render_widget(stick, matches_columns[j * 2 + 2]);
        }
    }

    // Stats container
    let game_state_container = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),
            Constraint::Min(1),
            Constraint::Min(1),
            Constraint::Min(1),
            Constraint::Min(1),
            Constraint::Min(1),
            Constraint::Min(2),
            Constraint::Min(1),
            Constraint::Min(1),
            Constraint::Min(1),
            Constraint::Min(1),
            Constraint::Min(1),
        ])
        .split(game_layout_container[1]);
    let mut player_1_text = Paragraph::new(Span::styled("Player 1", Style::default()));
    if game.current_player == PossiblePlayers::Player1 {
        player_1_text = player_1_text.style(Style::default().bg(Color::Blue));
    }
    f.render_widget(player_1_text, game_state_container[1]);
    let player_1_matches = Paragraph::new(Span::styled(
        "|".repeat(game.player_1_number_of_matches),
        Style::default().add_modifier(Modifier::BOLD),
    ));
    f.render_widget(player_1_matches, game_state_container[2]);

    let mut player_2_text = Paragraph::new(Span::styled("Player 2", Style::default()));
    if game.current_player == PossiblePlayers::Player2 {
        player_2_text = player_2_text.style(Style::default().bg(Color::Blue));
    }
    f.render_widget(player_2_text, game_state_container[4]);
    let player_2_matches = Paragraph::new(Span::styled(
        "|".repeat(game.player_2_number_of_matches),
        Style::default().add_modifier(Modifier::BOLD),
    ));
    f.render_widget(player_2_matches, game_state_container[5]);

    // Helper menu
    let movement_helper_text = Paragraph::new(Span::styled(
        "U/D/L/R Arrow => move",
        Style::default().add_modifier(Modifier::SLOW_BLINK),
    ))
    .alignment(Alignment::Left);
    f.render_widget(movement_helper_text, game_state_container[7]);

    let selection_helper_text = Paragraph::new(Span::styled(
        "Enter => Remove match",
        Style::default().add_modifier(Modifier::SLOW_BLINK),
    ))
    .alignment(Alignment::Left);
    f.render_widget(selection_helper_text, game_state_container[8]);

    let change_player_helper_text = Paragraph::new(Span::styled(
        "P => Change player",
        Style::default().add_modifier(Modifier::SLOW_BLINK),
    ))
    .alignment(Alignment::Left);
    f.render_widget(change_player_helper_text, game_state_container[9]);

    let restart_helper_text = Paragraph::new(Span::styled(
        "R => Restart",
        Style::default().add_modifier(Modifier::SLOW_BLINK),
    ))
    .alignment(Alignment::Left);
    f.render_widget(restart_helper_text, game_state_container[10]);

    let quit_helper_text = Paragraph::new(Span::styled(
        "Q => Quit",
        Style::default().add_modifier(Modifier::SLOW_BLINK),
    ))
    .alignment(Alignment::Left);
    f.render_widget(quit_helper_text, game_state_container[11]);

    // Win popup
    if game.is_finished {
        let popup_block = Block::default()
            .title(format!(
                "{} wins!",
                match game.current_player {
                    PossiblePlayers::Player1 => "Player 1",
                    PossiblePlayers::Player2 => "Player 2",
                }
            ))
            .borders(Borders::ALL);
        let popup_area = centered_rect(60, 20, f_size);
        f.render_widget(Clear, popup_area);
        f.render_widget(popup_block, popup_area);

        let popup_text = Paragraph::new(Span::styled(
            "Press Q to quit or R to restart.",
            Style::default().add_modifier(Modifier::BOLD),
        ))
        .alignment(Alignment::Center);
        let popup_text_area = centered_rect(90, 20, popup_area);

        f.render_widget(popup_text, popup_text_area);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let center_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(center_layout[1])[1]
}
