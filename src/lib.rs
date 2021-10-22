use std::fmt::Display;

use crossterm::{
    cursor, execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Entities {
    Wall,
    Road,
    Target,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Directions {
    Left,
    Right,
    Up,
    Down,
}

pub const DIRECTIONS: [Directions; 4] = [
    Directions::Left,
    Directions::Right,
    Directions::Up,
    Directions::Down,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Actions {
    Move(Directions),
    Push(Directions),
}

impl Display for Actions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Actions::Move(Directions::Left) => write!(f, "ML"),
            Actions::Move(Directions::Right) => write!(f, "MR"),
            Actions::Move(Directions::Up) => write!(f, "MU"),
            Actions::Move(Directions::Down) => write!(f, "MD"),
            Actions::Push(Directions::Left) => write!(f, "PL"),
            Actions::Push(Directions::Right) => write!(f, "PR"),
            Actions::Push(Directions::Up) => write!(f, "PU"),
            Actions::Push(Directions::Down) => write!(f, "PD"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameBoard {
    map: Vec<Vec<Entities>>,
    player: (usize, usize),
    pub targets: Vec<(usize, usize)>,
    pub boxes: Vec<(usize, usize)>,
    pub actions: Vec<Actions>,
}

impl GameBoard {
    pub fn from_src(source: &str) -> GameBoard {
        let mut lines = source.split('\n');
        let to_vec = |src: &str| {
            src.split_whitespace()
                .map(|i| i.parse().unwrap())
                .collect_vec()
        };
        let first_line = to_vec(lines.next().unwrap());
        let boxes = to_vec(lines.next().unwrap()).into_iter().tuples().collect();
        let map: Vec<Vec<_>> = lines
            .map(|line| {
                line.split_whitespace()
                    .map(|i| match i {
                        "0" => Entities::Wall,
                        "1" => Entities::Road,
                        "2" => Entities::Target,
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect();

        let mut targets = vec![];
        for y in 0..map.len() {
            for x in 0..map[0].len() {
                if matches!(map[y][x], Entities::Target) {
                    targets.push((x, y));
                }
            }
        }

        GameBoard {
            map,
            boxes,
            targets,
            player: (first_line[0], first_line[1]),
            actions: vec![],
        }
    }

    fn stringify(&self) -> String {
        let mut result = vec![];
        for y in 0..self.height() {
            let mut line = vec![];
            for x in 0..self.width() {
                line.push(match self.map[y][x] {
                    Entities::Wall => "⬛",
                    _ if self.player == (x, y) => "🐱",
                    Entities::Road => {
                        if self.boxes.contains(&(x, y)) {
                            "🔴"
                        } else {
                            "  "
                        }
                    }
                    Entities::Target => {
                        if self.boxes.contains(&(x, y)) {
                            "🔘"
                        } else {
                            "[]"
                        }
                    }
                });
            }
            result.push(line.join(""))
        }
        result.join("\n\r")
    }

    fn width(&self) -> usize {
        self.map[0].len()
    }

    fn height(&self) -> usize {
        self.map.len()
    }

    fn is_free(&self, x: usize, y: usize) -> bool {
        !matches!(self.map[y][x], Entities::Wall) && !self.boxes.contains(&(x, y))
    }

    fn set_player_position(&mut self, x: usize, y: usize) {
        self.player = (x, y);
    }

    fn move_box(&mut self, src: (usize, usize), dst: (usize, usize)) {
        let pos = self.boxes.iter().position(|&b| b == src).unwrap();
        let _ = std::mem::replace(&mut self.boxes[pos], dst);
    }

    fn _move(&mut self, direction: &Directions) -> Option<Actions> {
        let (px, py) = self.player;
        match direction {
            Directions::Left => {
                if px < 1 || matches!(self.map[py][px - 1], Entities::Wall) {
                    return None;
                }
                if !self.boxes.contains(&(px - 1, py)) {
                    self.set_player_position(px - 1, py);
                    return Some(Actions::Move(*direction));
                }
                if px < 2 || !self.is_free(px - 2, py) {
                    return None;
                }
                self.set_player_position(px - 1, py);
                self.move_box((px - 1, py), (px - 2, py));
                Some(Actions::Push(*direction))
            }
            Directions::Right => {
                if px + 1 >= self.width() || matches!(self.map[py][px + 1], Entities::Wall) {
                    return None;
                }
                if !self.boxes.contains(&(px + 1, py)) {
                    self.set_player_position(px + 1, py);
                    return Some(Actions::Move(*direction));
                }
                if px + 2 >= self.width() || !self.is_free(px + 2, py) {
                    return None;
                }
                self.set_player_position(px + 1, py);
                self.move_box((px + 1, py), (px + 2, py));
                Some(Actions::Push(*direction))
            }
            Directions::Up => {
                if py < 1 || matches!(self.map[py - 1][px], Entities::Wall) {
                    return None;
                }
                if !self.boxes.contains(&(px, py - 1)) {
                    self.set_player_position(px, py - 1);
                    return Some(Actions::Move(*direction));
                }
                if py < 2 || !self.is_free(px, py - 2) {
                    return None;
                }
                self.set_player_position(px, py - 1);
                self.move_box((px, py - 1), (px, py - 2));
                Some(Actions::Push(*direction))
            }
            Directions::Down => {
                if py + 1 >= self.height() || matches!(self.map[py + 1][px], Entities::Wall) {
                    return None;
                }
                if !self.boxes.contains(&(px, py + 1)) {
                    self.set_player_position(px, py + 1);
                    return Some(Actions::Move(*direction));
                }
                if py + 2 >= self.height() || !self.is_free(px, py + 2) {
                    return None;
                }
                self.set_player_position(px, py + 1);
                self.move_box((px, py + 1), (px, py + 2));
                Some(Actions::Push(*direction))
            }
        }
    }

    pub fn is_win(&self) -> bool {
        self.boxes
            .iter()
            .all(|&(x, y)| matches!(self.map[y][x], Entities::Target))
    }

    pub fn move_(&mut self, direction: &Directions) -> Option<Actions> {
        let result = self._move(direction);
        if let Some(res) = result {
            self.actions.push(res);
        }
        result
    }

    pub fn render(&self) {
        let mut stdout = std::io::stdout();
        enable_raw_mode().unwrap();
        execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();
        println!("{}", self.stringify());
        println!(
            "\rHistory: {}\r",
            self.actions.iter().map(|i| i.to_string()).join(" ")
        );
        println!("\rSteps: {}", self.actions.len());
        disable_raw_mode().unwrap();
    }

    pub fn undo(&mut self) {
        if let Some(act) = self.actions.pop() {
            let (px, py) = self.player;
            match act {
                Actions::Move(Directions::Left) => self.set_player_position(px + 1, py),
                Actions::Move(Directions::Right) => self.set_player_position(px - 1, py),
                Actions::Move(Directions::Up) => self.set_player_position(px, py + 1),
                Actions::Move(Directions::Down) => self.set_player_position(px, py - 1),
                Actions::Push(Directions::Left) => {
                    self.set_player_position(px + 1, py);
                    self.move_box((px - 1, py), (px, py));
                }
                Actions::Push(Directions::Right) => {
                    self.set_player_position(px - 1, py);
                    self.move_box((px + 1, py), (px, py));
                }
                Actions::Push(Directions::Up) => {
                    self.set_player_position(px, py + 1);
                    self.move_box((px, py - 1), (px, py));
                }
                Actions::Push(Directions::Down) => {
                    self.set_player_position(px, py - 1);
                    self.move_box((px, py + 1), (px, py));
                }
            }
        }
    }

    // pub fn targets(&self) -> Vec<(usize, usize)> {
    //     let mut result = vec![];
    //     for y in 0..self.height() {
    //         for x in 0..self.width() {
    //             if matches!(self.map[y][x], Entities::Target) {
    //                 result.push((x, y));
    //             }
    //         }
    //     }
    //     result
    // }
}
