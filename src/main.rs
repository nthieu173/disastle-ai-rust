mod mcts;

use disastle_castle_rust::Room;
use disastle_rust::{
    game::{player::PlayerInfo, GameSetting, GameState},
    load_disasters, load_rooms,
};
use mcts::{tree_search, ShallowNode};
use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let disasters = load_disasters(&Path::new("disasters.ron"))?
        .into_iter()
        .map(|d| d.to_disaster())
        .collect();
    let mut rooms: HashSet<Box<dyn Room>> = load_rooms(&Path::new("rooms.ron"))?
        .into_iter()
        .map(|r| r.to_room())
        .collect();
    for throne in load_rooms(&Path::new("thrones.ron"))?.into_iter() {
        rooms.insert(throne.to_room());
    }
    let setting = GameSetting {
        disasters,
        rooms,
        num_disasters: 5,
        num_safe: 15,
        num_shop: 5,
    };

    let players = vec![
        PlayerInfo {
            name: "one".to_string(),
            secret: "1".to_string(),
        },
        PlayerInfo {
            name: "two".to_string(),
            secret: "2".to_string(),
        },
        PlayerInfo {
            name: "three".to_string(),
            secret: "3".to_string(),
        },
        PlayerInfo {
            name: "four".to_string(),
            secret: "4".to_string(),
        },
    ];
    let game = GameState::new(&players, setting).schrodinger("1").unwrap();
    let mut node = ShallowNode {
        state: game,
        children: HashMap::new(),
        num_play: 0.0,
        num_win: 0.0,
        player: "1".to_string(),
    };
    println!("Node initilization successful");
    for _ in 0..100 {
        tree_search(&mut node);
    }
    Ok(())
}
