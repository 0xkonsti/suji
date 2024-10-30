use super::Backend;

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

#[derive(Debug, Clone)]
pub struct BitfieldGrid {
    // rows, cols, boxes are represented as a Array of 9 Bitfields
    // in which every 9 bits represent the presence of a number in a Cell there for only the first
    // 81 bits of each u128 are used
    rows: [u128; 9],
    cols: [u128; 9],
    boxes: [u128; 9],

    valid: bool,
    empty_cells: Vec<(usize, usize)>,
}

impl BitfieldGrid {
    pub fn new() -> Self {
        Self {
            rows: [0; 9],
            cols: [0; 9],
            boxes: [0; 9],

            valid: true,
            empty_cells: (0..81).map(Self::index_to_coords).collect(),
        }
    }

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

impl Backend for BitfieldGrid {
    fn reset(&mut self) {
        self.rows = [0; 9];
        self.cols = [0; 9];
        self.boxes = [0; 9];
        self.valid = true;
        self.empty_cells = (0..81).map(Self::index_to_coords).collect();
    }

    #[inline]
    fn get(&self, row: usize, col: usize) -> u8 {
        let cell = self.rows[row] >> (col * 9) & CELL_MASK;
        if cell == 0 {
            return 0;
        }
        cell.trailing_zeros() as u8 + 1
    }

    #[inline]
    fn set(&mut self, row: usize, col: usize, value: u8) {
        self.unset(row, col);
        if value == 0 {
            return;
        }
        let mask = 1 << (value - 1);

        self.rows[row] |= mask << (col * CELL_MASK_LEN);
        self.cols[col] |= mask << (row * CELL_MASK_LEN);
        self.boxes[BOXES[row][col]] |= mask << (IN_BOXES_IDX[row][col] * CELL_MASK_LEN);

        self.valid = self.verify_cell(row, col);
        self.empty_cells.retain(|&(r, c)| r != row || c != col);
    }

    #[inline]
    fn unset(&mut self, row: usize, col: usize) {
        let value = self.get(row, col);
        if value == 0 {
            return;
        }
        let mask = 1 << (value - 1);

        self.rows[row] &= !(mask << (col * CELL_MASK_LEN));
        self.cols[col] &= !(mask << (row * CELL_MASK_LEN));
        self.boxes[BOXES[row][col]] &= !(mask << (IN_BOXES_IDX[row][col] * CELL_MASK_LEN));

        self.valid = self.verify_cell(row, col);
        self.empty_cells.push((row, col));
    }

    #[inline]
    fn set_not_zero(&mut self, row: usize, col: usize, value: u8) {
        let mask = 1 << (value - 1);

        self.rows[row] |= mask << (col * CELL_MASK_LEN);
        self.cols[col] |= mask << (row * CELL_MASK_LEN);
        self.boxes[BOXES[row][col]] |= mask << (IN_BOXES_IDX[row][col] * CELL_MASK_LEN);

        self.valid = self.verify_cell(row, col);
        self.empty_cells.retain(|&(r, c)| r != row || c != col);
    }

    #[inline]
    fn set_not_zero_unckecked(&mut self, row: usize, col: usize, value: u8) {
        let mask = 1 << (value - 1);

        self.rows[row] |= mask << (col * CELL_MASK_LEN);
        self.cols[col] |= mask << (row * CELL_MASK_LEN);
        self.boxes[BOXES[row][col]] |= mask << (IN_BOXES_IDX[row][col] * CELL_MASK_LEN);

        self.empty_cells.retain(|&(r, c)| r != row || c != col);
    }

    #[inline]
    fn get_empty_cells(&self) -> &Vec<(usize, usize)> {
        &self.empty_cells
    }

    #[inline]
    fn is_valid(&self) -> bool {
        self.valid
    }

    #[inline]
    fn is_possible_value(&self, row: usize, col: usize, value: u8) -> bool {
        if self.get(row, col) != 0 {
            return false;
        }

        if value == 0 {
            return true;
        }

        let mask = 1 << (value - 1);
        let box_idx = BOXES[row][col];
        let in_box_idx = IN_BOXES_IDX[row][col];

        let new_row = self.rows[row] | mask << (col * CELL_MASK_LEN);
        let new_col = self.cols[col] | mask << (row * CELL_MASK_LEN);
        let new_box = self.boxes[box_idx] | mask << (in_box_idx * CELL_MASK_LEN);

        Self::verify_block(new_row) && Self::verify_block(new_col) && Self::verify_block(new_box)
    }
}
