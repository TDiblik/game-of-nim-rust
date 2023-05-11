use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
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

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {
        terminal.draw(|f| render_entry_point(f))?;

        if let Event::Key(key) = event::read()? {
            // Universal keys
            match key.code {
                KeyCode::Char('q') => {
                    return Ok(());
                }
                KeyCode::Char('r') | KeyCode::Char('R') => {
                    return Ok(());
                }
                _ => {}
            }

            // Game keys
            /*
                       if key.kind == KeyEventKind::Press && !game.is_finished {
                           match key.code {
                               KeyCode::Left => game.point_to_previous(),
                               KeyCode::Right => game.point_to_next(),
                               KeyCode::Enter => {
                                   game.change_selection();
                                   game.check_win_conditions();
                               }
                               _ => (),
                           }
                       }
            */
        }
    }
}

fn render_entry_point<B: Backend>(f: &mut Frame<B>) {
    let f_size = f.size();

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

    let game_layout_container = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(75), Constraint::Percentage(15)])
        .split(game_container_horizontal_chunks[1]);
}
