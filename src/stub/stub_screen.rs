use crate::model::{ApplicationEvent, ApplicationModel, GlobalEvent};
use crate::model::ScreenTrait;
use crossterm::event;
use crossterm::event::{Event, KeyCode};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::prelude::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph, Wrap};
use ratatui::Frame;
use crate::stub::model::StubEvent;

pub struct StubScreen {}

impl StubScreen {
    pub fn new() -> Self {
        StubScreen {}
    }

    fn get_stub_details(&self, app: &ApplicationModel) -> String {
        if app.stub_model.stubs.is_empty() {
            return "No stubs available".to_string();
        }

        let stub = &app.stub_model.stubs[app.stub_model.selected_stub_index];

        // Format as JSON for readability
        match serde_json::to_string_pretty(&stub) {
            Ok(json) => json,
            Err(_) => format!("{:#?}", stub), // Fallback to Debug format
        }
    }
}

impl ScreenTrait for StubScreen {
    fn draw(&self, app: &ApplicationModel, f: &mut Frame) {
        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(f.area());

        // Title
        let title = Paragraph::new("Wire Mock Inspector - Stub Mappings")
            .style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(title, main_layout[0]);

        // Split middle area into two columns
        let content_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(40), Constraint::Percentage(60)])
            .split(main_layout[1]);

        // Stubs list display (left side)
        let items: Vec<ListItem> = app
            .stub_model
            .stubs
            .iter()
            .enumerate()
            .map(|(i, stub)| {
                let url = stub
                    .request
                    .url
                    .as_ref()
                    .map(|s| s.as_str())
                    .unwrap_or("(no url)");

                let (text, style) = if i == app.stub_model.selected_stub_index {
                    (
                        format!("▶ {} {}", stub.request.method, url),
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    )
                } else {
                    (
                        format!("  {} {}", stub.request.method, url),
                        Style::default().fg(Color::White),
                    )
                };

                ListItem::new(text).style(style)
            })
            .collect();

        let stubs_list = List::new(items).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Stub Mappings"),
        );

        f.render_widget(stubs_list, content_layout[0]);

        // Details view (right side)
        let details = self.get_stub_details(app);

        let details_paragraph = Paragraph::new(details)
            .block(Block::default().borders(Borders::ALL).title("Details"))
            .style(Style::default().fg(Color::White))
            .wrap(Wrap { trim: false })
            .scroll((app.stub_model.scroll_offset as u16, 0));

        f.render_widget(details_paragraph, content_layout[1]);

        // Commands
        let commands = vec![
            "↑/k: Up",
            "↓/j: Down",
            "PgUp/PgDn: Scroll",
            "r: Refresh",
            "d: Delete",
            "q: Quit",
            "a: Toggle auto refresh",
        ];

        let control_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Ratio(1, commands.len() as u32);
                commands.len()
            ])
            .split(main_layout[2]);

        for (index, command) in commands.iter().enumerate() {
            let paragraph = Paragraph::new(*command);
            f.render_widget(paragraph, control_layout[index]);
        }
    }

    fn event_handling(&self) -> Result<Option<ApplicationEvent>, std::io::Error> {
        if let Event::Key(key) = event::read()? {
            let msg = match key.code {
                KeyCode::Char('a') => ApplicationEvent::Stub(StubEvent::ToggleAutoRefresh),
                KeyCode::Char('r') => ApplicationEvent::Stub(StubEvent::ReadAllStubs),
                KeyCode::Char('q') => ApplicationEvent::Global(GlobalEvent::Quit),
                KeyCode::Char('d') => ApplicationEvent::Stub(StubEvent::DeleteSelected),
                KeyCode::Up | KeyCode::Char('k') => {
                    ApplicationEvent::Stub(StubEvent::SelectPrevious)
                }
                KeyCode::Down | KeyCode::Char('j') => ApplicationEvent::Stub(StubEvent::SelectNext),
                KeyCode::PageUp => ApplicationEvent::Stub(StubEvent::ScrollDetailsUp),
                KeyCode::PageDown => ApplicationEvent::Stub(StubEvent::ScrollDetailsDown),
                _ => ApplicationEvent::None,
            };
            return Ok(Some(msg));
        }
        Ok(None)
    }
}
