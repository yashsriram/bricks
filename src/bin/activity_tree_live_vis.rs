use crossterm::{
    event::{self, poll, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::{Display, Formatter, Result};
use std::{collections::LinkedList, time::Duration};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
    Frame, Terminal,
};
use tungstenite::{connect, Message};
use url::Url;

const URL: &'static str = "ws://sandbox:8001";
const COMMAND: &'static str = r#"
{
    "msg_id" : "R.RDKAppActivity.TreeSnapshot",
    "payload": {},
    "date_time": null
}
"#;
const EVENT: &'static str = "R.RDKAppActivity.TreeSnapshot";

#[derive(Debug, Serialize, Deserialize)]
struct TreeSnapshot {
    nodes: Vec<String>,
    states: Vec<String>,
    children: Vec<Vec<usize>>,
}

impl Display for TreeSnapshot {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        struct FringeElement {
            node_idx: usize,
            left_rank: usize,
        }
        let mut fringe = LinkedList::from([FringeElement {
            node_idx: 0,
            left_rank: 0,
        }]);
        while let Some(FringeElement {
            node_idx,
            left_rank,
        }) = fringe.pop_back()
        {
            write!(
                f,
                "{:\t<width$}└─{name}\n",
                "",
                name = self.nodes[node_idx],
                width = left_rank
            )?;
            for child in self.children[node_idx].iter() {
                fringe.push_back(FringeElement {
                    node_idx: *child,
                    left_rank: left_rank + 1,
                });
            }
        }
        Ok(())
    }
}

impl Widget for TreeSnapshot {
    fn render(self, area: Rect, buf: &mut Buffer) {
        struct FringeElement {
            node_idx: usize,
            left_rank: usize,
        }
        let mut fringe = LinkedList::from([FringeElement {
            node_idx: 0,
            left_rank: 0,
        }]);
        let mut top_rank = 0;
        for x in area.left()..area.right() {
            for y in area.top()..area.bottom() {
                buf.get_mut(x, y).reset();
            }
        }
        while let Some(FringeElement {
            node_idx,
            left_rank,
        }) = fringe.pop_back()
        {
            buf.set_string(
                area.left() + left_rank as u16 * 4,
                area.top() + top_rank as u16,
                format!("└─{}", self.nodes[node_idx],),
                Style::default(),
            );
            top_rank += 1;
            for child in self.children[node_idx].iter() {
                fringe.push_back(FringeElement {
                    node_idx: *child,
                    left_rank: left_rank + 1,
                });
            }
        }
    }
}

fn main() {
    let url = Url::parse(URL).unwrap();
    let (mut socket, http_response) = connect(url).unwrap();
    println!("{:?}", http_response);

    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    loop {
        socket.write_message(Message::Text(COMMAND.into())).unwrap();
        let parsed = loop {
            let event = socket.read_message().unwrap().to_string();
            let maybe_parsed = serde_json::from_str::<Value>(&event);
            let parsed = match maybe_parsed {
                Ok(parsed) => parsed,
                Err(e) => {
                    continue;
                }
            };
            if parsed["msg_id"] != EVENT {
                continue;
            }
            break parsed;
        };
        let data = parsed["payload"]["tree_snapshot"]["data"].clone();
        match serde_json::from_value::<TreeSnapshot>(data) {
            Ok(snapshot) => {
                terminal
                    .draw(|frame| {
                        frame.render_widget(snapshot, frame.size());
                    })
                    .unwrap();
                if poll(Duration::from_millis(0)).unwrap() {
                    if let Event::Key(key) = event::read().unwrap() {
                        if let KeyCode::Char('q') = key.code {
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                continue;
            }
        }
    }

    // restore terminal
    disable_raw_mode().unwrap();
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )
    .unwrap();
    terminal.show_cursor().unwrap();
}
