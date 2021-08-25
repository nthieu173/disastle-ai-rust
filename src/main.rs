mod mcts;

use disastle_castle_rust::{Room, SimpleRoom};
use disastle_rust::{
    disaster::{Disaster, SimpleDisaster},
    game::{player::PlayerInfo, GameSetting, GameState},
    load_disasters, load_rooms,
};
use mcts::{tree_search, ShallowNode};
use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let disasters: Vec<SimpleDisaster> = load_disasters(&Path::new("disasters.ron"))?;
    let disasters: HashSet<Box<dyn Disaster>> =
        disasters.into_iter().map(|d| d.to_disaster()).collect();
    let rooms: Vec<SimpleRoom> = load_rooms(&Path::new("rooms.ron"))?;
    let mut rooms: HashSet<Box<dyn Room>> = rooms.into_iter().map(|r| r.to_room()).collect();
    let thrones: Vec<SimpleRoom> = load_rooms(&Path::new("thrones.ron"))?;
    let mut thrones: HashSet<Box<dyn Room>> = thrones.into_iter().map(|r| r.to_room()).collect();
    let setting = GameSetting {
        disasters,
        thrones,
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
    let game = GameState::new(&players, setting);
    let ai_index = game.get_player_turn_index("1")?;
    let game = game.to_schrodinger();
    let mut node = ShallowNode {
        state: game,
        children: HashMap::new(),
        num_play: 0.0,
        num_win: 0.0,
        player: ai_index.to_string(),
    };
    println!("Node initilization successful");
    for _ in 0..100 {
        tree_search(&mut node);
    }
    println!("{:?}", node.select_best_winrate());
    Ok(())
}
