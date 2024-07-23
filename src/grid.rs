const CELL_MASK_LEN: usize = 9;
const CELL_MASK: u128 = 0b111111111;

pub const BOXES: [[usize; 9]; 9] = [
    [0, 0, 0, 1, 1, 1, 2, 2, 2],
    [0, 0, 0, 1, 1, 1, 2, 2, 2],
    [0, 0, 0, 1, 1, 1, 2, 2, 2],
    [3, 3, 3, 4, 4, 4, 5, 5, 5],
    [3, 3, 3, 4, 4, 4, 5, 5, 5],
    [3, 3, 3, 4, 4, 4, 5, 5, 5],
    [6, 6, 6, 7, 7, 7, 8, 8, 8],
    [6, 6, 6, 7, 7, 7, 8, 8, 8],
    [6, 6, 6, 7, 7, 7, 8, 8, 8],
];

pub const BOXES_INVERSE: [[(usize, usize); 9]; 9] = [
    [(0, 0), (0, 1), (0, 2), (1, 0), (1, 1), (1, 2), (2, 0), (2, 1), (2, 2)],
    [(0, 3), (0, 4), (0, 5), (1, 3), (1, 4), (1, 5), (2, 3), (2, 4), (2, 5)],
    [(0, 6), (0, 7), (0, 8), (1, 6), (1, 7), (1, 8), (2, 6), (2, 7), (2, 8)],
    [(3, 0), (3, 1), (3, 2), (4, 0), (4, 1), (4, 2), (5, 0), (5, 1), (5, 2)],
    [(3, 3), (3, 4), (3, 5), (4, 3), (4, 4), (4, 5), (5, 3), (5, 4), (5, 5)],
    [(3, 6), (3, 7), (3, 8), (4, 6), (4, 7), (4, 8), (5, 6), (5, 7), (5, 8)],
    [(6, 0), (6, 1), (6, 2), (7, 0), (7, 1), (7, 2), (8, 0), (8, 1), (8, 2)],
    [(6, 3), (6, 4), (6, 5), (7, 3), (7, 4), (7, 5), (8, 3), (8, 4), (8, 5)],
    [(6, 6), (6, 7), (6, 8), (7, 6), (7, 7), (7, 8), (8, 6), (8, 7), (8, 8)],
];

pub const IN_BOXES_IDX: [[usize; 9]; 9] = [
    [0, 1, 2, 0, 1, 2, 0, 1, 2],
    [3, 4, 5, 3, 4, 5, 3, 4, 5],
    [6, 7, 8, 6, 7, 8, 6, 7, 8],
    [0, 1, 2, 0, 1, 2, 0, 1, 2],
    [3, 4, 5, 3, 4, 5, 3, 4, 5],
    [6, 7, 8, 6, 7, 8, 6, 7, 8],
    [0, 1, 2, 0, 1, 2, 0, 1, 2],
    [3, 4, 5, 3, 4, 5, 3, 4, 5],
    [6, 7, 8, 6, 7, 8, 6, 7, 8],
];

#[derive(Debug)]
pub struct Grid {
    // rows, cols, boxes are represented as a Array of 9 Bitfields
    // in which every 9 bits represent the presence of a number in a Cell there for only the first
    // 81 bits of each u128 are used
    rows: [u128; 9],
    cols: [u128; 9],
    boxes: [u128; 9],

    valid: bool,
    empty_cells: Vec<(usize, usize)>,
}

impl Grid {
    // ------------------- STATIC METHODS/CONSTRUCTORS -------------------

    pub fn new() -> Grid {
        Grid {
            rows: [0; 9],
            cols: [0; 9],
            boxes: [0; 9],

            valid: true,
            empty_cells: (0..81).map(Self::index_to_coords).collect(),
        }
    }

    // ------------------- PUBLIC METHODS -------------------

    pub fn load_from_str(&mut self, input: &str, null_chr: char) {
        if input.len() != 81 {
            panic!("Input string must be 81 characters long");
        }

        let digit_grid = input
            .chars()
            .map(|c| {
                if c == null_chr {
                    return 0;
                }
                c.to_digit(10).expect("Invalid character in board string.") as u8
            })
            .collect::<Vec<u8>>();

        // ---------- Reset the grid ----------
        self.rows = [0; 9];
        self.cols = [0; 9];
        self.boxes = [0; 9];
        self.valid = true;
        self.empty_cells = (0..81).map(Self::index_to_coords).collect();
        // ------------------------------------

        for i in 0..9 {
            for j in 0..9 {
                let value = digit_grid[i * 9 + j];
                if value != 0 {
                    self.set(i, j, value);
                }
            }
        }
    }

    pub fn get(&self, row: usize, col: usize) -> u8 {
        let cell = self.rows[row] >> (col * 9) & CELL_MASK;
        if cell == 0 {
            return 0;
        }
        cell.trailing_zeros() as u8 + 1
    }

    /// input is NOT checked for validity!
    pub fn set(&mut self, row: usize, col: usize, value: u8) {
        if value == 0 {
            return self.unset(row, col);
        }
        let mask = 1 << (value - 1);

        self.rows[row] |= mask << (col * CELL_MASK_LEN);
        self.cols[col] |= mask << (row * CELL_MASK_LEN);
        self.boxes[BOXES[row][col]] |= mask << (IN_BOXES_IDX[row][col] * CELL_MASK_LEN);

        self.valid = self.verify_cell(row, col);
        self.empty_cells.retain(|&(r, c)| r != row || c != col);
    }

    pub fn unset(&mut self, row: usize, col: usize) {
        todo!()
    }

    pub fn is_valid(&self) -> bool {
        self.valid
    }

    pub fn is_solved(&self) -> bool {
        self.empty_cells.is_empty() && self.is_valid()
    }

    // ------------------- PRIVATE METHODS -------------------

    /// This needs to be called after every set operation
    fn verify_cell(&self, row: usize, col: usize) -> bool {
        self.verify_row(row) && self.verify_col(col) && self.verify_box(BOXES[row][col])
    }

    fn verify_row(&self, row: usize) -> bool {
        Self::verify_block(self.rows[row])
    }

    fn verify_col(&self, col: usize) -> bool {
        Self::verify_block(self.cols[col])
    }

    fn verify_box(&self, box_idx: usize) -> bool {
        Self::verify_block(self.boxes[box_idx])
    }

    // ------------------- STATIC METHODS -------------------

    /// Verifies if the given block{row, col, box} is valid (no duplicates)
    /// A block does not have to be complete to be verified (i.e. it can contain empty cells)
    #[inline]
    fn verify_block(mut block: u128) -> bool {
        let mut mask = 0;
        for _ in 0..9 {
            let cell = block & CELL_MASK;
            if cell == 0 {
                block >>= CELL_MASK_LEN;
                continue;
            }
            if mask & cell != 0 {
                return false;
            }
            mask |= cell;
            block >>= CELL_MASK_LEN;
        }
        true
    }

    #[inline]
    pub fn index_to_coords(index: usize) -> (usize, usize) {
        (index / 9, index % 9)
    }
}

const TOP_ROW: &str = "╔═══╤═══╤═══╦═══╤═══╤═══╦═══╤═══╤═══╗";
const MIDDLE_ROW_DOUBLE: &str = "╠═══╪═══╪═══╬═══╪═══╪═══╬═══╪═══╪═══╣";
const MIDDLE_ROW_SINGLE: &str = "╟───┼───┼───╫───┼───┼───╫───┼───┼───╢";
const BOTTOM_ROW: &str = "╚═══╧═══╧═══╩═══╧═══╧═══╩═══╧═══╧═══╝";
const STRAIGHT_DOUBLE: &str = "║";
const STRAIGHT_SINGLE: &str = "│";
const EMPTY: &str = "   ";

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        result.push_str(TOP_ROW);
        result.push('\n');

        for i in 0..9 {
            result.push_str("║");
            for j in 0..9 {
                let value = self.get(i, j);
                if value == 0 {
                    result.push_str(EMPTY);
                } else {
                    result.push_str(&format!(" {} ", value));
                }

                if j == 2 || j == 5 {
                    result.push_str(STRAIGHT_DOUBLE);
                } else if j != 8 {
                    result.push_str(STRAIGHT_SINGLE);
                }
            }
            result.push_str(STRAIGHT_DOUBLE);
            result.push('\n');

            if i == 2 || i == 5 {
                result.push_str(MIDDLE_ROW_DOUBLE);
                result.push('\n');
            } else if i != 8 {
                result.push_str(MIDDLE_ROW_SINGLE);
                result.push('\n');
            }
        }

        result.push_str(BOTTOM_ROW);

        write!(f, "{}", result)
    }
}
