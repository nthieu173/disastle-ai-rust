use disastle_castle_rust::Action;
use disastle_rust::game::SchrodingerGameState;
use rand::{thread_rng, Rng};
use std::collections::HashMap;

const EXPLORATION_PARAMETER: f32 = std::f32::consts::SQRT_2;
const LARGE_NUMBER_FOR_UNEXPLORED: f32 = 1000000.0;

fn tree_search(start_node: &mut Node) {
    let mut actions = Vec::new();
    let mut curr: &Node = start_node;
    loop {
        if let Some(action) = start_node.selection() {
            actions.push(action);
            curr = curr.children.get(&action).unwrap();
        } else {
            break;
        }
    }
    drop(curr);
    let mut leaf: &mut Node = start_node;
    for action in actions.iter() {
        leaf = leaf.children.get_mut(action).unwrap();
    }
    leaf.expand();
    if leaf.state.is_victorious(&leaf.player) {
        drop(leaf);
        let mut curr: &mut Node = start_node;
        curr.num_win += 1.0;
        curr.num_play += 1.0;
        for action in actions {
            let mut curr = curr.children.get_mut(&action).unwrap();
            curr.num_win += 1.0;
            curr.num_play += 1.0;
        }
    } else {
        let mut curr: &mut Node = start_node;
        curr.num_play += 1.0;
        for action in actions {
            let mut curr = curr.children.get_mut(&action).unwrap();
            curr.num_play += 1.0;
        }
    }
}

struct Node {
    pub player: String,
    pub state: SchrodingerGameState,
    pub num_win: f32,
    pub num_play: f32,
    pub children: HashMap<Action, Node>,
}

impl Node {
    fn uct_score(&self, parent_num_play: f32) -> f32 {
        if self.num_play == 0.0 {
            return LARGE_NUMBER_FOR_UNEXPLORED
                + thread_rng().gen_range(0.0..LARGE_NUMBER_FOR_UNEXPLORED);
        }
        let win_rate: f32;
        if self.state.is_turn_player(&self.player) {
            win_rate = self.num_win / self.num_play;
        } else {
            win_rate = 1.0 - self.num_win / self.num_play; // Minimax
        }
        win_rate + EXPLORATION_PARAMETER * (parent_num_play.ln() / self.num_play).sqrt()
    }
    pub fn select_best_winrate(&self) -> Option<Action> {
        if self.children.len() == 0 {
            return None;
        }
        let mut max_score = 0.0;
        let mut max_action: &Action = self.children.keys().next().unwrap();
        for (action, node) in self.children.iter() {
            let score = node.num_win / node.num_play;
            if score > max_score {
                max_score = score;
                max_action = action;
            }
        }
        Some(max_action.clone())
    }
    pub fn selection(&self) -> Option<Action> {
        if self.children.len() == 0 {
            return None;
        }
        let mut max_score = 0.0;
        let mut max_action: &Action = self.children.keys().next().unwrap();
        for (action, node) in self.children.iter() {
            let score = node.uct_score(self.num_play);
            if score > max_score {
                max_score = score;
                max_action = action;
            }
        }
        Some(max_action.clone())
    }
    pub fn expand(&mut self) {
        if self.state.is_over() {
            self.num_play += 1.0;
            self.num_win += 1.0;
        }
        let turn_player = &self.state.get_turn_index().to_string();
        for action in self.state.possible_actions(turn_player) {
            self.children.insert(
                action,
                Node {
                    player: self.player.clone(),
                    state: self.state.action(turn_player, action).unwrap(),
                    num_win: 0.0,
                    num_play: 0.0,
                    children: HashMap::new(),
                },
            );
        }
    }
    pub fn evaluation(&self) -> bool {
        self.state.is_victorious(&self.player)
    }
}
