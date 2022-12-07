use log::debug;

use cursive::{
    event::{EventResult, Key},
    theme::{BaseColor, Color, ColorStyle},
    view::CannotFocus,
};
use enum_iterator::{all, next_cycle, previous_cycle, Sequence};

use rand::seq::IteratorRandom;

use std::{
    process::exit,
    time::{Duration, Instant},
};

const NUM_ROWS: isize = 40;
const NUM_VISIBLE_ROWS: isize = 20;
const NUM_COLUMNS: isize = 10;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PieceState {
    T,
    O,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CellState {
    X(Color), // filled
    T(Color), // active tetronimo
    O,        // empty
}

#[derive(Copy, Clone)]
enum RotateKind {
    Clockwise,
    Counterclockwise,
}

#[derive(Sequence, Debug, Copy, Clone)]
enum Rotation {
    R0,
    R1,
    R2,
    R3,
}

enum Move {
    Left,
    Right,
    Down,
}

impl Rotation {
    fn rotate(&self, kind: RotateKind) -> Rotation {
        match kind {
            // why do I need to unwrap these?
            RotateKind::Clockwise => next_cycle(self).unwrap(),
            RotateKind::Counterclockwise => previous_cycle(self).unwrap(),
        }
    }
}

#[derive(PartialEq)]
pub enum TickResult {
    GameOver,
    Continue,
}

use Rotation::*;

#[derive(Clone, Copy, Debug, Sequence)]
enum TetronimoKind {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

#[derive(Debug, Copy, Clone)]
struct Position(isize, isize);

use PieceState::*;

impl TetronimoKind {
    fn random() -> TetronimoKind {
        let mut rng = rand::thread_rng();
        all().choose(&mut rng).unwrap()
    }

    fn color(&self) -> Color {
        match self {
            Self::I => Color::Rgb(3, 65, 174),
            Self::J => Color::Rgb(114, 203, 59),
            Self::L => Color::Rgb(255, 213, 0),
            Self::O => Color::Rgb(255, 151, 28),
            Self::S => Color::Rgb(255, 50, 19),
            Self::T => Color::Rgb(128, 0, 128),
            Self::Z => Color::Rgb(255, 127, 0),
        }
    }

    // TODO: make this not allocate...
    // source: https://tetris.fandom.com/wiki/SRS
    fn cells(&self, rotation: &Rotation) -> Vec<Vec<PieceState>> {
        match (self, rotation) {
            (Self::I, R0) => {
                vec![
                    vec![O, O, O, O],
                    vec![T, T, T, T],
                    vec![O, O, O, O],
                    vec![O, O, O, O],
                ]
            }
            (Self::I, R1) => {
                vec![
                    vec![O, O, T, O],
                    vec![O, O, T, O],
                    vec![O, O, T, O],
                    vec![O, O, T, O],
                ]
            }
            (Self::I, R2) => {
                vec![
                    vec![O, O, O, O],
                    vec![O, O, O, O],
                    vec![T, T, T, T],
                    vec![O, O, O, O],
                ]
            }
            (Self::I, R3) => {
                vec![
                    vec![O, T, O, O],
                    vec![O, T, O, O],
                    vec![O, T, O, O],
                    vec![O, T, O, O],
                ]
            }
            (Self::J, R0) => {
                vec![
                    vec![T, O, O], // fmt
                    vec![T, T, T],
                    vec![O, O, O],
                ]
            }
            (Self::J, R1) => {
                vec![
                    vec![O, T, T], // fmt
                    vec![O, T, O],
                    vec![O, T, O],
                ]
            }
            (Self::J, R2) => {
                vec![
                    vec![O, O, O], // fmt
                    vec![T, T, T],
                    vec![O, O, T],
                ]
            }
            (Self::J, R3) => {
                vec![
                    vec![O, T, O], // fmt
                    vec![O, T, O],
                    vec![T, T, O],
                ]
            }
            (Self::L, R0) => {
                vec![
                    vec![O, O, T], // fmt
                    vec![T, T, T],
                    vec![O, O, O],
                ]
            }
            (Self::L, R1) => {
                vec![
                    vec![O, T, O], // fmt
                    vec![O, T, O],
                    vec![O, T, T],
                ]
            }
            (Self::L, R2) => {
                vec![
                    vec![O, O, O], // fmt
                    vec![T, T, T],
                    vec![T, O, O],
                ]
            }
            (Self::L, R3) => {
                vec![
                    vec![T, T, O], // fmt
                    vec![O, T, O],
                    vec![O, T, O],
                ]
            }
            (Self::O, R0 | R1 | R2 | R3) => {
                vec![
                    vec![O, T, T, O],
                    vec![O, T, T, O],
                    vec![O, O, O, O],
                    vec![O, O, O, O],
                ]
            }
            (Self::S, R0) => {
                vec![
                    vec![O, T, T], // fmt
                    vec![T, T, O],
                    vec![O, O, O],
                ]
            }
            (Self::S, R1) => {
                vec![
                    vec![O, T, O], // fmt
                    vec![O, T, T],
                    vec![O, O, T],
                ]
            }
            (Self::S, R2) => {
                vec![
                    vec![O, O, O], // fmt
                    vec![O, T, T],
                    vec![T, T, O],
                ]
            }
            (Self::S, R3) => {
                vec![
                    vec![T, O, O], // fmt
                    vec![T, T, O],
                    vec![O, T, O],
                ]
            }
            (Self::T, R0) => {
                vec![
                    vec![O, T, O], // fmt
                    vec![T, T, T],
                    vec![O, O, O],
                ]
            }
            (Self::T, R1) => {
                vec![
                    vec![O, T, O], // fmt
                    vec![O, T, T],
                    vec![O, T, O],
                ]
            }
            (Self::T, R2) => {
                vec![
                    vec![O, O, O], // fmt
                    vec![T, T, T],
                    vec![O, T, O],
                ]
            }
            (Self::T, R3) => {
                vec![
                    vec![O, T, O], // fmt
                    vec![T, T, O],
                    vec![O, T, O],
                ]
            }
            (Self::Z, R0) => {
                vec![
                    vec![T, T, O], // fmt
                    vec![O, T, T],
                    vec![O, O, O],
                ]
            }
            (Self::Z, R1) => {
                vec![
                    vec![O, O, T], // fmt
                    vec![O, T, T],
                    vec![O, T, O],
                ]
            }
            (Self::Z, R2) => {
                vec![
                    vec![O, O, O], // fmt
                    vec![T, T, O],
                    vec![O, T, T],
                ]
            }
            (Self::Z, R3) => {
                vec![
                    vec![O, T, O], // fmt
                    vec![T, T, O],
                    vec![T, O, O],
                ]
            }
        }
    }

    fn start_position(&self) -> Position {
        Position(20, 4)
    }
}

#[derive(Debug, Copy, Clone)]
struct Tetronimo {
    kind: TetronimoKind,
    pos: Position,
    rotation: Rotation,
}

#[derive(Debug)]
pub struct Game {
    // indexing: (0, 0) is top left
    cells: [[CellState; NUM_COLUMNS as usize]; NUM_ROWS as usize],
    active_tet: Option<Tetronimo>,
    last_tick_time: Instant,
    tick_interval: Duration,
}

impl Game {
    pub fn new() -> Self {
        Self {
            cells: [[CellState::O; NUM_COLUMNS as usize]; NUM_ROWS as usize],
            active_tet: None,
            last_tick_time: Instant::now(),
            tick_interval: Duration::from_millis(200),
        }
    }

    fn can_place(
        &self,
        Tetronimo {
            kind,
            rotation,
            pos: Position(x, y),
        }: &Tetronimo,
    ) -> bool {
        let piece_cells = kind.cells(rotation);
        for (i, row) in piece_cells.iter().enumerate() {
            for (j, piece_state) in row.iter().enumerate() {
                if let PieceState::T = piece_state {
                    let cell_x = *x + i as isize;
                    let cell_y = *y + j as isize;

                    if cell_x < 0 || cell_x >= NUM_ROWS || cell_y < 0 || cell_y >= NUM_COLUMNS {
                        return false;
                    }
                    if let CellState::X(_) = self.cells[cell_x as usize][cell_y as usize] {
                        return false;
                    };
                };
            }
        }
        true
    }

    fn rotate(&mut self, rotate_kind: RotateKind) {
        if let Some(tet @ Tetronimo { rotation, .. }) = self.active_tet {
            let new_rotation = rotation.rotate(rotate_kind);
            let next_tet = Tetronimo {
                rotation: new_rotation,
                ..tet
            };
            if self.can_place(&next_tet) {
                self.place(&next_tet);
            }
        }
    }

    fn move_(&mut self, move_: Move) {
        if let Some(tet @ Tetronimo { pos, .. }) = self.active_tet {
            let Position(x, y) = pos;

            let new_position = match move_ {
                Move::Left => Position(x, y - 1),
                Move::Right => Position(x, y + 1),
                Move::Down => Position(x + 1, y),
            };

            let next_tet = Tetronimo {
                pos: new_position,
                ..tet
            };
            if self.can_place(&next_tet) {
                self.place(&next_tet);
            }
        }
    }

    fn place(&mut self, tet: &Tetronimo) {
        // reset the tetromino pieces
        for row in self.cells.iter_mut() {
            for cell in row.iter_mut() {
                *cell = match *cell {
                    CellState::O | CellState::T(_) => CellState::O,
                    x @ CellState::X(_) => x,
                }
            }
        }

        let Tetronimo {
            kind,
            pos: Position(x, y),
            rotation,
        } = tet;

        let piece_cells = kind.cells(rotation);

        for (i, row) in piece_cells.iter().enumerate() {
            for (j, state) in row.iter().enumerate() {
                match state {
                    O => (),
                    T => {
                        // the pos the box can be negative, which is fine as long as all the nonempty cells
                        // are contained within the board. We need to be careful to avoid overflowing though
                        // in the case that x is negative.
                        let cell_x = (*x + (i as isize)) as usize;
                        let cell_y = (*y + (j as isize)) as usize;

                        let cell = &mut self.cells[cell_x][cell_y];
                        assert!(*cell == CellState::O);
                        *cell = CellState::T(kind.color());
                    }
                }
            }
        }

        self.active_tet = Some(*tet);
    }

    fn freeze_and_remove_completed_rows(&mut self) {
        // transition any cells in state T => X.
        for row in self.cells.iter_mut() {
            for cell in row.iter_mut() {
                *cell = match *cell {
                    CellState::O => CellState::O,
                    x @ CellState::X(_) => x,
                    CellState::T(color) => CellState::X(color),
                }
            }
        }

        let mut row_completed: [bool; NUM_ROWS as usize] = [false; NUM_ROWS as usize];

        for (row, cols) in self.cells.iter().enumerate() {
            // this seems gross.
            row_completed[row] = cols.iter().all(|&x| match x {
                CellState::X(_) => true,
                _ => false,
            });
        }

        // shift non-completed rows down-inplace,
        let mut new_row_idx = NUM_ROWS - 1;
        let mut old_row_idx = NUM_ROWS - 1;

        while old_row_idx >= 0 {
            log::info!(
                "old_row_idx: {:?}, new_row_idx: {:?}",
                old_row_idx,
                new_row_idx
            );
            if row_completed[old_row_idx as usize] {
                log::info!("row completed - old_row_idx {:?}", old_row_idx);
                // don't write out the completed row.
                old_row_idx = old_row_idx - 1;
            } else {
                if old_row_idx != new_row_idx {
                    let src = self.cells[old_row_idx as usize];
                    let dst = &mut self.cells[new_row_idx as usize];
                    dst.copy_from_slice(&src);
                }
                new_row_idx = new_row_idx - 1;
                old_row_idx = old_row_idx - 1;
            }
        }

        let empty_row: [CellState; NUM_COLUMNS as usize] = [CellState::O; NUM_COLUMNS as usize];
        if new_row_idx > 0 {
            for i in new_row_idx..=0 {
                log::info!("padding with empty rows {:?}", new_row_idx);
                let dst = &mut self.cells[new_row_idx as usize];
                dst.copy_from_slice(&empty_row);
            }
        }
    }

    pub fn tick(&mut self) -> TickResult {
        self.last_tick_time = Instant::now();

        match &self.active_tet {
            None => {
                let kind = TetronimoKind::random();
                let new_tet = Tetronimo {
                    kind,
                    pos: kind.start_position(),
                    rotation: R0,
                };

                if self.can_place(&new_tet) {
                    self.place(&new_tet);
                    self.active_tet = Some(new_tet);
                    TickResult::Continue
                } else {
                    TickResult::GameOver
                }
            }

            Some(active_tet @ Tetronimo { kind, pos, .. }) => {
                let Position(x, y) = pos;

                // TODO: reuse move_

                let next_pos = Position(*x + 1, *y);

                let next_tet = Tetronimo {
                    pos: next_pos,
                    ..*active_tet
                };

                if self.can_place(&next_tet) {
                    self.place(&next_tet);
                } else {
                    self.freeze_and_remove_completed_rows();
                    self.active_tet = None;
                }
                TickResult::Continue
            }
        }
    }

    pub fn maybe_tick(&mut self) {
        let now = Instant::now();
        let time_since_ticked = now - self.last_tick_time;
        if time_since_ticked > self.tick_interval {
            self.tick();
        }
    }
}

impl cursive::View for Game {
    fn layout(&mut self, _: cursive::Vec2) {
        self.maybe_tick()
    }

    fn draw(&self, printer: &cursive::Printer) {
        let num_non_visible = NUM_ROWS - NUM_VISIBLE_ROWS;
        for j in 0..(NUM_COLUMNS as usize) {
            for i in 0..(NUM_VISIBLE_ROWS as usize) {
                // we use different indexing for the game state than cursive,
                // cursive has (0,0) at top left but that's bottom left for the board,
                // so we need to convert.

                let xy = cursive::XY::new(j * 2, i);

                let color = match self.cells[i + num_non_visible as usize][j] {
                    CellState::O => Color::Dark(BaseColor::White),
                    CellState::X(color) | CellState::T(color) => color,
                };

                printer.with_color(
                    ColorStyle::new(Color::Dark(BaseColor::Black), color),
                    |printer| printer.print(xy, "  "),
                );
            }
        }
    }

    fn take_focus(&mut self, _: cursive::direction::Direction) -> Result<EventResult, CannotFocus> {
        Ok(EventResult::Consumed(None))
    }

    fn required_size(&mut self, _: cursive::Vec2) -> cursive::Vec2 {
        // why do we need this plus 1?
        cursive::Vec2::new((NUM_COLUMNS * 2) as usize, NUM_VISIBLE_ROWS as usize)
    }

    fn on_event(&mut self, event: cursive::event::Event) -> cursive::event::EventResult {
        match event {
            cursive::event::Event::Key(Key::Left) => {
                self.move_(Move::Left);
                EventResult::Consumed(None)
            }
            cursive::event::Event::Key(Key::Right) => {
                self.move_(Move::Right);
                EventResult::Consumed(None)
            }
            cursive::event::Event::Key(Key::Up) => {
                self.rotate(RotateKind::Counterclockwise);
                EventResult::Consumed(None)
            }
            cursive::event::Event::Key(Key::Down) => {
                self.move_(Move::Down);
                EventResult::Consumed(None)
            }
            _ => EventResult::Ignored,
        }
    }
}
