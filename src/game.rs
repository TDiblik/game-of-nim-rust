use ratatui::layout::Constraint;

pub struct Game {
    pub matches: Vec<Vec<bool>>,

    pub player_1_number_of_matches: usize,
    pub player_2_number_of_matches: usize,

    pub matches_number_of_rows: usize,
    pub matches_vertical_container_constraints: Vec<Constraint>,

    pub pointing_to_match: PointerToSelected,
    pub current_player: PossiblePlayers,
}

impl Game {
    pub fn new(number_of_rows: u8) -> Self {
        let number_of_rows = number_of_rows as usize;
        let mut new = Self {
            matches: Vec::with_capacity(number_of_rows),
            player_1_number_of_matches: 0,
            player_2_number_of_matches: 0,
            matches_number_of_rows: number_of_rows,
            matches_vertical_container_constraints: Vec::with_capacity(number_of_rows + 2),
            pointing_to_match: PointerToSelected { row: 0, column: 0 },
            current_player: PossiblePlayers::Player1,
        };

        new.matches_vertical_container_constraints
            .push(Constraint::Percentage(5));
        let constraint = 90 / number_of_rows;
        for _i in 0..number_of_rows {
            new.matches_vertical_container_constraints
                .push(Constraint::Percentage(constraint as u16));
        }
        new.matches_vertical_container_constraints
            .push(Constraint::Percentage(5));

        for i in 0..number_of_rows {
            let number_of_matches = 1 + i * 2;
            let matches_row = vec![true; number_of_matches];
            new.matches.push(matches_row);
        }

        new
    }

    pub fn make_move(&mut self, move_to_make: PossibleMoves) {
        let currentlly_selected_row = &mut self.matches[self.pointing_to_match.row];
        match move_to_make {
            PossibleMoves::Up => {
                if self.pointing_to_match.row > 0 {
                    self.pointing_to_match.row -= 1;
                }
            }
            PossibleMoves::Down => {
                if self.pointing_to_match.row < self.matches_number_of_rows - 1 {
                    self.pointing_to_match.row += 1;
                }
            }
            PossibleMoves::Right => {
                if self.pointing_to_match.column < currentlly_selected_row.len() - 1 {
                    self.pointing_to_match.column += 1;
                }
            }
            PossibleMoves::Left => {
                if self.pointing_to_match.column > 0 {
                    self.pointing_to_match.column -= 1;
                }
            }
            PossibleMoves::Select => {
                currentlly_selected_row[self.pointing_to_match.column] = false;
                match self.current_player {
                    PossiblePlayers::Player1 => self.player_1_number_of_matches += 1,
                    PossiblePlayers::Player2 => self.player_2_number_of_matches += 1,
                }
            }
        };
    }

    pub fn next_player(&mut self) {
        self.current_player = match self.current_player {
            PossiblePlayers::Player1 => PossiblePlayers::Player2,
            PossiblePlayers::Player2 => PossiblePlayers::Player1,
        }
    }
}

pub struct PointerToSelected {
    pub row: usize,
    pub column: usize,
}

pub enum PossibleMoves {
    Up,
    Down,
    Right,
    Left,
    Select,
}

#[derive(PartialEq)]
pub enum PossiblePlayers {
    Player1,
    Player2,
}
