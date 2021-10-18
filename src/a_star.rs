use sokoban::{Actions::*, Directions::*, *};
use std::{collections::BinaryHeap, thread, time::Duration};

fn heuristic(game: &GameBoard) -> usize {
    // a simple heuristic define by sum of minimal manhattan distance of box - target
    game.targets
        .iter()
        .map(|&(tx, ty)| {
            game.boxes
                .iter()
                .map(|&(bx, by)| {
                    let dx = if tx > bx { tx - bx } else { bx - tx };
                    let dy = if ty > by { ty - by } else { by - ty };
                    dx + dy
                })
                .min()
                .unwrap()
        })
        .sum()
}

#[derive(Debug, PartialEq, Eq)]
struct State {
    pub perdict_cost: usize,
    pub game: GameBoard,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.perdict_cost.partial_cmp(&self.perdict_cost)
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.perdict_cost.cmp(&self.perdict_cost)
    }
}

fn a_star(game: &mut GameBoard) -> Option<GameBoard> {
    let mut  queue = BinaryHeap::new();
    queue.push(State {
        perdict_cost: heuristic(&game) + game.actions.len(),
        game: game.clone()
    });

    while !queue.is_empty() {
        let game = queue.pop().unwrap().game;
        game.render();
        thread::sleep(Duration::from_millis(50));
        for d in DIRECTIONS {
            if let Some(Move(last)) = game.actions.last() {
                if matches!(
                    (last, &d),
                    (Left, Right) | (Right, Left) | (Up, Down) | (Down, Up)
                ) {
                    continue;
                }
            }
            let mut game = game.clone();
            if game.move_(&d).is_none() {
                continue;
            };
            if game.is_win() {
                return Some(game);
            }        
            queue.push(State {
                perdict_cost: heuristic(&game) + game.actions.len(),
                game: game.clone()
            });        
        }
    }
    None
}

fn main() {
    let src = include_str!("../resources/maps/map00");
    let mut game = GameBoard::from_src(src);
    if let Some(game) = a_star(&mut game) {
        game.render();
    };
}
