use crate::board::{BOARD_SIZE, Board, Cell, Move, NonEmptyCell};
use crate::directions::{ALL_DIRECTIONS, DOUBLE_THREE_TAB, PRIMARY_DIRECTIONS};
use smallvec::SmallVec;

#[derive(Debug)]
pub enum BoardError {
    InvalidMove,
    OccupiedCell,
    FreeThree,
    BoardEmpty,
}

impl Board {
    pub fn unset(&mut self) -> Result<(), BoardError> {
        let last_move = self.moves.pop();
        match last_move {
            None => Err(BoardError::BoardEmpty),
            Some(mov) => {
                match self.grid[mov.x][mov.y] {
                    Cell::Black => {
                        for captured in mov.captured {
                            self.grid[captured.0][captured.1] = Cell::White;
                        }
                    }
                    Cell::White => {
                        for captured in mov.captured {
                            self.grid[captured.0][captured.1] = Cell::White;
                        }
                    }
                    _ => (),
                }
                self.grid[mov.x][mov.y] = Cell::Empty;
                Ok(())
            }
        }
    }
}

impl Board {
    pub fn check_secure(&self, x: i32, y: i32, cell: Cell, opposite_cell: Cell) -> bool {
        for (dx, dy) in ALL_DIRECTIONS {
            let x1 = x + dx;
            let y1 = y + dy;
            let x2 = x + 2 * dx;
            let y2 = y + 2 * dy;
            let x_1 = x - dx;
            let y_1 = y - dy;
            if x2 >= 0
                && x2 < BOARD_SIZE as i32
                && y2 >= 0
                && y2 < BOARD_SIZE as i32
                && x_1 >= 0
                && x_1 < BOARD_SIZE as i32
                && y_1 >= 0
                && y_1 < BOARD_SIZE as i32
                && self.grid[(x1) as usize][(y1) as usize] == cell
            {
                match self.grid[(x2) as usize][(y2) as usize] {
                    Cell::Empty => {
                        if self.grid[(x_1) as usize][(y_1) as usize] == opposite_cell {
                            return false;
                        }
                    }
                    other => {
                        if other == opposite_cell
                            && self.grid[(x_1) as usize][(y_1) as usize] == Cell::Empty
                        {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }
}

impl Board {
    fn count_direction(
        &self,
        x: i32,
        y: i32,
        dx: i32,
        dy: i32,
        cell: Cell,
        opposite_cell: Cell,
    ) -> i32 {
        let mut count = 0;
        let mut nx = x + dx;
        let mut ny = y + dy;

        while nx >= 0
            && nx < BOARD_SIZE as i32
            && ny >= 0
            && ny < BOARD_SIZE as i32
            && self.grid[nx as usize][ny as usize] == cell
            && self.check_secure(nx, ny, cell, opposite_cell)
        {
            count += 1;
            nx += dx;
            ny += dy;
        }
        count
    }
}

impl Board {
    fn capture(&mut self, x: i32, y: i32, cell: NonEmptyCell) -> SmallVec<[(usize, usize); 4]> {
        let mut captured = SmallVec::new();
        let opposite_cell = cell.get_opposite();
        let cell = cell.get();

        for (dx, dy) in ALL_DIRECTIONS {
            if (x + 3 * dx) >= 0
                && (x + 3 * dx) < BOARD_SIZE as i32
                && (y + 3 * dy) >= 0
                && (y + 3 * dy) < BOARD_SIZE as i32
                && (self.grid[(x + dx) as usize][(y + dy) as usize] == opposite_cell)
                && (self.grid[(x + 2 * dx) as usize][(y + 2 * dy) as usize] == opposite_cell)
                && (self.grid[(x + 3 * dx) as usize][(y + 3 * dy) as usize] == cell)
            {
                captured.push(((x + dx) as usize, (y + dy) as usize));
                captured.push(((x + 2 * dx) as usize, (y + 2 * dy) as usize));
            }
        }
        captured
    }
}

impl Board {
    #[inline(always)]
    fn match_cell(cell: Cell, curent_cell: Cell, expected: u8) -> bool {
        expected == 0 && cell == Cell::Empty || expected == 1 && cell == curent_cell
    }

    #[inline(always)]
    fn match_pattern(
        &self,
        nx: &i32,
        ny: &i32,
        i: &i32,
        cell: Cell,
        lst_tab: &mut SmallVec<[u8; 4]>,
    ) {
        let mut j = 0;
        if *nx >= 0 && *nx < BOARD_SIZE as i32 && *ny >= 0 && *ny < BOARD_SIZE as i32 {
            while j < lst_tab.len() {
                let expected = DOUBLE_THREE_TAB[lst_tab[j] as usize][*i as usize];
                if expected != 8 {
                    if Self::match_cell(self.grid[*nx as usize][*ny as usize], cell, expected) {
                        j += 1;
                    } else {
                        _ = lst_tab.remove(j);
                    }
                } else {
                    j += 1;
                }
            }
        } else {
            while j < lst_tab.len() {
                if DOUBLE_THREE_TAB[lst_tab[j] as usize][*i as usize] != 8 {
                    lst_tab.remove(j);
                } else {
                    j += 1;
                }
            }
        }
    }

    fn check_one_three(
        &self,
        x: usize,
        y: usize,
        dx: i32,
        dy: i32,
        cell: Cell,
        lst_tab: &mut SmallVec<[u8; 4]>,
    ) -> bool {
        let x1 = x as i32 + dx;
        let y1 = y as i32 + dy;
        let x_1 = x as i32 - dx;
        let y_1 = y as i32 - dy;

        if x1 >= 0
            && x1 < BOARD_SIZE as i32
            && y1 >= 0
            && y1 < BOARD_SIZE as i32
            && x_1 >= 0
            && x_1 < BOARD_SIZE as i32
            && y_1 >= 0
            && y_1 < BOARD_SIZE as i32
        {
            match self.grid[x1 as usize][y1 as usize] {
                Cell::Empty => match self.grid[x_1 as usize][y_1 as usize] {
                    Cell::Empty => lst_tab.extend([2, 9]),
                    c if c == cell => lst_tab.extend([0, 1, 5, 7]),
                    _ => return false,
                },
                c if c == cell => match self.grid[x_1 as usize][y_1 as usize] {
                    Cell::Empty => lst_tab.extend([4, 6, 10, 11]),
                    c if c == cell => lst_tab.extend([3, 8]),
                    _ => return false,
                },
                _ => return false,
            }

            for i in 0..3 {
                let nx = x as i32 - (4 + i) * dx;
                let ny = y as i32 - (4 + i) * dy;
                self.match_pattern(&nx, &ny, &i, cell, lst_tab);
            }

            for i in 6..9 {
                let nx = x as i32 + (i - 4) * dx;
                let ny = y as i32 + (i - 4) * dy;
                self.match_pattern(&nx, &ny, &i, cell, lst_tab);
            }

            if lst_tab.len() >= 1 {
                return true;
            }
        }
        false
    }

    fn free_trees(&mut self, x: usize, y: usize, cell: Cell) -> bool {
        let mut lst_tab: SmallVec<[u8; 4]> = SmallVec::new();
        let mut get_one: bool = false;

        for (dx, dy) in PRIMARY_DIRECTIONS {
            if self.check_one_three(x, y, dx, dy, cell, &mut lst_tab) {
                if !get_one {
                    get_one = true;
                } else {
                    return true;
                }
                lst_tab.clear();
            }
        }
        false
    }
}

impl Board {
    pub fn set(&mut self, x: usize, y: usize, cell: NonEmptyCell) -> Result<(), BoardError> {
        let new_cell = cell.get();
        if x >= BOARD_SIZE || y >= BOARD_SIZE {
            return Err(BoardError::InvalidMove);
        }

        if self.grid[x][y] == Cell::Empty {
            if self.free_trees(x, y, new_cell) {
                return Err(BoardError::FreeThree);
            }
            self.grid[x][y] = new_cell;

            let captured: SmallVec<[(usize, usize); 4]> = self.capture(x as i32, y as i32, cell);
            match cell {
                NonEmptyCell::Black => self.captured_black += captured.len(),
                NonEmptyCell::White => self.captured_white += captured.len(),
            }
            for (cx, cy) in &captured {
                self.grid[*cx][*cy] = Cell::Empty;
            }
            print!("Captured: ");
            for (cx, cy) in &captured {
                print!("({}, {}) ", cx, cy);
            }
            println!();
            self.moves.push(Move {
                x,
                y,
                cell,
                captured,
            });
            return Ok(());
        }
        return Err(BoardError::OccupiedCell);
    }
}

impl Board {
    pub fn check(&self, x: usize, y: usize, cell: NonEmptyCell) -> bool {
        let x = x as i32;
        let y = y as i32;
        let opposite_cell = cell.get_opposite();

        match cell {
            NonEmptyCell::Black => {
                if self.captured_black >= 10 {
                    return true;
                }
            }
            NonEmptyCell::White => {
                if self.captured_white >= 10 {
                    return true;
                }
            }
        }

        let cell: Cell = cell.get();

        for (dx, dy) in PRIMARY_DIRECTIONS {
            let count = 1
                + self.count_direction(x, y, dx, dy, cell, opposite_cell)
                + self.count_direction(x, y, -dx, -dy, cell, opposite_cell);

            if count >= 5 {
                return true;
            }
        }

        false
    }
}

impl Board {
    pub fn set_and_check(&mut self, x: usize, y: usize, cell: NonEmptyCell) -> bool {
        match self.set(x, y, cell) {
            Ok(()) => self.check(x, y, cell),
            Err(e) => {
                println!("Error: {:?}", e);
                false
            }
        }
    }
}
