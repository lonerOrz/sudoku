// board.rs: 数独棋盘数据结构

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Cell {
    Given(u8),
    UserInput(u8),
    Empty,
}

impl Cell {
    pub fn value(&self) -> Option<u8> {
        match self {
            Cell::Given(v) | Cell::UserInput(v) => Some(*v),
            Cell::Empty => None,
        }
    }
}

/// 9×9 Sudoku board. Transparent 2D indexing via Deref.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Grid {
    cells: [[Cell; 9]; 9],
}

impl Grid {
    pub fn new() -> Self {
        Self {
            cells: [[Cell::Empty; 9]; 9],
        }
    }

    pub fn flat(&self) -> [u8; 81] {
        let mut out = [0u8; 81];
        for r in 0..9 {
            for c in 0..9 {
                out[r * 9 + c] = self.cells[r][c].value().unwrap_or(0);
            }
        }
        out
    }

    pub fn from_flat(values: [u8; 81]) -> Self {
        let mut grid = Self::new();
        for (i, &v) in values.iter().enumerate() {
            if v > 0 {
                grid.cells[i / 9][i % 9] = Cell::Given(v);
            }
        }
        grid
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self::new()
    }
}

impl std::ops::Deref for Grid {
    type Target = [[Cell; 9]; 9];
    fn deref(&self) -> &Self::Target {
        &self.cells
    }
}

impl std::ops::DerefMut for Grid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cells
    }
}

impl From<[[Cell; 9]; 9]> for Grid {
    fn from(cells: [[Cell; 9]; 9]) -> Self {
        Self { cells }
    }
}

pub type Solution = [[u8; 9]; 9];

pub const ALL_VALUES: u16 = 0x3FE;

#[derive(Clone)]
pub struct BitmaskGrid {
    pub rows: [u16; 9],
    pub cols: [u16; 9],
    pub boxes: [u16; 9],
}

impl Default for BitmaskGrid {
    fn default() -> Self {
        Self::new()
    }
}

impl BitmaskGrid {
    pub fn new() -> Self {
        Self {
            rows: [0; 9],
            cols: [0; 9],
            boxes: [0; 9],
        }
    }

    pub fn from_grid(grid: &Grid) -> Self {
        let mut masks = Self::new();
        for (r, row) in grid.iter().take(9).enumerate() {
            for (c, cell) in row.iter().take(9).enumerate() {
                if let Some(v) = cell.value() {
                    let bit = 1u16 << v;
                    masks.rows[r] |= bit;
                    masks.cols[c] |= bit;
                    masks.boxes[(r / 3) * 3 + c / 3] |= bit;
                }
            }
        }
        masks
    }

    #[inline]
    pub fn candidates(&self, r: usize, c: usize) -> u16 {
        let b = (r / 3) * 3 + c / 3;
        ALL_VALUES & !(self.rows[r] | self.cols[c] | self.boxes[b])
    }

    #[inline]
    pub fn place(&mut self, r: usize, c: usize, v: u8) {
        let bit = 1u16 << v;
        self.rows[r] |= bit;
        self.cols[c] |= bit;
        self.boxes[(r / 3) * 3 + c / 3] |= bit;
    }

    #[inline]
    pub fn remove(&mut self, r: usize, c: usize, v: u8) {
        let bit = 1u16 << v;
        self.rows[r] &= !bit;
        self.cols[c] &= !bit;
        self.boxes[(r / 3) * 3 + c / 3] &= !bit;
    }
}

pub fn build_candidates(grid: &Grid) -> [sudoku_solver::Candidates; 81] {
    let mut cands = [sudoku_solver::Candidates::full(); 81];
    for i in 0..81 {
        let r = i / 9;
        let c = i % 9;
        if let Some(v) = grid[r][c].value() {
            cands[i] = sudoku_solver::Candidates::empty();
            for &peer in &PEERS[i] {
                if peer == SENTINEL {
                    break;
                }
                cands[peer as usize].remove(v);
            }
        }
    }
    cands
}

const SENTINEL: u8 = u8::MAX;

const fn calc_peers() -> [[u8; 20]; 81] {
    let mut result = [[SENTINEL; 20]; 81];
    let mut i = 0;

    while i < 81 {
        let r = i / 9;
        let c = i % 9;
        let box_r = (r / 3) * 3;
        let box_c = (c / 3) * 3;
        let mut count = 0;
        let mut j = 0;
        while j < 9 {
            if j != c {
                result[i][count] = (r * 9 + j) as u8;
                count += 1;
            }
            j += 1;
        }

        j = 0;
        while j < 9 {
            if j != r {
                let idx = j * 9 + c;
                let mut duplicate = false;
                let mut k = 0;
                while k < count {
                    if result[i][k] == idx as u8 {
                        duplicate = true;
                        break;
                    }
                    k += 1;
                }
                if !duplicate {
                    result[i][count] = idx as u8;
                    count += 1;
                }
            }
            j += 1;
        }

        j = 0;
        while j < 3 {
            let mut k = 0;
            while k < 3 {
                let idx = (box_r + j) * 9 + (box_c + k);
                if idx != i {
                    let mut duplicate = false;
                    let mut m = 0;
                    while m < count {
                        if result[i][m] == idx as u8 {
                            duplicate = true;
                            break;
                        }
                        m += 1;
                    }
                    if !duplicate {
                        result[i][count] = idx as u8;
                        count += 1;
                    }
                }
                k += 1;
            }
            j += 1;
        }

        i += 1;
    }

    result
}

pub static PEERS: [[u8; 20]; 81] = calc_peers();

#[inline]
pub fn is_valid(grid: &Grid, idx: usize, val: u8) -> bool {
    for &peer in &PEERS[idx] {
        if peer == SENTINEL {
            break;
        }
        if let Some(v) = grid[(peer / 9) as usize][(peer % 9) as usize].value()
            && v == val
        {
            return false;
        }
    }
    true
}
