mod mcts;

use disastle_castle_rust::{Room, SimpleRoom};
use disastle_rust::{
    disaster::{Disaster, SimpleDisaster},
    game::{player::PlayerInfo, GameSetting, GameState},
    load_disasters, load_rooms,
};
use mcts::{tree_search, ShallowNode};
use rand::prelude::SliceRandom;
use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let disasters: Vec<SimpleDisaster> = load_disasters(&Path::new("disasters.ron"))?;
    let disasters: HashSet<Box<dyn Disaster>> =
        disasters.into_iter().map(|d| d.to_disaster()).collect();
    let rooms: Vec<SimpleRoom> = load_rooms(&Path::new("rooms.ron"))?;
    let rooms: HashSet<Box<dyn Room>> = rooms.into_iter().map(|r| r.to_room()).collect();
    let thrones: Vec<SimpleRoom> = load_rooms(&Path::new("thrones.ron"))?;
    let thrones: HashSet<Box<dyn Room>> = thrones.into_iter().map(|r| r.to_room()).collect();
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
    let mut game = GameState::new(&players, setting);
    let ai = ["1", "2", "3", "4"]
        .choose(&mut rand::thread_rng())
        .unwrap()
        .to_string();
    let mut i = 0;
    while !game.is_over() {
        i += 1;
        println!("Round {}", i);
        if game.is_turn_player(&ai) {
            let mut node = ShallowNode {
                state: game.to_schrodinger(),
                children: HashMap::new(),
                num_play: 0.0,
                num_win: 0.0,
                player: game.get_player_turn_index(&ai).unwrap().to_string(),
                is_random: false,
            };
            println!("{:?}", node.state.castles[&node.player]);
            for _ in 0..100 {
                tree_search(&mut node);
            }
            game = game
                .action(&ai, node.select_best_winrate().unwrap())
                .unwrap();
        }
        for i in 1..5 {
            if i.to_string() != ai && game.is_turn_player(&i.to_string()) {
                let actions = game.possible_actions(&i.to_string());
                if let Some(action) = actions.choose(&mut rand::thread_rng()) {
                    game = game.action(&i.to_string(), *action).unwrap();
                }
            }
        }
    }
    if game.is_victorious(&ai) {
        println!("AI is the winner :)");
    } else {
        println!("AI lost to random :(");
    }
    Ok(())
}
