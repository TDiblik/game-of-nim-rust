use ratatui::layout::Constraint;

#[derive(Debug, Clone)]
pub struct Game {
    pub matches: Vec<Vec<bool>>,

    pub player_1_number_of_matches: usize,
    pub player_2_number_of_matches: usize,

    pub matches_number_of_rows: usize,
    pub matches_number_of_columns: usize,
    pub matches_vertical_container_constraints: Vec<Constraint>,
}

impl Game {
    pub fn new(number_of_rows: u8) -> Self {
        let number_of_rows = number_of_rows as usize;
        let mut new = Self {
            matches: Vec::with_capacity(number_of_rows),
            player_1_number_of_matches: 0,
            player_2_number_of_matches: 0,
            matches_number_of_rows: number_of_rows,
            matches_number_of_columns: 0,
            matches_vertical_container_constraints: Vec::with_capacity(number_of_rows + 2),
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
            new.matches_number_of_columns = number_of_matches;
        }

        new
    }
}
