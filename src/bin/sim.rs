use serde::{Deserialize, Serialize};
use serde_json::Value;
use tungstenite::{connect, Message};
use url::Url;

const URL: &'static str = "ws://sandbox:1235";
const COMMAND: &'static str = "{}";

#[derive(Debug, Serialize, Deserialize)]
struct RobotInfo {
    fov_rads: f32,
    heading_angle: f32,
    pose_px_coords: [u32; 2],
}

#[derive(Debug, Serialize, Deserialize)]
struct Message2 {
    footprint_polygon: Vec<[u32; 2]>,
    is_path_following: bool,
    robot_info: RobotInfo,
}

fn main() {
    let url = Url::parse(URL).unwrap();
    let (mut socket, http_response) = connect(url).unwrap();
    println!("{:?}", http_response);
    println!("{:-^80}", "");
    socket.write_message(Message::Text(COMMAND.into())).unwrap();
    {
        let event = socket.read_message().unwrap().to_string();
        let parsed = serde_json::from_str::<Value>(&event).unwrap();
        println!("{:?}", parsed.to_string());
    }
    loop {
        let event = socket.read_message().unwrap().to_string();
        let parsed = serde_json::from_str::<Value>(&event).unwrap();
        match serde_json::from_value::<Message2>(parsed) {
            Ok(message2) => {
                println!("{:?}", message2);
            }
            Err(e) => {
                eprintln!("{:?}", e);
                continue;
            }
        }
    }
}
