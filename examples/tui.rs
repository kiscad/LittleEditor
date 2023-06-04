use crossterm::terminal::enable_raw_mode;
use std::io;
use tui::backend::{Backend, CrosstermBackend};
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Tabs};
use tui::{Frame, Terminal};

fn ui<B: Backend>(f: &mut Frame<B>) {
  let size = f.size();
  let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
      Constraint::Length(3), // Heading
      Constraint::Length(3), // Menu
      Constraint::Min(2),    // Content
      Constraint::Length(3), // Footer
    ])
    .split(size);

  let rss_chunks = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([
      Constraint::Percentage(20), // RSS Feed List
      Constraint::Percentage(30), // Article List
      Constraint::Percentage(50), // Article Summary
    ])
    .split(chunks[2]); // Content

  let heading = Paragraph::new("RSS Feeder")
    .style(Style::default().fg(Color::Yellow))
    .alignment(Alignment::Center)
    .block(
      Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .border_type(BorderType::Plain),
    );
  f.render_widget(heading, chunks[0]);

  let menu_titles = vec!["Add", "Update", "Delete", "Quit"];
  let menu = menu_titles
    .iter()
    .map(|t| {
      let (first, rest) = t.split_at(1);
      Spans::from(vec![
        Span::styled(
          first,
          Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::UNDERLINED),
        ),
        Span::styled(rest, Style::default().fg(Color::White)),
      ])
    })
    .collect();

  let menu_titles = Tabs::new(menu)
    .block(Block::default().title("Menu").borders(Borders::ALL))
    .style(Style::default().fg(Color::White))
    .highlight_style(Style::default().fg(Color::Yellow))
    .divider(Span::raw("|"));

  f.render_widget(menu_titles, chunks[1]);

  let left = render_rss_feed_list();
  let (middle, right) = render_rss_article_list();
  f.render_widget(left, rss_chunks[0]);
  f.render_widget(middle, rss_chunks[1]);
  f.render_widget(right, rss_chunks[2]);

  let license = Paragraph::new("Released and maintained under GPL-3.0 license")
    .style(Style::default().fg(Color::LightCyan))
    .alignment(Alignment::Center)
    .block(
      Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .border_type(BorderType::Plain),
    );

  f.render_widget(license, chunks[3]);
}

fn render_rss_feed_list<'a>() -> List<'a> {
  let rss_feeds = Block::default()
    .borders(Borders::ALL)
    .style(Style::default().fg(Color::White))
    .title("RSS Feeds")
    .border_type(BorderType::Plain);

  let items: Vec<_> = vec!["Feed-1", "Feed-2", "Feed-3"]
    .into_iter()
    .map(|feed| ListItem::new(Spans::from(vec![Span::styled(feed, Style::default())])))
    .collect();

  let rss_feed_list = List::new(items).block(rss_feeds).highlight_style(
    Style::default()
      .bg(Color::Yellow)
      .fg(Color::Black)
      .add_modifier(Modifier::BOLD),
  );
  rss_feed_list
}

fn render_rss_article_list<'a>() -> (List<'a>, Paragraph<'a>) {
  let articles = Block::default()
    .borders(Borders::ALL)
    .style(Style::default().fg(Color::White))
    .title("Articles")
    .border_type(BorderType::Plain);

  let items: Vec<_> = vec!["Article-1", "Article-2"]
    .into_iter()
    .map(|feed| ListItem::new(Spans::from(feed)))
    .collect();

  let list = List::new(items).block(articles).highlight_style(
    Style::default()
      .bg(Color::Yellow)
      .fg(Color::Black)
      .add_modifier(Modifier::BOLD),
  );

  let article_summary = Paragraph::new(vec![Spans::from(vec![Span::styled(
    "Article Summary",
    Style::default().fg(Color::LightBlue),
  )])])
  .block(
    Block::default()
      .borders(Borders::ALL)
      .style(Style::default().fg(Color::White))
      .border_type(BorderType::Plain),
  );

  (list, article_summary)
}

fn main() -> Result<(), io::Error> {
  enable_raw_mode()?;
  let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
  terminal.clear()?;

  loop {
    terminal.draw(|f| {
      ui(f);
    })?;
  }
}
