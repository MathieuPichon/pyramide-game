use std::hash::{Hash, Hasher};

use indicatif::{MultiProgress, ProgressBar, ProgressIterator};
use petgraph::{algo::connected_components, prelude::GraphMap, Undirected};

pub fn test_dyn_graph() {
    let diag_allowed = true;
    println!("Using build_full_graph method");
    let lines = 4;
    let full_graph = build_full_graph(lines, diag_allowed);
    let nb_comp = connected_components(&full_graph);
    println!("Number of nodes : {:?}", full_graph.node_count());
    println!("Number of edges : {:?}", full_graph.edge_count());
    println!("Number of components : {:?}", nb_comp);

    println!("Using build_partial_graph method");
    let max_seed = 2_u128.pow((lines*lines) as u32) - 1;
    let pyras = (1..max_seed).map(|x| Pyramide::init_from_seed(lines, x, diag_allowed)).collect();
    let full_graph_partial = build_partial_graph(pyras);
    let nb_comp_partial = connected_components(&full_graph_partial);
    println!("Number of nodes : {:?}", full_graph_partial.node_count());
    println!("Number of edges : {:?}", full_graph_partial.edge_count());
    println!("Number of components : {:?}", nb_comp_partial);
}

pub fn build_full_graph(lines: usize, diag_allowed: bool) -> GraphMap::<Seed, Coup, Undirected> {
    let mut state_graph = GraphMap::<Seed, Coup, Undirected>::new();
    let temp_pyra = Pyramide::new(lines, diag_allowed);
    let coups_theoriques = coups_theoriques(&temp_pyra);
    let max_seed: u128 = 2_u128.pow((lines*lines).try_into().unwrap()) - 1;
    let prog_bar = ProgressBar::new((max_seed-1) as u64);
    for seed in (1..max_seed).progress_with(prog_bar) {
        let pyra = Pyramide::init_from_seed(lines, seed, diag_allowed);
        let coups_possibles = cherche_coups_possibles(&pyra, &coups_theoriques);
        if coups_possibles.is_none() {
            state_graph.add_node(seed);
            continue;
        };
        for coup in coups_possibles.expect("already checked") {
            let mut new_pyra = pyra.clone();
            new_pyra.coup(&coup).expect("already checked");
            state_graph.add_edge(seed, new_pyra.seed(), coup);
        }
    };

    return state_graph
}

fn build_partial_graph(mut to_visit: Vec<Pyramide>) -> GraphMap::<Seed, Coup, Undirected> {
    // Create graph with all reachable positions from the positions in to_visit
    let mut state_graph = GraphMap::<Seed, Coup, Undirected>::new();
    let coups_theoriques = coups_theoriques(&to_visit[0]);

    let mut new_to_visit: Vec<Pyramide> = vec![];
    let mult_prog = MultiProgress::new();
    let bar = mult_prog.add(ProgressBar::new((to_visit[0].lines.pow(2)-2) as u64));
    mult_prog.println("starting!").unwrap();
    while to_visit.len() != 0 {
        bar.set_position(state_graph.node_count() as u64);
        let bar2 = mult_prog.add(ProgressBar::new(to_visit.len() as u64));
        for pyra in to_visit.drain(..).progress_with(bar2) {
            let coups_possibles = cherche_coups_possibles(&pyra, &coups_theoriques);
            let pyra_seed = pyra.seed();
            if coups_possibles.is_none() {
                state_graph.add_node(pyra_seed);
                continue;
            };
            for coup_pos in coups_possibles.expect("checked before").iter() {
                let mut new_pyramide = pyra.clone();
                new_pyramide
                    .coup(coup_pos)
                    .expect("Coup always possible");
                let new_pyra_seed = new_pyramide.seed();
                if !state_graph.contains_node(new_pyra_seed) {
                    new_to_visit.push(new_pyramide);
                };
                state_graph.add_edge(pyra_seed, new_pyra_seed, *coup_pos);
            }
        }
        to_visit.extend(new_to_visit.drain(..));
    }
    bar.finish();
    return state_graph
}


fn cherche_coups_possibles(pyramide: &Pyramide, coups: &Vec<Coup>) -> Option<Vec<Coup>> {
    let res: Vec<Coup> = coups.iter()
        .filter_map(|x| (pyramide.is_coup_valid(x)).then(|| *x)).collect();
    if res.len() > 0 {
        return Some(res)
    } else {
        return None
    }
}

#[derive(Debug, PartialEq, Hash, Clone, Copy, Eq, PartialOrd, Ord)]
pub struct PyramideRules {
    lines: usize,
    diag_allowed: bool
}

#[derive(Debug, PartialEq, Hash, Clone, Copy)]
pub struct Coup {
    mil: CellIndex,
    orientation: Orientation,
    dir: Direction,
}

#[derive(Debug, PartialEq, Hash, Clone, Copy)]
pub enum Orientation {
    Horizontal,
    Vertical,
    DiagonalG,
    DiagonalD
}

#[derive(Debug, PartialEq, Hash, Clone, Copy)]
pub enum Direction {
    Haut,
    Bas
}

type Seed = u128;
// This size allows for the graph resolutions to be computed with a pyramide of lines : 11
// to solve larger boards, a bigger seed representation needs to be implemented (the code is most likely too slow to handle such size)
// or change the way to generate boards

type Cell = Option<bool>;
type CellIndex = usize;
// type MoveCellIndex = usize;

#[derive(Debug, Clone, PartialOrd, Eq, Ord)]
pub struct Pyramide {
// pub struct Pyramide<'a>{
    // rules: &'a PyramideRules,
    lines: usize,
    cells: Vec<Cell>,
    diag_allowed: bool,
}    

impl Pyramide {
    fn new(lines: usize, diag_allowed: bool) -> Pyramide {
        Pyramide { lines, cells: vec![None; (lines+1)*(2*lines+1)], diag_allowed: diag_allowed }
    }

    fn init_from_seed(lines: usize, seed: Seed, diag_allowed: bool) -> Pyramide {
        let mut pyra = Pyramide::new(lines, diag_allowed);

        let cells_num = lines*lines;
        let max_line = 2 * lines + 1;

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
            2_u128.pow((lines*lines).try_into().unwrap())-1,
            diag_allowed
        )
    }

    fn partie_gagne(&self) -> bool {
        self.cells.iter().filter(|x| *x == &Some(true)).count() == 1
    }

    fn update_cell(&mut self, idx: CellIndex, value: Cell) -> Result<(),()> {
        let vec_idx = match self.cell_index_to_vec_index(idx) {
            Ok(vec_idx) => vec_idx,
            Err(e) => return Err(e)
        };
        self.cells[vec_idx] = value;
        Ok(())
    }

    fn coup(&mut self, coup: &Coup) -> Result<(),()> {
        if !self.is_coup_valid(&coup) {
            return Err(())
        };
        let mil_vec_idx = self.cell_index_to_vec_index(coup.mil).expect("Already checked");
        
        _ = self.cells[mil_vec_idx].replace(false);

        let max_line = self.lines * 2 + 1;
        let top_row = mil_vec_idx - max_line;
        let mid_row = mil_vec_idx;
        let bot_row = mil_vec_idx + max_line;

        // let top = &mut self.cells[first_row-1..=first_row+1];
        // let mid = &mut self.cells.[sec_row-1..=sec_row+1];
        // let bot = &mut self.cells[third_row-1..=third_row+1];

        let (dep_idx, arr_idx) = match (coup.orientation, coup.dir) {
            (Orientation::DiagonalG, Direction::Bas) => (top_row-1, bot_row+1),
            (Orientation::DiagonalG, Direction::Haut) => (bot_row+1, top_row-1),
            (Orientation::Vertical, Direction::Bas) => (top_row, bot_row),
            (Orientation::Vertical, Direction::Haut) => (bot_row, top_row),
            (Orientation::DiagonalD, Direction::Bas) => (top_row+1, bot_row-1),
            (Orientation::DiagonalD, Direction::Haut) => (bot_row-1, top_row+1),
            (Orientation::Horizontal, Direction::Bas) => (mid_row-1, mid_row+1),
            (Orientation::Horizontal, Direction::Haut) => (mid_row+1, mid_row-1),
        };
        _ = self.cells[dep_idx].replace(false);
        _ = self.cells[arr_idx].replace(true);
        Ok(())
    }

    fn cell_index_to_vec_index(&self, idx: CellIndex) -> Result<usize, ()> {
        if idx >= self.lines*self.lines {
            return Err(())
        }
        let mut cur_idx = 0;
        let max_line_length = self.lines * 2 - 1;
        for (row, line_length) in (1..=max_line_length).step_by(2).enumerate() {
            cur_idx += line_length;
            if idx < cur_idx {
                let base = (row+1) * (max_line_length+2);
                let recul = (max_line_length+2)/2 - row;
                return Ok(base - recul + idx - cur_idx); 
            };
        };
        Err(())
    }

    // fn move_cell_index_to_vec_index(&self, idx: CellIndex) -> Result<usize, ()> {
    //     if idx >= self.lines*self.lines {
    //         return Err(())
    //     }
    //     let mut cur_idx = 0;
    //     let max_line = self.lines * 2 - 1;
    //     for (row, line_length) in (1..=max_line-2).step_by(2).enumerate() {
    //         cur_idx += line_length;
    //         if idx < cur_idx {
    //             // on est sur la bonne ligne
    //             let base = (row+2) * max_line;
    //             let recul = (max_line-1)/2 - row;
    //             return Ok(base - recul + idx - cur_idx); 
    //         };
    //     };
    //     Err(())
    // }

    pub fn iter(&self) -> CellsIterator {
        CellsIterator { cells: &self.cells, lines: self.lines, index: 0, col_index: 0, cur_line: 0 }
    }

    pub fn move_iter(&self) -> CellsIterator {
        CellsIterator { cells: &self.cells, lines: self.lines, index: 1, col_index: 0, cur_line: 1 }
    }

    pub fn is_coup_valid(&self, coup: &Coup) -> bool {
        let idx = match self.cell_index_to_vec_index(coup.mil) {
            Ok(idx) => idx,
            Err(_) => return false
        };
        let max_line = 2 * self.lines + 1;
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

    pub fn is_there_a_valid_coup(&self, mil: CellIndex) -> bool {
        // mil is center of a possible move
        let idx = match self.cell_index_to_vec_index(mil) {
            Ok(idx) => idx,
            Err(_) => return false
        };
        let max_line = 2 * self.lines - 1;
        let first_row = idx - max_line;
        let sec_row = idx;
        let third_row = idx + max_line;

        let msg = "this slice is always 3";
        let trois_trois: [[Option<bool>;3];3] = [self.cells[first_row-1..first_row+1].try_into().expect(msg), 
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

    fn seed(&self) -> u128 {
        self.iter()
            .enumerate()
            .filter_map(|(i, cell)| (cell == &Some(true)).then(|| 1 << i))
            .sum()
    }
}

impl Hash for Pyramide {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.seed().hash(state);
        self.diag_allowed.hash(state);
    }
}

impl PartialEq for Pyramide {
    fn eq(&self, other: &Self) -> bool {
        (self.cells == other.cells) & (self.diag_allowed == other.diag_allowed)
    }
}

fn coups_theoriques(pyra: &Pyramide) -> Vec<Coup> {
    let mut res = Vec::new();
    let max_pos = (pyra.lines)*(pyra.lines);
    let orients = if pyra.diag_allowed {
        vec![Orientation::DiagonalD, Orientation::DiagonalG,
             Orientation::Horizontal, Orientation::Vertical]
    } else {
        vec![Orientation::Horizontal, Orientation::Vertical]
    };
    let directions = [Direction::Bas, Direction::Haut];
    for orientation in orients {
        for dir in directions {
            for mil in 1..max_pos {
                res.push(Coup { mil, orientation, dir });
            }
        }
    }
    return res
}

pub struct CellsIterator<'a> {
    cells: &'a Vec<Cell>,
    lines: usize,
    index: CellIndex,
    col_index: usize,
    cur_line: usize,
}

impl<'a> Iterator for CellsIterator<'a> {
    type Item = &'a Cell;
    fn next(&mut self) -> Option<Self::Item> {
        if self.cur_line < self.lines {
            let max_line = self.lines * 2 + 1;
            let recul = (max_line-1)/2 - self.cur_line;
            let temp_index = self.cur_line*max_line + recul + self.col_index;

            self.col_index += 1;
            self.index += 1;
            if self.col_index > 2 * self.cur_line {
                self.col_index = 0;
                self.cur_line += 1;
            };
            return Some(&self.cells[temp_index])
        }
        else {
            return None
        }
    }
}

// pub struct MoveCellsIterator<'a> {
//     cells: &'a Vec<Cell>,
//     lines: usize,
//     index: MoveCellIndex,
//     col_index: usize,
//     cur_line: usize,
// }

// impl<'a> Iterator for MoveCellsIterator<'a> {
//     // Iterates on all the pyramide cells
//     type Item = &'a Cell;
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.cur_line < self.lines {
//             let max_line = self.lines * 2 - 1;
//             let recul = (max_line-1)/2 - self.cur_line + 1;
//             let temp_index = self.cur_line*max_line + recul + self.col_index;

//             self.col_index += 1;
//             self.index += 1;
//             if self.col_index > 2 * (self.cur_line - 1) {
//                 self.col_index = 0;
//                 self.cur_line += 1;
//             };
//             Some(&self.cells[temp_index])
//         }
//         else {
//             None
//         }
//     }
// }


#[cfg(test)]
mod tests {
    use std::iter::zip;

    use super::*;

    #[test]
    fn create_pyra_2() {
        let pyra = Pyramide::init_full(2, false);
        assert_eq!(pyra.lines, 2);
        let expected_pyra = vec![
            None, None,       Some(true), None,       None,
            None, Some(true), Some(true), Some(true), None,
            None, None,       None,       None,       None];
        assert_eq!(pyra.cells, expected_pyra);
    }
    
    #[test]
    fn create_pyra_3() {
        let pyra = Pyramide::init_full(3, false);
        assert_eq!(pyra.lines, 3);
        let cell = Some(true);
        let expected_pyra = vec![
            None, None, None, cell, None, None, None,
            None, None, cell, cell, cell, None, None,
            None, cell, cell, cell, cell, cell, None,
            None, None, None, None, None, None, None];
        assert_eq!(pyra.cells, expected_pyra);
    }
    
    #[test]
    fn create_pyra_4() {
        let pyra = Pyramide::init_full(4, false);
        assert_eq!(pyra.lines, 4);
        let cell = Some(true);
        let expected_pyra = vec![
            None, None, None, None, cell, None, None, None, None,
            None, None, None, cell, cell, cell, None, None, None,
            None, None, cell, cell, cell, cell, cell, None, None,
            None, cell, cell, cell, cell, cell, cell, cell, None,
            None, None, None, None, None, None, None, None, None];
        assert_eq!(pyra.cells, expected_pyra);
    }

    #[test]
    fn test_iterator() {
        let pyra = Pyramide::init_full(3, false);
        for elem in pyra.iter() {
            assert_eq!(elem, &Some(true))
        }
        assert_eq!(pyra.iter().collect::<Vec<&Cell>>().len(), 9)
    }

    #[test]
    fn test_init_seed_1() {
        let pyra = Pyramide::init_from_seed(3, 1, false);
        let celt = Some(true);
        let celf = Some(false);
        assert_eq!(pyra.cells, vec![
            None, None, None, celt, None, None, None,
            None, None, celf, celf, celf, None, None,
            None, celf, celf, celf, celf, celf, None,
            None, None, None, None, None, None, None])
    }
    
    #[test]
    fn test_init_seed_6() {
        let pyra = Pyramide::init_from_seed(3, 6, false);
        let celt = Some(true);
        let celf = Some(false);
        assert_eq!(pyra.cells, vec![
            None, None, None, celf, None, None, None,
            None, None, celt, celt, celf, None, None,
            None, celf, celf, celf, celf, celf, None,
            None, None, None, None, None, None, None])
    }

    #[test]
    fn test_iter_move_cells() {
        // _ _ F
        // _ T F T
        // F T F T F
        // seed : 170
        let pyra = Pyramide::init_from_seed(3, 170, false);
        for (i, cell) in pyra.move_iter().enumerate() {
            if i % 2 == 0 {
                assert_eq!(cell, &Some(true))
            } else {
                assert_eq!(cell, &Some(false))
            }
        }
    }

    #[test]
    fn test_coup_valide_lines_3_seed_510() {
        // _ _ F
        // _ T T T
        // T T T T T
        // seed : 510
        let pyra = Pyramide::init_from_seed(3, 510, false);
        let coup = Coup {dir: Direction::Haut, mil: 2, orientation: Orientation::Vertical};
        assert!(pyra.is_coup_valid(&coup))
    }

    #[test]
    fn test_coup_valide_lines_4_seed_65534() {
        // _ _ _ F
        // _ _ T T T
        // _ T T T T T
        // T T T T T T T
        // seed : 65534
        let pyra = Pyramide::init_from_seed(4, 65534, false);
        let coup = Coup {dir: Direction::Haut, mil: 2, orientation: Orientation::Vertical};
        assert!(pyra.is_coup_valid(&coup))
    }

    #[test]
    fn test_cell_index_to_vec_index_size_3() {
        let pyra = Pyramide::init_full(3, false);
        let vec_idxs = [3, 9,10,11, 15,16,17,18,19];
        for (cell_idx, vec_idx) in zip(0..9, vec_idxs) {
            assert_eq!(pyra.cell_index_to_vec_index(cell_idx), Ok(vec_idx))
        };
    }

    #[test]
    fn test_cell_index_to_vec_index_size_4() {
        let pyra = Pyramide::init_full(4, false);
        let vec_idx = [4, 12,13,14, 20,21,22,23,24, 28,29,30,31,32,33,34];
        for (cell_idx, vec_idx) in zip(0..16, vec_idx) {
            assert_eq!(pyra.cell_index_to_vec_index(cell_idx), Ok(vec_idx))
        }
    }

    // #[test]
    // fn test_move_cell_index_to_vec_index_size_3() {
    //     let pyra = Pyramide::init_full(3, false);
    //     let res = pyra.cell_index_to_vec_index(3);
    //     assert_eq!(res, Ok(13));
    //     let res = pyra.cell_index_to_vec_index(2);
    //     assert_eq!(res, Ok(12));
    //     let res = pyra.cell_index_to_vec_index(0);
    //     assert_eq!(res, Ok(7));
    // }

    // #[test]
    // fn test_move_cell_index_to_vec_index_size_4() {
    //     let pyra = Pyramide::init_full(4, false);
    //     let res = pyra.move_cell_index_to_vec_index(8);
    //     assert_eq!(res, Ok(26));
    //     let res = pyra.move_cell_index_to_vec_index(2);
    //     assert_eq!(res, Ok(17));
    // }

    #[test]
    fn test_hash() {
        for i in 0..=65535 {
            let pyra = Pyramide::init_from_seed(4, i, false);
            assert_eq!(i, pyra.seed());
        }
    }

    #[test]
    fn test_coups_theoriques() {
        let pyra = Pyramide::init_full(3, false);
        let res = coups_theoriques(&pyra);
        assert_eq!(res.len(), 2 * 2 * 8);

        let pyra = Pyramide::init_full(3, true);
        let res = coups_theoriques(&pyra);
        assert_eq!(res.len(), 4 * 2 * 8);
    }

    #[test]
    fn test_cherche_coups_possibles() {
        // _ _ F
        // _ T T F
        // F T T T F
        // seed : 230
        let pyra = Pyramide::init_from_seed(3, 230, false);
        let coups = coups_theoriques(&pyra);
        let res = cherche_coups_possibles(&pyra, &coups).expect("calculated");
        print!("{:?}", res);
        assert_eq!(res.len(), 4);
        assert!(res.contains(&Coup{mil: 2, orientation: Orientation::Vertical, dir: Direction::Haut}));
        assert!(res.contains(&Coup{mil: 2, orientation: Orientation::Horizontal, dir: Direction::Bas}));
        assert!(res.contains(&Coup{mil: 5, orientation: Orientation::Horizontal, dir: Direction::Haut}));
        assert!(res.contains(&Coup{mil: 7, orientation: Orientation::Horizontal, dir: Direction::Bas}));
        // _ _ F
        // _ T T T
        // T F T T T
        // seed : 478
        let pyra = Pyramide::init_from_seed(3, 478, true);
        let coups = &coups_theoriques(&pyra);
        let res = cherche_coups_possibles(&pyra, coups).expect("calculated");
        assert_eq!(res.len(), 4);
        assert!(res.contains(&Coup { mil: 1, orientation: Orientation::DiagonalD, dir: Direction::Haut }));
        assert!(res.contains(&Coup { mil: 2, orientation: Orientation::Vertical, dir: Direction::Haut }));
        assert!(res.contains(&Coup { mil: 3, orientation: Orientation::DiagonalG, dir: Direction::Haut }));
        assert!(res.contains(&Coup { mil: 6, orientation: Orientation::Horizontal, dir: Direction::Haut }));
    }
}