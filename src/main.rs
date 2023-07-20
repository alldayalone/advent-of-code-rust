#[macro_use]
extern crate lazy_static;

// use rand::thread_rng;
// use rand::seq::SliceRandom;

use std::fs;

#[derive(Clone, Debug)]
struct Field {
  data: Vec<Vec<char>>,
  width: usize,
  height: usize,
  start_pos: Position,
  final_pos: Position
}

impl Field {
  fn parse_input(file: &str) -> Field {
    let input = fs::read_to_string(file).expect("Input file exists");
    let data: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let width = data[0].len();
    let height = data.len();

    Field {
      data,
      width,
      height,
      start_pos: Position { x: 1, y: 0 },
      final_pos: Position { x: width - 2, y: height - 1 }
    }
  }
}

#[derive(Clone, Debug)]
enum Direction {
  North,
  South, 
  East,
  West
}

#[derive(PartialEq, Clone, Copy, Debug)]
struct Position {
  x: usize,
  y: usize,
}

impl Position {
  fn distance(&self, other: &Position) -> usize {
    (self.x as isize - other.x as isize).abs() as usize + (self.y as isize - other.y as isize).abs() as usize
  }
}

#[derive(Clone, Debug)]
struct Blizzard {
  position: Position,
  direction: Direction
}

impl Blizzard {
  fn get_next_square(&self) -> Position {
    let mut next_square = self.position.clone();
    match self.direction {
      Direction::North => next_square.y -= 1,
      Direction::South => next_square.y += 1,
      Direction::East => next_square.x += 1,
      Direction::West => next_square.x -= 1
    }

    if next_square.x <= 0 {
      next_square.x = FIELD.width - 2;
    }

    if next_square.x >= FIELD.width - 1 {
      next_square.x = 1;
    }

    if next_square.y <= 0 {
      next_square.y = FIELD.height - 2;
    }

    if next_square.y >= FIELD.height - 1 {
      next_square.y = 1;
    }

    next_square 
  }
}

#[derive(Clone, Debug)]
struct State {
  minutes: usize,
  blizzards: Vec<Blizzard>,
  expedition: Position
}

impl State {
  fn initial(field: &Field) -> State {
    State {
      minutes: 0,
      blizzards: field.data.iter().enumerate().flat_map(|row| {
        row.1.iter().enumerate().filter_map(|c| {
          match c.1 {
            '^' => Some(Blizzard { position: Position { x: c.0, y: row.0 }, direction: Direction::North }),
            'v' => Some(Blizzard { position: Position { x: c.0, y: row.0 }, direction: Direction::South }),
            '<' => Some(Blizzard { position: Position { x: c.0, y: row.0 }, direction: Direction::West }),
            '>' => Some(Blizzard { position: Position { x: c.0, y: row.0 }, direction: Direction::East }),
            _ => None
          }
        }).collect::<Vec<_>>()
      }).collect(),
      expedition: Position {
        x: field.data[0].iter().position(|c| c == &'.').unwrap(),
        y: 0
      }
    }
  }

  fn is_worse(&self, other: &State) -> bool {
    self.expedition.distance(&FIELD.final_pos) + self.minutes > other.minutes
  }

  fn is_better(&self, other: &State) -> bool {
    self.minutes < other.minutes
  }

  fn is_finished(&self) -> bool {
    self.expedition.distance(&FIELD.final_pos) == 0
  }

  fn display_field(&self) {
    for y in 0..FIELD.height {
      for x in 0..FIELD.width {
        let mut found = false;
        for b in &self.blizzards {
          if b.position.x == x && b.position.y == y {
            print!("{}", match b.direction {
              Direction::North => '^',
              Direction::South => 'v',
              Direction::East => '>',
              Direction::West => '<'
            });
            found = true;
            break;
          }
        }

        if !found {
          if x == self.expedition.x && y == self.expedition.y {
            print!("E");
          } else if x == FIELD.final_pos.x && y == FIELD.final_pos.y {
            print!(".");
          } else if x == 1 && y == 0 {
            print!(".");
          } else if x == 0 || y == 0 || x == FIELD.width - 1 || y == FIELD.height - 1 {
            print!("#");
          } else {
            print!(".");
          }
        }
      }
      println!();
    }
  }
}

fn iterate(state: &State, best_state: &mut State) {
  // println!("State: {:#?}", state.expedition);
  // state.display_field();
  
  // bound worse
  if state.is_worse(best_state) {
    return;
  }
  
  if state.is_finished() {
    if state.is_better(best_state) {
      println!("Best state: {:#?}, {}, {:?}", state.minutes, best_state.minutes, state.expedition);
      *best_state = state.clone();
    }
    return;
  }

  let blizzards = state.blizzards.iter().map(|b| Blizzard { position: b.get_next_square(), direction: b.direction.clone() }).collect::<Vec<_>>();

  let candidate_moves: Vec<Position> = [
    Some(Position { x: state.expedition.x + 1, y: state.expedition.y}),
    Some(Position { x: state.expedition.x, y: state.expedition.y + 1}),
    Some(Position { x: state.expedition.x, y: state.expedition.y}),
    (if state.expedition.y > 0 { Some(Position { x: state.expedition.x, y: state.expedition.y - 1}) } else { None }),
    Some(Position { x: state.expedition.x - 1, y: state.expedition.y}),
  ].into_iter().filter_map(|pos| pos).filter(|pos| {
    if pos == &FIELD.final_pos || pos == &FIELD.start_pos {
      return true
    }

    if pos.x == 0 || pos.x == FIELD.width - 1 || pos.y == 0 || pos.y == FIELD.height - 1 {
      return false;
    }

    true
  }).map(|ipos|
    Position { x: ipos.x as usize, y: ipos.y as usize }
  ).filter(|pos| {
    if blizzards.iter().find(|b| b.position.x == pos.x && b.position.y == pos.y).is_some() {
      return false;
    }

    true
  }).collect();

  // candidate_moves.shuffle(&mut thread_rng());
  
  candidate_moves.into_iter().for_each(|pos| {
    let new_state = State {
      minutes: state.minutes + 1,
      blizzards: blizzards.clone(),
      expedition: pos
    };

    iterate(&new_state, best_state);
  });
}

lazy_static! {
  static ref FIELD: Field = Field::parse_input("src/input24.txt");
}

fn main() {
  let initial_state = State::initial(&FIELD);
  let mut best_state = initial_state.clone();
  
  best_state.minutes = 1000;

  iterate(&initial_state, &mut best_state);

  println!("Best state minutes: {}", best_state.minutes);
}