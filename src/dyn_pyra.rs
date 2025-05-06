fn build_graph() {

}

#[derive(PartialEq)]
struct Coup {
    mil: MoveCellIndex,
    orientation: Orientation,
    dir: Direction,
}

#[derive(PartialEq)]
enum Orientation {
    Horizontal,
    Vertical,
    DiagonalG,
    DiagonalD
}

#[derive(PartialEq)]
enum Direction {
    Haut,
    Bas
}

type Cell = Option<bool>;
type CellIndex = usize;
type MoveCellIndex = usize;

struct Pyramide {
    lines: usize,
    pub cells: Vec<Cell>,
    diag_allowed: bool,
}    

impl Pyramide {
    fn new(lines: usize, diag_allowed: bool) -> Pyramide {
        Pyramide { lines, cells: vec![None; (lines+1)*(2*lines-1)], diag_allowed: diag_allowed }
    }

    fn init_from_seed(lines: usize, seed: usize, diag_allowed: bool) -> Pyramide {
        let mut pyra = Pyramide::new(lines, diag_allowed);

        let cells_num = lines*lines;
        let max_line = 2 * lines - 1;

        let vals: Vec<bool> = (0..cells_num).map(|x| (seed & (1 << x)) != 0).collect();

        let mut index: usize = 0;
        for l in 0..lines {
            let nb_val_remplir = 2 * l + 1;
            let begin = max_line*l+(max_line-nb_val_remplir)/2;
            let end = begin + nb_val_remplir;
            let temp = &mut pyra.cells[begin..end];
            for val in temp.iter_mut() {
                *val = Cell::Some(vals[index]);
                index += 1;
            }
        }
        return pyra
    }

    fn init_full(lines: usize, diag_allowed: bool) -> Pyramide {
        return Pyramide::init_from_seed(
            lines,
            2_usize.pow((lines*lines).try_into().unwrap())-1,
            diag_allowed
        )
    }

    fn cell_index_to_vec_index(&self, idx: CellIndex) -> Result<usize, ()> {
        if idx >= self.lines*self.lines {
            return Err(())
        }
        let mut cur_idx = 0;
        let max_line = self.lines * 2 - 1;
        for (row, line_length) in (1..=max_line).step_by(2).enumerate() {
            cur_idx += line_length;
            if idx < cur_idx {
                let base = (row+1) * max_line;
                let recul = (max_line-1)/2 - row;
                return Ok(base - recul + idx - cur_idx); 
            };
        };
        Err(())
    }

    fn move_cell_index_to_vec_index(&self, idx: CellIndex) -> Result<usize, ()> {
        if idx >= self.lines*self.lines {
            return Err(())
        }
        let mut cur_idx = 0;
        let max_line = self.lines * 2 - 1;
        for (row, line_length) in (1..=max_line-2).step_by(2).enumerate() {
            cur_idx += line_length;
            if idx < cur_idx {
                // on est sur la bonne ligne
                let base = (row+2) * max_line;
                let recul = (max_line-1)/2 - row;
                return Ok(base - recul + idx - cur_idx); 
            };
        };
        Err(())
    }

    pub fn iter(&self) -> CellsIterator {
        CellsIterator { pyramide: self, index: 0, col_index: 0, cur_line: 0 }
    }

    pub fn move_iter(&self) -> MoveCellsIterator {
        MoveCellsIterator { cells: &self.cells, lines: self.lines, index: 0, col_index: 0, cur_line: 1 }
    }

    pub fn is_coup_valid(&self, coup: Coup) -> bool {
        let idx = match self.move_cell_index_to_vec_index(coup.mil) {
            Ok(idx) => idx,
            Err(_) => return false
        };
        let max_line = 2 * self.lines - 1;
        let first_row = idx - max_line;
        let sec_row = idx;
        let third_row = idx + max_line;

        let top = &self.cells[first_row-1..=first_row+1];
        let mid = &self.cells[sec_row-1..=sec_row+1];
        let bot = &self.cells[third_row-1..=third_row+1];

        if mid[1] != Some(true) {
            return false
        };

        let pat = [Some(true), Some(false)];
        return match (coup.orientation, coup.dir) {
            (Orientation::DiagonalG, Direction::Bas) if self.diag_allowed => [top[0], bot[2]] == pat,
            (Orientation::DiagonalG, Direction::Haut) if self.diag_allowed => [bot[2], top[0]] == pat,
            (Orientation::Vertical, Direction::Bas) => [top[1], bot[1]] == pat,
            (Orientation::Vertical, Direction::Haut) => [bot[1], top[1]] == pat,
            (Orientation::DiagonalD, Direction::Bas) if self.diag_allowed => [top[2], bot[0]] == pat,
            (Orientation::DiagonalD, Direction::Haut) if self.diag_allowed => [bot[0], top[2]] == pat,
            (Orientation::Horizontal, Direction::Bas) => [mid[0], mid[2]] == pat,
            (Orientation::Horizontal, Direction::Haut) => [mid[2], mid[0]] == pat,
            _ => false,
        }
    }

    pub fn is_there_a_valid_coup(&self, mil: usize) -> bool {
        // mil is center of a possible move
        let (first_row, sec_row, third_row) = (mil*(self.lines-1) , mil*self.lines, mil*(self.lines+1));
        let msg = "this slice is always 3";
        let trois_trois: [[Option<bool>;3];3] = [self.cells[first_row-1..first_row+1].try_into().expect("should"), 
                                         self.cells[sec_row-1..sec_row+1].try_into().expect(msg),
                                         self.cells[third_row-1..third_row+1].try_into().expect(msg)];

        match trois_trois {
            [[Some(true), _, _], [_, Some(true), _], [_,_,Some(false)]] if self.diag_allowed => true,
            [[Some(false), _, _], [_, Some(true), _], [_,_,Some(true)]] if self.diag_allowed => true,
            [[_, Some(true), _], [_, Some(true), _], [_,Some(false),_]] => true,
            [[_, Some(false), _], [_, Some(true), _], [_,Some(true),_]] => true,
            [[_, _, Some(true)], [_, Some(true), _], [Some(false),_,_]] if self.diag_allowed => true,
            [[_, _, Some(false)], [_, Some(true), _], [Some(true),_,_]] if self.diag_allowed => true,
            [[_, _, _], [Some(true), Some(true), Some(false)], [_,_,_]] => true,
            [[_, _, _], [Some(false), Some(true), Some(true)], [_,_,_]] => true,
            _ => false
        }
    }

    pub fn print(&self) {
        let mut first_line = " ".to_string();
        for i in 1..=self.lines*2-1 {
            first_line.push_str(format!(" {}", i).as_str());
        }
        // println!("  1 2 3 4 5 6 7");
        for l in self.cells.chunks(self.lines*2-1) {

        }
        // println!("a       {}      ", self.a[0] as u8);
        // let b: Vec<u8> = self.b.clone().into_iter().map(|x| x as u8).collect();
        // println!("b     {} {} {}   ", b[0], b[1], b[2]);
        // let c: Vec<u8> = self.c.clone().into_iter().map(|x| x as u8).collect();
        // println!("c   {} {} {} {} {}", c[0], c[1], c[2], c[3], c[4]);
        // let d: Vec<u8> = self.d.clone().into_iter().map(|x| x as u8).collect();
        // println!(
        //     "d {} {} {} {} {} {} {}",
        //     d[0], d[1], d[2], d[3], d[4], d[5], d[6],
        // );
        println!();
    }
}

struct CellsIterator<'a> {
    pyramide: &'a Pyramide,
    index: CellIndex,
    col_index: usize,
    cur_line: usize,
}

impl<'a> Iterator for CellsIterator<'a> {
    // Iterates on all the pyramide cells
    type Item = &'a Cell;
    fn next(&mut self) -> Option<Self::Item> {
        if self.cur_line < self.pyramide.lines {
            let max_line = self.pyramide.lines * 2 - 1;
            let recul = (max_line-1)/2 - self.cur_line;
            let temp_index = self.cur_line*max_line + recul + self.col_index;

            self.col_index += 1;
            self.index += 1;
            if self.col_index > 2 * self.cur_line {
                self.col_index = 0;
                self.cur_line += 1;
            };
            Some(&self.pyramide.cells[temp_index])
        }
        else {
            None
        }
    }
}

struct MoveCellsIterator<'a> {
    cells: &'a Vec<Cell>,
    lines: usize,
    index: MoveCellIndex,
    col_index: usize,
    cur_line: usize,
}

impl<'a> Iterator for MoveCellsIterator<'a> {
    // Iterates on all the pyramide cells
    type Item = &'a Cell;
    fn next(&mut self) -> Option<Self::Item> {
        if self.cur_line < self.lines {
            let max_line = self.lines * 2 - 1;
            let recul = (max_line-1)/2 - self.cur_line + 1;
            let temp_index = self.cur_line*max_line + recul + self.col_index;

            self.col_index += 1;
            self.index += 1;
            if self.col_index > 2 * (self.cur_line - 1) {
                self.col_index = 0;
                self.cur_line += 1;
            };
            Some(&self.cells[temp_index])
        }
        else {
            None
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_pyra_2() {
        let pyra = Pyramide::init_full(2, false);
        assert_eq!(pyra.lines, 2);
        let expected_pyra = vec![None, Some(true), None, Some(true), Some(true), Some(true)];
        assert_eq!(pyra.cells, expected_pyra);
    }
    #[test]
    fn create_pyra_3() {
        let pyra = Pyramide::init_full(3, false);
        assert_eq!(pyra.lines, 3);
        let expected_pyra = vec![None, None, Some(true), None, None, None, Some(true), Some(true), Some(true), None, Some(true), Some(true), Some(true), Some(true), Some(true)];
        assert_eq!(pyra.cells, expected_pyra);
    }
    #[test]
    fn create_pyra_4() {
        let pyra = Pyramide::init_full(4, false);
        assert_eq!(pyra.lines, 4);
        let expected_pyra = vec![
            None, None, None, Some(true), None, None, None,
            None, None, Some(true), Some(true), Some(true), None, None,
            None, Some(true), Some(true), Some(true), Some(true), Some(true), None,
            Some(true), Some(true), Some(true), Some(true), Some(true), Some(true), Some(true)];
        assert_eq!(pyra.cells, expected_pyra);
    }

    #[test]
    fn test_iterator() {
        let pyra = Pyramide::init_full(3, false);
        for elem in pyra.iter() {
            assert_eq!(elem, &Some(true))
        }
    }

    #[test]
    fn test_init_seed_1() {
        let pyra = Pyramide::init_from_seed(3, 1, false);
        assert_eq!(pyra.cells, vec![None, None, Some(true), None, None, None, Some(false), Some(false), Some(false), None, Some(false), Some(false), Some(false), Some(false), Some(false)])
    }
    #[test]
    fn test_init_seed_6() {
        let pyra = Pyramide::init_from_seed(3, 6, false);
        assert_eq!(pyra.cells, vec![None, None, Some(false), None, None, None, Some(true), Some(true), Some(false), None, Some(false), Some(false), Some(false), Some(false), Some(false)])
    }

    #[test]
    fn test_iter_move_cells() {
        let pyra = Pyramide::init_from_seed(3, 228, false);
        for cell in pyra.move_iter() {
            assert_eq!(cell, &Some(true))
        }
    }

    #[test]
    fn test_coup_valide() {
        let pyra = Pyramide::init_from_seed(3, 510, false);
        let coup = Coup {dir: Direction::Haut, mil: 0, orientation: Orientation::Vertical};
        assert!(pyra.is_coup_valid(coup))
    }

    #[test]
    fn test_cell_index_to_vec_index_size_3() {
        let pyra = Pyramide::init_full(3, false);
        let res = pyra.cell_index_to_vec_index(8);
        assert_eq!(res, Ok(14));
        let res = pyra.cell_index_to_vec_index(2);
        assert_eq!(res, Ok(7));
    }

    #[test]
    fn test_cell_index_to_vec_index_size_4() {
        let pyra = Pyramide::init_full(4, false);
        let res = pyra.cell_index_to_vec_index(8);
        assert_eq!(res, Ok(19));
        let res = pyra.cell_index_to_vec_index(2);
        assert_eq!(res, Ok(10));
    }

    #[test]
    fn test_move_cell_index_to_vec_index_size_3() {
        let pyra = Pyramide::init_full(3, false);
        let res = pyra.move_cell_index_to_vec_index(3);
        assert_eq!(res, Ok(13));
        let res = pyra.move_cell_index_to_vec_index(2);
        assert_eq!(res, Ok(12));
        let res = pyra.move_cell_index_to_vec_index(0);
        assert_eq!(res, Ok(7));
    }

    #[test]
    fn test_move_cell_index_to_vec_index_size_4() {
        let pyra = Pyramide::init_full(4, false);
        let res = pyra.move_cell_index_to_vec_index(8);
        assert_eq!(res, Ok(26));
        let res = pyra.move_cell_index_to_vec_index(2);
        assert_eq!(res, Ok(17));
    }
}