use sokoban::{Actions::*, Directions::*, *};
use std::{thread, time::Duration};

fn dfs(game: &mut GameBoard, max_deep: usize) -> bool {
    if game.is_win() {
        return true;
    }
    if game.actions.len() >= max_deep {
        return false;
    }
    for d in DIRECTIONS {
        if let Some(Move(last)) = game.actions.last() {
            if matches!(
                (last, &d),
                (Left, Right) | (Right, Left) | (Up, Down) | (Down, Up)
            ) {
                continue;
            }
        }
        if game.move_(&d).is_none() {
            continue;
        };
        thread::sleep(Duration::from_millis(50));
        game.render();
        if dfs(game, max_deep) {
            return true;
        }
        game.undo();
    }
    false
}

fn main() {
    let src = include_str!("../resources/maps/map00");
    let mut game = GameBoard::from_src(src);
    for i in 14..50 {
        if dfs(&mut game, i) {
            game.render();
            println!("You win!");
            break;
        }
    }
}
