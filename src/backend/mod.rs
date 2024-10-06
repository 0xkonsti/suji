mod bitfield_grid;

pub use bitfield_grid::BitfieldGrid;
use std::fmt::Debug;

const ASCII_ZERO: u8 = 48;

pub enum BackendType {
    BitfieldGrid,
}

pub trait Backend: Debug {
    fn reset(&mut self);

    fn get(&self, row: usize, col: usize) -> u8;
    fn set(&mut self, row: usize, col: usize, value: u8);
    fn unset(&mut self, row: usize, col: usize);

    fn get_empty_cells(&self) -> &Vec<(usize, usize)>;
    fn is_valid(&self) -> bool;

    // ---------- PRE-Implemented methods ----------
    fn load_from_str(&mut self, input: &str, null_chr: char) {
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

        self.reset();

        for i in 0..9 {
            for j in 0..9 {
                let value = digit_grid[i * 9 + j];
                if value != 0 {
                    self.set(i, j, value);
                }
            }
        }
    }

    fn to_string(&self, null_chr: char) -> String {
        let mut output = String::with_capacity(81);
        for i in 0..9 {
            for j in 0..9 {
                let value = self.get(i, j);
                if value == 0 {
                    output.push(null_chr);
                } else {
                    output.push((value + ASCII_ZERO) as char);
                }
            }
        }
        output
    }
}

pub trait CloneableBackend: Backend {
    fn clone_box(&self) -> Box<dyn CloneableBackend>;
}

impl<T> CloneableBackend for T
where
    T: Backend + Clone + 'static,
{
    fn clone_box(&self) -> Box<dyn CloneableBackend> {
        Box::new(self.clone())
    }
}
