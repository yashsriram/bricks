use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::LinkedList;
use std::fmt::{Display, Formatter, Result};
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

fn main() {
    let url = Url::parse(URL).unwrap();
    let (mut socket, http_response) = connect(url).unwrap();
    println!("{:?}", http_response);
    loop {
        println!("{:-^80}", "");
        socket.write_message(Message::Text(COMMAND.into())).unwrap();
        let parsed = loop {
            let event = socket.read_message().unwrap().to_string();
            let maybe_parsed = serde_json::from_str::<Value>(&event);
            let parsed = match maybe_parsed {
                Ok(parsed) => parsed,
                Err(e) => {
                    eprintln!("{:?}", e);
                    continue;
                }
            };
            if parsed["msg_id"] != EVENT {
                // println!("{:?}", parsed["msg_id"]);
                continue;
            }
            break parsed;
        };
        let data = parsed["payload"]["tree_snapshot"]["data"].clone();
        match serde_json::from_value::<TreeSnapshot>(data) {
            Ok(snapshot) => {
                println!("{}", snapshot);
            }
            Err(e) => {
                eprintln!("{:?}", e);
                continue;
            }
        }
    }
}
