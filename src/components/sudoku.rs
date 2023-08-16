use thiserror::Error;

#[derive(Debug)]
pub struct Sudoku {
    pub grid: [u8; 81],
    pub error: Option<Vec<u8>>,
    pub solved: bool,
}

impl Sudoku {
    fn row(&self, n: u8) -> Result<[u8; 9], ProcessingError> {
        if n < 1 || n > 9 {
            return Err(ProcessingError::Validation(
                "Row number must be between 1 and 9".to_string(),
            ));
        }
        let start = usize::from((n - 1) * 9); // 1 -> 0-9, 2 -> 9
        let end = usize::from(start + 9);
        Ok(self.grid[start..end].try_into().unwrap())
    }
}

impl Default for Sudoku {
    fn default() -> Self {
        Sudoku {
            grid: [0; 81],
            error: None,
            solved: false,
        }
    }
}

impl From<&str> for Sudoku {
    fn from(puzzle: &str) -> Self {
        let chars = puzzle.chars();
        if chars.clone().count() != 81 {
            return Sudoku::default();
        }

        let grid = chars
            .map(|c| {
                let mut i: u8 = c.to_digit(10).unwrap().try_into().unwrap();
                if i < 1 || i > 9 {
                    i = 0;
                }
                i
            })
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();

        Sudoku {
            grid,
            error: None,   // TODO check
            solved: false, // TODO check
        }
    }
}

#[derive(Error, Debug)]
pub enum ProcessingError {
    #[error("Puzzle validation error: {0}")]
    Validation(String),
    #[error("{0}")]
    Internal(String),
}
