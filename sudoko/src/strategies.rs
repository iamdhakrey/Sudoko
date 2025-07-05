use crate::sudoku::Sudoku;

pub trait SolvingStrategy {
    fn apply(&self, sudoku: &mut Sudoku) -> bool;
    fn name(&self) -> &'static str;
}

/// Naked Singles: If a cell has only one possible candidate, fill it
pub struct NakedSingles;

impl SolvingStrategy for NakedSingles {
    fn apply(&self, sudoku: &mut Sudoku) -> bool {
        let mut progress = false;
        
        for row in 0..sudoku.size {
            for col in 0..sudoku.size {
                if sudoku.grid[row][col].is_empty() {
                    let candidates = sudoku.get_candidates(row, col);
                    if candidates.len() == 1 {
                        let value = *candidates.iter().next().unwrap();
                        sudoku.set(row, col, value).unwrap();
                        progress = true;
                    }
                }
            }
        }
        
        progress
    }

    fn name(&self) -> &'static str {
        "Naked Singles"
    }
}

/// Hidden Singles: If a value can only go in one cell in a unit (row, column, or box)
pub struct HiddenSingles;

impl SolvingStrategy for HiddenSingles {
    fn apply(&self, sudoku: &mut Sudoku) -> bool {
        let mut progress = false;
        
        // Check rows
        for row in 0..sudoku.size {
            progress |= self.apply_to_row(sudoku, row);
        }
        
        // Check columns
        for col in 0..sudoku.size {
            progress |= self.apply_to_col(sudoku, col);
        }
        
        // Check boxes
        for box_row in 0..sudoku.box_size {
            for box_col in 0..sudoku.box_size {
                progress |= self.apply_to_box(sudoku, box_row, box_col);
            }
        }
        
        progress
    }

    fn name(&self) -> &'static str {
        "Hidden Singles"
    }
}

impl HiddenSingles {
    fn apply_to_row(&self, sudoku: &mut Sudoku, row: usize) -> bool {
        let mut progress = false;
        
        for value in 1..=sudoku.size as u8 {
            let mut possible_cells = Vec::new();
            
            for col in 0..sudoku.size {
                if sudoku.grid[row][col].is_empty() {
                    let candidates = sudoku.get_candidates(row, col);
                    if candidates.contains(&value) {
                        possible_cells.push(col);
                    }
                }
            }
            
            if possible_cells.len() == 1 {
                let col = possible_cells[0];
                sudoku.set(row, col, value).unwrap();
                progress = true;
            }
        }
        
        progress
    }
    
    fn apply_to_col(&self, sudoku: &mut Sudoku, col: usize) -> bool {
        let mut progress = false;
        
        for value in 1..=sudoku.size as u8 {
            let mut possible_cells = Vec::new();
            
            for row in 0..sudoku.size {
                if sudoku.grid[row][col].is_empty() {
                    let candidates = sudoku.get_candidates(row, col);
                    if candidates.contains(&value) {
                        possible_cells.push(row);
                    }
                }
            }
            
            if possible_cells.len() == 1 {
                let row = possible_cells[0];
                sudoku.set(row, col, value).unwrap();
                progress = true;
            }
        }
        
        progress
    }
    
    fn apply_to_box(&self, sudoku: &mut Sudoku, box_row: usize, box_col: usize) -> bool {
        let mut progress = false;
        
        for value in 1..=sudoku.size as u8 {
            let mut possible_cells = Vec::new();
            
            for row in box_row * sudoku.box_size..(box_row + 1) * sudoku.box_size {
                for col in box_col * sudoku.box_size..(box_col + 1) * sudoku.box_size {
                    if sudoku.grid[row][col].is_empty() {
                        let candidates = sudoku.get_candidates(row, col);
                        if candidates.contains(&value) {
                            possible_cells.push((row, col));
                        }
                    }
                }
            }
            
            if possible_cells.len() == 1 {
                let (row, col) = possible_cells[0];
                sudoku.set(row, col, value).unwrap();
                progress = true;
            }
        }
        
        progress
    }
}

/// Naked Pairs: If two cells in a unit have the same two candidates, eliminate those from other cells
pub struct NakedPairs;

impl SolvingStrategy for NakedPairs {
    fn apply(&self, _sudoku: &mut Sudoku) -> bool {
        // This is a more complex strategy that would modify candidate lists
        // For now, we'll implement a simplified version
        false
    }

    fn name(&self) -> &'static str {
        "Naked Pairs"
    }
}

/// Pointing Pairs/Triples: If all candidates for a value in a box are in the same row/column
pub struct PointingPairs;

impl SolvingStrategy for PointingPairs {
    fn apply(&self, _sudoku: &mut Sudoku) -> bool {
        // This is a candidate elimination strategy
        // For simplicity, we'll implement basic logic
        false
    }

    fn name(&self) -> &'static str {
        "Pointing Pairs"
    }
}

/// Box/Line Reduction: If all candidates for a value in a row/column are in the same box
pub struct BoxLineReduction;

impl SolvingStrategy for BoxLineReduction {
    fn apply(&self, _sudoku: &mut Sudoku) -> bool {
        // This is a candidate elimination strategy
        false
    }

    fn name(&self) -> &'static str {
        "Box/Line Reduction"
    }
}

/// X-Wing: Advanced pattern recognition strategy
pub struct XWing;

impl SolvingStrategy for XWing {
    fn apply(&self, _sudoku: &mut Sudoku) -> bool {
        // Advanced strategy - simplified for now
        false
    }

    fn name(&self) -> &'static str {
        "X-Wing"
    }
}

/// Swordfish: Even more advanced pattern recognition
pub struct Swordfish;

impl SolvingStrategy for Swordfish {
    fn apply(&self, _sudoku: &mut Sudoku) -> bool {
        // Very advanced strategy - simplified for now
        false
    }

    fn name(&self) -> &'static str {
        "Swordfish"
    }
}

pub fn get_all_strategies() -> Vec<Box<dyn SolvingStrategy>> {
    vec![
        Box::new(NakedSingles),
        Box::new(HiddenSingles),
        Box::new(NakedPairs),
        Box::new(PointingPairs),
        Box::new(BoxLineReduction),
        Box::new(XWing),
        Box::new(Swordfish),
    ]
}
