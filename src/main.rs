#![allow(non_snake_case)]

use crossterm::event::DisableMouseCapture;
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, LeaveAlternateScreen};
use std::io::{self, Read};
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders};
use tui::{backend::CrosstermBackend, Frame, Terminal};

fn ui<B: Backend>(f: &mut Frame<B>) {
  let chunks = Layout::default()
    .direction(Direction::Vertical)
    .margin(1)
    .constraints(
      [
        Constraint::Percentage(10),
        Constraint::Percentage(80),
        Constraint::Percentage(10),
      ]
      .as_ref(),
    )
    .split(f.size());
  let block = Block::default().title("Block").borders(Borders::ALL);
  f.render_widget(block, chunks[0]);
  let block = Block::default().title("Block 2").borders(Borders::ALL);
  f.render_widget(block, chunks[1]);
  let block = Block::default().title("Block 3").borders(Borders::ALL);
  f.render_widget(block, chunks[2]);
}

fn main() -> Result<(), io::Error> {
  // setup terminal
  enable_raw_mode()?;
  let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
  terminal.clear()?;

  loop {
    terminal.draw(|f| ui(f))?;

    let mut ch = [0u8];
    let sz = io::stdin().read(&mut ch)?;
    if sz == 0 || ch[0] == b'q' {
      terminal.clear()?;
      break;
    }
  }

  // restore terminal
  disable_raw_mode()?;
  execute!(
    terminal.backend_mut(),
    LeaveAlternateScreen,
    DisableMouseCapture
  )?;
  terminal.show_cursor()?;

  Ok(())
}
