use ratatui::layout::Constraint;

pub struct Game {
    pub matches: Vec<Vec<bool>>,

    pub player_1_number_of_matches: usize,
    pub player_2_number_of_matches: usize,

    pub matches_number_of_rows: usize,
    pub matches_vertical_container_constraints: Vec<Constraint>,

    pub pointing_to_match: PointerToSelected,
    pub current_player: PossiblePlayers,
    pub last_took_from_row: Option<usize>,

    pub is_finished: bool,
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
            last_took_from_row: None,
            is_finished: false,
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
                let mut next_to_try = self.pointing_to_match.row;
                loop {
                    if next_to_try == 0 {
                        break;
                    }
                    if self.matches[next_to_try - 1].iter().any(|s| *s) {
                        self.pointing_to_match.row = next_to_try - 1;
                        break;
                    }
                    next_to_try -= 1;
                }

                let currentlly_selected_row = &self.matches[self.pointing_to_match.row];
                let last_possible_index = currentlly_selected_row.len() - 1;
                if self.pointing_to_match.column > last_possible_index {
                    self.pointing_to_match.column = last_possible_index;
                }
                self.make_move(PossibleMoves::Right);
                self.make_move(PossibleMoves::Left);
            }
            PossibleMoves::Down => {
                let mut next_to_try = self.pointing_to_match.row;
                loop {
                    if next_to_try == self.matches_number_of_rows - 1 {
                        break;
                    }
                    if self.matches[next_to_try + 1].iter().any(|s| *s) {
                        self.pointing_to_match.row = next_to_try + 1;
                        break;
                    }
                    next_to_try += 1;
                }

                self.make_move(PossibleMoves::Right);
                self.make_move(PossibleMoves::Left);
            }
            PossibleMoves::Right => {
                let mut next_to_try = self.pointing_to_match.column;
                loop {
                    if next_to_try == currentlly_selected_row.len() - 1 {
                        break;
                    }
                    if currentlly_selected_row[next_to_try + 1] {
                        self.pointing_to_match.column = next_to_try + 1;
                        break;
                    }
                    next_to_try += 1;
                }
            }
            PossibleMoves::Left => {
                let mut next_to_try = self.pointing_to_match.column;
                loop {
                    if next_to_try == 0 {
                        break;
                    }
                    if currentlly_selected_row[next_to_try - 1] {
                        self.pointing_to_match.column = next_to_try - 1;
                        break;
                    }
                    next_to_try -= 1;
                }
            }
            PossibleMoves::Select => {
                if self.last_took_from_row.is_none() {
                    self.last_took_from_row = Some(self.pointing_to_match.row);
                }
                if self.pointing_to_match.row != self.last_took_from_row.unwrap() {
                    return;
                }

                currentlly_selected_row[self.pointing_to_match.column] = false;
                match self.current_player {
                    PossiblePlayers::Player1 => self.player_1_number_of_matches += 1,
                    PossiblePlayers::Player2 => self.player_2_number_of_matches += 1,
                }
                let previous_pointing_state = self.pointing_to_match.clone();

                self.make_move(PossibleMoves::Left);
                if previous_pointing_state != self.pointing_to_match {
                    return;
                }

                self.make_move(PossibleMoves::Right);
                if previous_pointing_state != self.pointing_to_match {
                    return;
                }

                self.make_move(PossibleMoves::Down);
                if previous_pointing_state != self.pointing_to_match {
                    return;
                }

                self.make_move(PossibleMoves::Up);
            }
        };
    }

    pub fn check_win_conditions(&mut self) {
        let mut remaining = 0;
        for s in self.matches.iter() {
            for k in s {
                if *k {
                    remaining += 1;
                }
            }
        }

        if remaining == 1 {
            self.is_finished = true;
        }
    }

    pub fn next_player(&mut self) {
        self.last_took_from_row = None;
        self.current_player = match self.current_player {
            PossiblePlayers::Player1 => PossiblePlayers::Player2,
            PossiblePlayers::Player2 => PossiblePlayers::Player1,
        }
    }
}

#[derive(PartialEq, Clone)]
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
