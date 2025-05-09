use std::{collections::HashMap, ops::IndexMut, time::Duration};

use crate::dyn_pyra::{vec_index_hashmap, CellIndex, Pyramide, PyramideRules};
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    widgets::Paragraph,
    Frame,
};

#[derive(Debug, Default)]
struct Model {
    pyra: Pyramide,
    running_state: RunningState,
    game_state: GameState,
    cursor_pos: CellIndex,
}

#[derive(Debug, Default, PartialEq, Eq)]
enum RunningState {
    #[default]
    Running,
    Done,
}

#[derive(Debug, Default, PartialEq)]
enum GameState {
    #[default]
    ChooseFirstCell,
    ChooseCell,
    ChooseEdge,
    Done,
}

#[derive(PartialEq)]
enum Message {
    CursorMoveUp,
    CursorMoveLeft,
    CursorMoveRight,
    CursorMoveDown,
    SelectCell,
    UnselectCell,
    Enter,
    Reset,
    Quit,
}

pub fn main() -> color_eyre::Result<()> {
    tui::install_panic_hook();
    let mut terminal = tui::init_terminal()?;
    let mut model = Model::default();
    let vec_2_cell_idx = vec_index_hashmap(
        PyramideRules{lines: model.pyra.lines, diag_allowed: model.pyra.diag_allowed}
    );

    while model.running_state != RunningState::Done {
        // Render the current view
        terminal.draw(|f| view(&mut model, f))?;

        // Handle events and map to a Message
        let mut current_msg = handle_event(&model)?;

        // Process updates as long as they return a non-None message
        while current_msg.is_some() {
            current_msg = update(&mut model, current_msg.unwrap(), &vec_2_cell_idx);
        }
    }

    tui::restore_terminal()?;
    Ok(())
}

fn view(model: &mut Model, frame: &mut Frame) {
    let mut board = model.pyra.fmt();
    let vec_idx = model.pyra.cell_index_to_vec_index(model.cursor_pos).expect("checked");
    let max_line = model.pyra.lines * 2 + 1; 
    let crochet_idx = (max_line + vec_idx) * 2;
    board.replace_range(crochet_idx..crochet_idx+1, "(");
    board.replace_range(crochet_idx+2..crochet_idx+3, ")");
    frame.render_widget(
        Paragraph::new(format!("{}", board)),
        frame.area(),
    );
}

/// Convert Event to Message
///
/// We don't need to pass in a `model` to this function in this example
/// but you might need it as your project evolves
fn handle_event(_: &Model) -> color_eyre::Result<Option<Message>> {
    if event::poll(Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                return Ok(handle_key(key));
            }
        }
    }
    Ok(None)
}

fn handle_key(key: event::KeyEvent) -> Option<Message> {
    match key.code {
        KeyCode::Char('z') | KeyCode::Up => Some(Message::CursorMoveUp),
        KeyCode::Char('q') | KeyCode::Left => Some(Message::CursorMoveLeft),
        KeyCode::Char('s') | KeyCode::Down => Some(Message::CursorMoveDown),
        KeyCode::Char('d') | KeyCode::Right => Some(Message::CursorMoveRight),
        KeyCode::Char('e') | KeyCode::Enter => Some(Message::Enter),
        KeyCode::Char('r') => Some(Message::Reset),
        KeyCode::Esc => Some(Message::Quit),
        _ => None,
    }
}

fn update(model: &mut Model, msg: Message, vec_2_cell_idx: &HashMap<usize, usize>) -> Option<Message> {
    match msg {
        // Message::Enter => {
        //     match model.game_state {
        //         GameState::ChooseFirstCell => Some()

        //     }
        // }
        Message::CursorMoveUp => {
            let max_lines = model.pyra.lines * 2 + 1;
            let vec_idx = model.pyra.cell_index_to_vec_index(model.cursor_pos).expect("checked");
            
            if vec_idx >= max_lines {
                model.cursor_pos = *vec_2_cell_idx.get(&(vec_idx-max_lines)).expect("checked");
            }
        },
        Message::CursorMoveDown => {
            let max_lines = model.pyra.lines * 2 + 1;
            let vec_idx = model.pyra.cell_index_to_vec_index(model.cursor_pos).expect("checked");
            
            if vec_idx < max_lines*(model.pyra.lines-1) {
                model.cursor_pos = *vec_2_cell_idx.get(&(vec_idx+max_lines)).expect("checked");
            }
        },
        Message::CursorMoveLeft => {
            if model.cursor_pos > 0 {
                model.cursor_pos -= 1;
            }
        },
        Message::CursorMoveRight => {
            if model.cursor_pos < model.pyra.lines*model.pyra.lines - 1 {
                model.cursor_pos += 1;
            }

        }
        Message::Reset => {model.pyra = Pyramide::init_full(model.pyra.lines, model.pyra.diag_allowed);},
        Message::Quit => {
            // You can handle cleanup and exit here
            model.running_state = RunningState::Done;
        },
        _ => (),
    };
    None
}

mod tui {
    use ratatui::{
        backend::{Backend, CrosstermBackend},
        crossterm::{
            terminal::{
                disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
            },
            ExecutableCommand,
        },
        Terminal,
    };
    use std::{io::stdout, panic};

    pub fn init_terminal() -> color_eyre::Result<Terminal<impl Backend>> {
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
        let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        Ok(terminal)
    }

    pub fn restore_terminal() -> color_eyre::Result<()> {
        stdout().execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn install_panic_hook() {
        let original_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic_info| {
            stdout().execute(LeaveAlternateScreen).unwrap();
            disable_raw_mode().unwrap();
            original_hook(panic_info);
        }));
    }
}