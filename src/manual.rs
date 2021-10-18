use crossterm::{event::{read, Event, KeyCode, KeyEvent, KeyModifiers}, terminal::{disable_raw_mode, enable_raw_mode}};
use sokoban::*;

fn main() {
    let src = include_str!("../resources/maps/map00");
    let mut game = GameBoard::from_src(src);
    loop {
        game.render();
        if game.is_win() {
            break;
        }
        enable_raw_mode().unwrap();
        let direction = match read().unwrap() {
            Event::Key(KeyEvent {
                code,
                modifiers: KeyModifiers::NONE,
            }) => match code {
                KeyCode::Char('q') => {
                    println!("Quit!");
                    return;
                }
                KeyCode::Char('u') => {
                    game.undo();
                    continue;
                }
                KeyCode::Left => Directions::Left,
                KeyCode::Right => Directions::Right,
                KeyCode::Up => Directions::Up,
                KeyCode::Down => Directions::Down,
                _ => continue,
            },
            _ => continue,
        };
        disable_raw_mode().unwrap();
        game.move_(&direction);
    }
    println!("\rYou win!");
}
