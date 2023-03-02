#![no_std]
use engine::{Client as G, Direction};
use soroban_sdk::{contractimpl, BytesN, Env};
pub struct Solution;
mod engine {
    soroban_sdk::contractimport!(file = "../game_engine.wasm");
}
mod test;
#[contractimpl]
impl Solution {
    pub fn solve(env: Env, engine_id: BytesN<32>) {
        let e = G::new(&env, &engine_id);
        e.p_turn(&Direction::Left);
        e.p_move(&Some(6));
        while e.p_pos().0 < 285 {
            match e.get_map().iter().filter_map(Result::ok).next() {
                Some((poi, _el)) => {
                    e.p_turn(if poi.1 <= e.p_pos().1 { &Direction::Down } else { &Direction::Up });
                    e.p_move(&Some((poi.1 - e.p_pos().1).abs() as u32));
                    e.p_turn(&Direction::Right);
                    e.p_move(&Some((poi.0 - e.p_pos().0) as u32));
                    e.p_harvest();
                    e.p_shoot();
                }
                _ => {
                    e.p_move(&Some(1));
                }
            }
        }
    }
}
