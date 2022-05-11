use bevy::{
    prelude::*,
    render::{mesh::Indices, pipeline::PrimitiveTopology},
};
use bricks::vz::{BasePlugins, NON_FILL_PIPELINE};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{f32::consts::PI, net::TcpStream};
use std::{thread, time};
use tungstenite::{connect, stream::MaybeTlsStream, Message, WebSocket};
use url::Url;

const CIRCLE_RESOLUTION: usize = 20;
const CIRCLE_RAD: f32 = 0.2;
const URL: &'static str = "ws://sandbox:8001";
const COMMAND: &'static str = r#"
{
    "msg_id" : "R.SenseAppActivity.TreeSnapshot",
    "payload": {},
    "date_time": null
}
"#;

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct StateMachine {
    nodes: Vec<String>,
    edges: Vec<[String; 3]>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct TreeSnapshot {
    nodes: Vec<String>,
    states: Vec<String>,
    edges: Vec<[String; 2]>,
    depths: Vec<usize>,
    idxes_from_left: Vec<usize>,
    node_state_machines: Vec<StateMachine>,
}

impl From<TreeSnapshot> for Mesh {
    fn from(tree_snapshot: TreeSnapshot) -> Self {
        let tops = tree_snapshot
            .depths
            .iter()
            .map(|&depth| depth as f32 / tree_snapshot.depths.len() as f32);
        let lefts = tree_snapshot
            .idxes_from_left
            .iter()
            .map(|&idx| idx as f32 / tree_snapshot.idxes_from_left.len() as f32);
        let centers = lefts.zip(tops).map(|(left, top)| [left, top]);
        let positions: Vec<[f32; 3]> = centers
            .map(|[left, top]| {
                (0..CIRCLE_RESOLUTION)
                    .map(move |idx| {
                        (0..2).map(move |offset| {
                            let angle = (idx + offset) as f32 / CIRCLE_RESOLUTION as f32 * 2.0 * PI;
                            let point = [
                                left + CIRCLE_RAD * angle.cos(),
                                top + CIRCLE_RAD * angle.sin(),
                                0.0,
                            ];
                            point
                        })
                    })
                    .flatten()
            })
            .flatten()
            .collect();
        let mut mesh = Mesh::new(PrimitiveTopology::LineList);
        mesh.set_indices(Some(Indices::U32((0..positions.len() as u32).collect())));
        mesh.set_attribute(
            Mesh::ATTRIBUTE_COLOR,
            vec![[1.0, 1.0, 1.0, 1.0]; positions.len()],
        );
        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh
    }
}

fn main() {
    let url = Url::parse(URL).unwrap();
    let (socket, http_response) = connect(url).unwrap();
    println!("{:?}", http_response);
    App::build()
        .add_plugins(BasePlugins)
        .insert_resource(socket)
        .add_startup_system(ask_and_vis.system())
        .run();
}

fn ask_and_vis(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut socket: ResMut<WebSocket<MaybeTlsStream<TcpStream>>>,
    asset_server: Res<AssetServer>,
) {
    println!("*");
    meshes.clear();
    let command = Message::Text(COMMAND.into());
    socket.write_message(command).unwrap();
    let event = socket.read_message().unwrap().to_string();
    let parsed = serde_json::from_str::<Value>(&event).unwrap();
    let data = parsed["payload"]["tree_snapshot"]["data"].clone();
    if let Ok(snapshot) = serde_json::from_value::<TreeSnapshot>(data) {
        println!("{:?}", snapshot);
        commands.spawn_bundle(MeshBundle {
            mesh: meshes.add(Mesh::from(snapshot)),
            render_pipelines: RenderPipelines::from_handles(&[NON_FILL_PIPELINE.typed()]),
            ..Default::default()
        });
    }
    // let text_style = TextStyle {
    //     font: asset_server.load("fonts/FiraSans-Bold.ttf"),
    //     font_size: 60.0,
    //     color: Color::WHITE,
    // };
    // commands.spawn_bundle(Text2dBundle {
    //     text: Text::with_section(
    //         "translation",
    //         text_style.clone(),
    //         TextAlignment {
    //             vertical: VerticalAlign::Center,
    //             horizontal: HorizontalAlign::Center,
    //         },
    //     ),
    //     ..Default::default()
    // });
}
