use disastle_castle_rust::Action;
use disastle_rust::game::SchrodingerGameState;
use rand::{prelude::IteratorRandom, thread_rng, Rng};
use std::collections::HashMap;

const EXPLORATION_PARAMETER: f32 = std::f32::consts::SQRT_2;
const LARGE_NUMBER_FOR_UNEXPLORED: f32 = 1000000.0;

pub fn tree_search(start_node: &mut ShallowNode) {
    let mut actions = Vec::new();
    let mut curr: &ShallowNode = start_node;
    loop {
        if let Some(action) = start_node.selection() {
            actions.push(action);
            curr = curr.children.get(&action).unwrap();
        } else {
            break;
        }
    }
    let mut leaf: &mut ShallowNode = start_node;
    for action in actions.iter() {
        leaf = leaf.children.get_mut(action).unwrap();
    }
    leaf = leaf.expand();
    // Simulation and backpropagation
    if leaf.simulate() {
        drop(leaf);
        let mut curr: &mut ShallowNode = start_node;
        curr.num_win += 1.0;
        curr.num_play += 1.0;
        for action in actions {
            let mut curr = curr.children.get_mut(&action).unwrap();
            curr.num_win += 1.0;
            curr.num_play += 1.0;
        }
    } else {
        let mut curr: &mut ShallowNode = start_node;
        curr.num_play += 1.0;
        for action in actions {
            let mut curr = curr.children.get_mut(&action).unwrap();
            curr.num_play += 1.0;
        }
    }
}

// Stop do not save the expansion when a new shop is dealt (randomness is introducted)
// Then, expand, simulate and backpropagate
pub struct ShallowNode {
    pub player: String,
    pub state: SchrodingerGameState,
    pub num_win: f32,
    pub num_play: f32,
    pub children: HashMap<Action, ShallowNode>,
}

impl ShallowNode {
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
    /*
     * Expand the leaf node and choose one random child to become the new leaf node.
     * If there are no children (terminal state) or if the expansion would rely on randomness
     * (the round incremented and the shop was dealt), we immediately backpropagate.
     */
    pub fn expand(&mut self) -> &mut ShallowNode {
        if !self.state.is_over() {
            let mut is_random = false;
            let mut new_nodes = Vec::new();
            let turn_player = &self.state.get_turn_index().to_string();
            for action in self.state.possible_actions(turn_player) {
                let new_node = ShallowNode {
                    player: self.player.clone(),
                    state: self.state.action(turn_player, action).unwrap(),
                    num_win: 0.0,
                    num_play: 0.0,
                    children: HashMap::new(),
                };
                if new_node.state.round == self.state.round {
                    new_nodes.push((action, new_node));
                } else {
                    is_random = true;
                    break;
                }
            }
            if !is_random {
                for (action, new_node) in new_nodes {
                    self.children.insert(action, new_node);
                }
                return self
                    .children
                    .values_mut()
                    .choose(&mut thread_rng())
                    .unwrap();
            }
        }
        return self;
    }
    pub fn simulate(&self) -> bool {
        let mut game = self.state.clone();
        let mut rng = thread_rng();
        while !game.is_over() {
            let turn_player = &game.turn_order[game.turn_index];
            let actions = self.state.possible_actions(turn_player);
            game = game
                .action(turn_player, actions.into_iter().choose(&mut rng).unwrap())
                .unwrap();
        }
        game.is_victorious(&self.player)
    }
}
