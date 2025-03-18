fn main() {
    let pos_possible = pos_possibles();
    let mut won_games: Vec<Pyramide> = vec![];
    let mut lost_games: Vec<Pyramide> = vec![];
    for pos in pos_possible.iter() {
        println!("Pos testing : {:?}", pos);
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
                    // new_pyramide.print();
                    new_pyramide
                        .coup_coup(coup_pos.clone())
                        .expect("Coup always possible");
                    // new_pyramide.print();
                    new_branches.push(new_pyramide)
                }
            }
            branches.extend(new_branches.drain(..))
        }
    }
    println!("{:?}", won_games.len());
    println!("{:?}", lost_games.len());

    lost_games[0].print();
    lost_games[1].print();
    lost_games[2].print();
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

#[derive(Debug, Clone, Copy)]
struct Position {
    ligne: char,
    col: usize,
}

#[derive(Debug, Clone, Copy)]
struct Coup {
    pos_depart: Position,
    pos_arrive: Position,
}

const DIAG_RULE_ALLOWED: bool = false;

#[derive(Debug, Clone)]
struct Pyramide {
    a: Vec<bool>,
    b: Vec<bool>,
    c: Vec<bool>,
    d: Vec<bool>,
    _coups: Vec<Coup>,
}

impl Pyramide {
    fn new() -> Pyramide {
        Pyramide {
            a: vec![true],
            b: vec![true, true, true],
            c: vec![true, true, true, true, true],
            d: vec![true, true, true, true, true, true, true],
            _coups: vec![],
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
        let (dep, offset) = match ligne {
            'a' => (&self.a, 4),
            'b' => (&self.b, 3),
            'c' => (&self.c, 2),
            'd' => (&self.d, 1),
            _ => return None,
        };
        return Some(dep[col - offset]);
    }

    fn update(&mut self, ligne: char, col: usize, value: bool) {
        let (dep, offset) = match ligne {
            'a' => (&mut self.a, 4),
            'b' => (&mut self.b, 3),
            'c' => (&mut self.c, 2),
            'd' => (&mut self.d, 1),
            _ => return (),
        };
        dep[col - offset] = value
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
        // self.update(ligne_milieu, (col_depart + dist) as usize, false);

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
        let res = self.coup_pos(coup.pos_depart, coup.pos_arrive);
        match res {
            Ok(()) => {
                self._coups.push(coup.clone());
                return Ok(());
            }
            Err(_) => return Err("retrow"),
        }
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
