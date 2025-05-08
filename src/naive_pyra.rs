use std::{collections::{HashMap, HashSet}, hash::{Hash, Hasher}, thread, time::Duration};
use indicatif::{MultiProgress, ProgressBar, ProgressIterator, ProgressStyle};
use petgraph::{algo::{connected_components, dijkstra}, prelude::GraphMap, visit::{Dfs}, Directed, Undirected};
use rand::*;

pub fn test_full_graph() {
    let mut to_visit:Vec<Pyramide> = vec![]; 
    for seed in 1..=65534 {
        to_visit.push(Pyramide::init_from_seed(seed))
    }
    println!("Using Naive build_graph method");
    let full_graph = build_graph(to_visit.clone());
    let nb_comp = connected_components(&full_graph);
    println!("Number of nodes : {:?}", full_graph.node_count());
    println!("Number of edges : {:?}", full_graph.edge_count());
    println!("Number of components : {:?}", nb_comp);
    
    return;
    let mut already_visited = HashSet::<Pyramide>::default();
    let mut map_real_index: HashMap<u32, Vec<Pyramide>> = HashMap::new();

    let mut cpt = 0;

    for current_node_index in full_graph.nodes().progress() {
        // code taken from https://stackoverflow.com/questions/46778636/return-all-petgraph-connected-components-in-a-hashtable
        if already_visited.contains(&current_node_index) {
            continue;
        }
        let mut current_vec: Vec<Pyramide> = Vec::new();
        let mut dfs = Dfs::new(&full_graph, current_node_index);
        while let Some(nx) = dfs.next(&full_graph) {
            current_vec.push(nx);
            already_visited.insert(nx);
        }
        map_real_index.insert(cpt, current_vec);
        cpt += 1;
    }
    
    for (id, comp) in map_real_index.iter() {
        if comp.len() > 1 {
            println!("Comp {id} has {:?} elements", comp.len());
        } else {
            println!("Comp {id} has pyramide :");
            comp[0].print();
        }
    }
    
    let pos_possible = pos_possibles();
    for pos in pos_possible.iter() {
        let mut pyramide = Pyramide::new();
        pyramide.update(pos.ligne, pos.col, false); // retire pos de départ
        for (id, comp) in map_real_index.iter() {
            if comp.contains(&pyramide) {
                pyramide.print();
                println!("Pyramide is in comp : {:?}", id);
                continue;
            }
        }
    }
}

pub fn test_graph_from_start_pos() {
    let pos_possible = pos_possibles();
    let mut to_visit: Vec<Pyramide> = vec![];
    //On initialize avec les positions de départ possible
    for pos in pos_possible.iter() {
        let mut pyramide = Pyramide::new();
        pyramide.update(pos.ligne, pos.col, false);
        to_visit.push(pyramide.clone());
    }

    let state_graph = build_graph(to_visit);
    println!("Stats du composant de graph contenant les positions de départ");
    println!("Graph nodes : {:?}", state_graph.node_count());
    println!("Graph edges : {:?}", state_graph.edge_count());

    let mut start = Pyramide::new();
    start.update('a', 4, false);
    let node_dist = dijkstra(&state_graph, start, None, |_| 1);
    let max_node_dist = node_dist.values().max().expect("always number");
    println!("Max graph depth from graph : {:?}", max_node_dist);
    let deepest_games: Vec<(&Pyramide, &i32)> = node_dist.iter().filter(|(_, dist)| *dist == max_node_dist ).collect();
    println!("Number of deepest games : {:?}", deepest_games.len())
}

fn build_graph(mut to_visit: Vec<Pyramide>) -> GraphMap<Pyramide, Coup, Undirected>{
    let mut state_graph = GraphMap::<Pyramide, Coup, Undirected>::new();
    let pos_possible = pos_possibles();

    let mut new_to_visit: Vec<Pyramide> = vec![];
    let mult_prog = MultiProgress::new();
    let bar = mult_prog.add(ProgressBar::new(65534));
    mult_prog.println("starting!").unwrap();
    while to_visit.len() != 0 {
        bar.set_position(state_graph.node_count() as u64);
        let bar2 = mult_prog.add(ProgressBar::new(to_visit.len() as u64));
        for pyra in to_visit.drain(..).progress_with(bar2) {
            let coups_possibles = cherche_coups_possible(&pyra, &pos_possible);
            if coups_possibles.is_none() {
                state_graph.add_node(pyra);
                continue;
            }
            for coup_pos in coups_possibles.expect("checked before").iter() {
                let mut new_pyramide = pyra.clone();
                new_pyramide
                    .coup_coup(coup_pos.clone())
                    .expect("Coup always possible");
                if !state_graph.contains_node(new_pyramide) {
                    new_to_visit.push(new_pyramide.clone())
                }
                state_graph.add_edge(pyra, new_pyramide, *coup_pos);
            }
        }
        to_visit.extend(new_to_visit.drain(..));
    }
    bar.finish();
    return state_graph
}

pub fn brute_force_search() {
    let pos_possible = pos_possibles();
    let mut won_games: Vec<Pyramide> = vec![];
    let mut lost_games: Vec<Pyramide> = vec![];
    for pos in pos_possible.iter() {
        let mut pyramide = Pyramide::new();
        pyramide.update(pos.ligne, pos.col, false); // retire pos de départ
        let mut branches: Vec<Pyramide> = vec![pyramide];
        let mut new_branches: Vec<Pyramide> = vec![];
        while branches.len() != 0 {
            for pyra in branches.drain(0..branches.len()) {
                if pyra.partie_finie() {
                    won_games.push(pyra);
                    continue;
                }
                let coups_possibles = cherche_coups_possible(&pyra, &pos_possible);
                if coups_possibles.is_none() {
                    lost_games.push(pyra);
                    continue;
                }
                for coup_pos in coups_possibles.expect("checked before").iter() {
                    let mut new_pyramide = pyra.clone();
                    new_pyramide
                        .coup_coup(coup_pos.clone())
                        .expect("Coup always possible");
                    new_branches.push(new_pyramide)
                }
            }
            branches.extend(new_branches.drain(..))
        }
    }
    println!("Won games : {:?} ; Lost games : {:?}", won_games.len(), lost_games.len());
}

fn _jouer_partie1() {
    let mut pyramide = Pyramide::new();
    pyramide.update('a', 4, false);
    pyramide.print();
    // pyramide.coup('c', 4, 'a', 4).unwrap();
    let coups = _partie1();
    let res = _enchainer_coups(coups, &mut pyramide);
    if res.is_err() {
        let coup_invalide = res.unwrap_err();
        println!("failed to execute coup {:?}", coup_invalide);
    }
    pyramide.print();
}

fn _enchainer_coups(coups: Vec<Coup>, pyramide: &mut Pyramide) -> Result<(), Coup> {
    for coup in coups {
        if let Err(_) = pyramide.coup(
            coup.pos_depart.ligne,
            coup.pos_depart.col as i32,
            coup.pos_arrive.ligne,
            coup.pos_arrive.col as i32,
        ) {
            return Err(coup);
        };
        if pyramide.partie_finie() {
            return Ok(());
        }
    }
    return Ok(());
}

#[derive(Debug, Clone, Copy, Hash, PartialEq)]
struct Position {
    ligne: char,
    col: usize,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq)]
struct Coup {
    pos_depart: Position,
    pos_arrive: Position,
}

const DIAG_RULE_ALLOWED: bool = true;

#[derive(Debug, Clone, Copy, Eq, PartialOrd, Ord)]
struct Pyramide {
    a: [bool;1],
    b: [bool;3],
    c: [bool;5],
    d: [bool;7],
    // _coups: Vec<Coup>,
}

impl PartialEq for Pyramide {
    fn eq(&self, other: &Self) -> bool {
        if self.a == other.a && self.b == other.b && self.c == other.c && self.d == other.d {
            return true
        }
        return false
    }

    fn ne(&self, other: &Self) -> bool {
        if self.a != other.a || self.b != other.b || self.c != other.c || self.d != other.d {
            return true
        }
        return false
    }
}

impl Hash for Pyramide {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.a.hash(state);
        self.b.hash(state);
        self.c.hash(state);
        self.d.hash(state);
    }
}


impl Pyramide {
    fn new() -> Pyramide {
        Pyramide {
            a: [true],
            b: [true, true, true],
            c: [true, true, true, true, true],
            d: [true, true, true, true, true, true, true],
            // _coups: vec![],
        }
    }

    fn empty() -> Pyramide {
        Pyramide { 
            a: [false],
            b: [false, false, false],
            c: [false, false, false, false, false],
            d: [false, false, false, false, false, false, false] }
    }

    fn init_from_seed(seed: u16) -> Pyramide {
        let val: Vec<bool> = (0..16).map(|x:u16| (seed & (1 << x)) != 0).collect();
        return Pyramide { 
            a: [val[0]],
            b: [val[1], val[2], val[3]],
            c: [val[4], val[5], val[6], val[7], val[8]],
            d: [val[9], val[10], val[11], val[12], val[13], val[14], val[15]],
        }
    }

    fn partie_finie(&self) -> bool {
        let mut count_true = 0;
        if self.a[0] {
            count_true += 1
        };
        count_true += self
            .b
            .iter()
            .map(|x| if *x == true { 1 } else { 0 })
            .sum::<i32>();
        count_true += self
            .c
            .iter()
            .map(|x| if *x == true { 1 } else { 0 })
            .sum::<i32>();
        count_true += self
            .d
            .iter()
            .map(|x| if *x == true { 1 } else { 0 })
            .sum::<i32>();
        return count_true == 1;
    }

    fn get(&self, ligne: char, col: usize) -> Option<bool> {
        match ligne {
            'a' => return Some(self.a[col-4]),
            'b' => return Some(self.b[col-3]),
            'c' => return Some(self.c[col-2]),
            'd' => return Some(self.d[col-1]),
            _ => return None,
        };
    }

    fn update(&mut self, ligne: char, col: usize, value: bool) {
        match ligne {
            'a' => self.a[col-4] = value,
            'b' => self.b[col-3] = value,
            'c' => self.c[col-2] = value,
            'd' => self.d[col-1] = value,
            _ => (),
        };
    }

    fn print(&self) {
        println!("  1 2 3 4 5 6 7");
        println!("a       {}      ", self.a[0] as u8);
        let b: Vec<u8> = self.b.clone().into_iter().map(|x| x as u8).collect();
        println!("b     {} {} {}   ", b[0], b[1], b[2]);
        let c: Vec<u8> = self.c.clone().into_iter().map(|x| x as u8).collect();
        println!("c   {} {} {} {} {}", c[0], c[1], c[2], c[3], c[4]);
        let d: Vec<u8> = self.d.clone().into_iter().map(|x| x as u8).collect();
        println!(
            "d {} {} {} {} {} {} {}",
            d[0], d[1], d[2], d[3], d[4], d[5], d[6],
        );
        println!();
    }

    fn _check_coup_valide(
        &self,
        ligne_depart: char,
        col_depart: i32,
        ligne_arrive: char,
        col_arrive: i32,
    ) -> bool {
        // Check point de départ dans les cases possible, nécessaire car le .get en dessous ne vérifie pas
        match (ligne_depart, col_depart) {
            ('a', 4) | ('b', 3..=5) | ('c', 2..=6) | ('d', 1..=7) => (),
            (_, _) => return false,
        }
        if self.get(ligne_depart, col_depart as usize) == Some(false) {
            return false;
        }
        match (ligne_arrive, col_arrive) {
            // Check point d'arrivée dans les cases possibles
            ('a', 4) | ('b', 3..=5) | ('c', 2..=6) | ('d', 1..=7) => (),
            (_, _) => return false,
        }
        if self.get(ligne_arrive, col_arrive as usize) == Some(true) {
            return false;
        }
        if DIAG_RULE_ALLOWED {
            match (col_arrive - col_depart).abs() {
                // Check écart entre col départ arrivée
                2 | 0 => (),
                _ => return false,
            }
            match (ligne_depart, ligne_arrive) {
                // Check écart en ligne départ arrivée
                ('a', 'c')
                | ('b', 'd')
                | ('b', 'b')
                | ('c', 'c')
                | ('d', 'd')
                | ('d', 'b')
                | ('c', 'a') => (),
                _ => return false,
            }
        } else {
            match ((col_arrive - col_depart).abs(), ligne_arrive, ligne_depart) {
                (2, 'b', 'b')
                | (2, 'c', 'c')
                | (2, 'd', 'd')
                | (0, 'a', 'c')
                | (0, 'b', 'd')
                | (0, 'c', 'a')
                | (0, 'd', 'b') => (),
                _ => return false,
            }
        }
        // besoin de vérifier qu'il y a un pion à true entre les 2 points
        let ligne_milieu = find_middle_line(ligne_arrive, ligne_depart);
        let dist = (col_arrive - col_depart) / 2;
        // Si le pion du milieu est à false alors il manque un pion à sauter
        if self.get(ligne_milieu, (col_depart + dist) as usize) == Some(false) {
            return false;
        }

        true
    }

    fn coup_pos(&mut self, pos_depart: Position, pos_arrive: Position) -> Result<(), &str> {
        self.coup(
            pos_depart.ligne,
            pos_depart.col as i32,
            pos_arrive.ligne,
            pos_arrive.col as i32,
        )
    }

    fn coup_coup(&mut self, coup: Coup) -> Result<(), &str> {
        self.coup_pos(coup.pos_depart, coup.pos_arrive)
        // let res = self.coup_pos(coup.pos_depart, coup.pos_arrive);
        // match res {
        //     Ok(()) => {
        //         self._coups.push(coup.clone());
        //         return Ok(());
        //     }
        //     Err(_) => return Err("retrow"),
        // }
    }

    fn coup(
        &mut self,
        ligne_depart: char,
        col_depart: i32,
        ligne_arrive: char,
        col_arrive: i32,
    ) -> Result<(), &'static str> {
        if !self._check_coup_valide(ligne_depart, col_depart, ligne_arrive, col_arrive) {
            return Err("coup invalide");
        }

        self.update(ligne_depart, col_depart as usize, false);
        self.update(ligne_arrive, col_arrive as usize, true);

        let ligne_milieu = find_middle_line(ligne_arrive, ligne_depart);
        let dist = (col_arrive - col_depart) / 2;
        self.update(ligne_milieu, (col_depart + dist) as usize, false);
        return Ok(());
    }
}

fn find_middle_line(ligne_arrive: char, ligne_depart: char) -> char {
    let ligne_milieu = match (ligne_arrive, ligne_depart) {
        ('a', 'c') | ('b', 'b') | ('c', 'a') => 'b',
        ('b', 'd') | ('c', 'c') | ('d', 'b') => 'c',
        ('d', 'd') => 'd',
        _ => panic!("not supposed to happen"),
    };
    ligne_milieu
}

fn _partie1() -> Vec<Coup> {
    vec![
        Coup {
            pos_depart: Position { ligne: 'c', col: 4 },
            pos_arrive: Position { ligne: 'a', col: 4 },
        },
        Coup {
            pos_depart: Position { ligne: 'c', col: 2 },
            pos_arrive: Position { ligne: 'c', col: 4 },
        },
        Coup {
            pos_depart: Position { ligne: 'd', col: 4 },
            pos_arrive: Position { ligne: 'b', col: 4 },
        },
        Coup {
            pos_depart: Position { ligne: 'd', col: 6 },
            pos_arrive: Position { ligne: 'd', col: 4 },
        },
        Coup {
            pos_depart: Position { ligne: 'b', col: 5 },
            pos_arrive: Position { ligne: 'd', col: 5 },
        },
    ]
}

fn pos_possibles() -> Vec<Position> {
    let mut pos_possible_init = Vec::<Position>::new();
    for (l, start, end) in [('a', 4, 4), ('b', 3, 5), ('c', 2, 6), ('d', 1, 7)] {
        for c in start..=end {
            pos_possible_init.push(Position { ligne: l, col: c })
        }
    }
    return pos_possible_init;
}


fn cherche_coups_possible(pyramide: &Pyramide, pos_possible: &Vec<Position>) -> Option<Vec<Coup>> {
    // chercher toutes les cases false et voir si on peut les remplir
    // possible de remplir de plusieurs façons

    let mut coups_possibles = Vec::<Coup>::new();
    for dep in pos_possible.iter() {
        for arr in pos_possible.iter() {
            if pyramide._check_coup_valide(dep.ligne, dep.col as i32, arr.ligne, arr.col as i32) {
                coups_possibles.push(Coup {
                    pos_depart: dep.clone(),
                    pos_arrive: arr.clone(),
                })
            }
        }
    }

    // faut renvoyer le cas où il n'y a pas de coups possibles
    if coups_possibles.len() > 0 {
        return Some(coups_possibles);
    } else {
        return None;
    }
}


fn _test_progress_bar() {
    let m = MultiProgress::new();
    let sty = ProgressStyle::with_template(
        "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
    )
    .unwrap()
    .progress_chars("##-");

    let n = 200;
    let pb = m.add(ProgressBar::new(n));
    pb.set_style(sty.clone());
    pb.set_message("todo");
    let pb2 = m.add(ProgressBar::new(n));
    pb2.set_style(sty.clone());
    pb2.set_message("finished");

    let pb3 = m.insert_after(&pb2, ProgressBar::new(1024));
    pb3.set_style(sty);

    m.println("starting!").unwrap();

    let mut threads = vec![];

    let m_clone = m.clone();
    let h3 = thread::spawn(move || {
        for i in 0..1024 {
            thread::sleep(Duration::from_millis(2));
            pb3.set_message(format!("item #{}", i + 1));
            pb3.inc(1);
        }
        m_clone.println("pb3 is done!").unwrap();
        pb3.finish_with_message("done");
    });

    for i in 0..n {
        thread::sleep(Duration::from_millis(15));
        if i == n / 3 {
            thread::sleep(Duration::from_secs(2));
        }
        pb.inc(1);
        let pb2 = pb2.clone();
        threads.push(thread::spawn(move || {
            thread::sleep(rand::rng().random_range(Duration::from_secs(1)..Duration::from_secs(5)));
            pb2.inc(1);
        }));
    }
    pb.finish_with_message("all jobs started");

    for thread in threads {
        let _ = thread.join();
    }
    let _ = h3.join();
    pb2.finish_with_message("all jobs done");
    m.clear().unwrap();
}