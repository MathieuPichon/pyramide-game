fn main() {
    let mut pyramide = Pyramide::new();
    
    pyramide.update('a', 4, false);
    pyramide.print();
    // pyramide.coup('c', 4, 'a', 4).unwrap();
    let coups = partie1();
    let res = enchainer_coups(coups, &mut pyramide);
    if res.is_err() {
        let coup_invalide = res.unwrap_err();
        println!("failed to execute coup {:?}", coup_invalide);
    }
    pyramide.print();
}

fn enchainer_coups(coups: Vec::<Coup>,  pyramide: &mut Pyramide) -> Result<(), Coup>{
    for coup in coups {
        if let Err(_) = pyramide.coup(
            coup.pos_depart.ligne,
            coup.pos_depart.col as i32,
            coup.pos_arrive.ligne,
            coup.pos_arrive.col as i32
        ) {
            return Err(coup)
        };
    }
    return Ok(())
}

#[derive(Debug)]
struct Position {
    ligne: char,
    col: usize,
}

#[derive(Debug)]
struct Coup{
    pos_depart: Position,
    pos_arrive: Position,
}

struct Pyramide {
    a: Vec<bool>,
    b: Vec<bool>,
    c: Vec<bool>,
    d: Vec<bool>
}

impl Pyramide {
    fn new() -> Pyramide {
        Pyramide { 
            a: vec![true], 
            b: vec![true, true, true], 
            c: vec![true, true, true, true, true], 
            d: vec![true, true, true, true, true, true, true] }
    }

    fn get(& self, ligne: char, col: usize) -> Option<bool> {
        let (dep, offset) = match ligne {
            'a' => (&self.a, 4),
            'b' => (&self.b, 3),
            'c' => (&self.c, 2),
            'd' => (&self.d, 1),
            _ => return None
        };
        return Some(dep[col - offset])
    }

    fn update(&mut self, ligne: char, col: usize, value: bool) {
        let (dep, offset) = match ligne {
            'a' => (&mut self.a, 4),
            'b' => (&mut self.b, 3),
            'c' => (&mut self.c, 2),
            'd' => (&mut self.d, 1),
            _ => return ()
        };
        dep[col - offset] = value
    }

    fn print(&self) {
        println!("      {}      ", self.a[0] as u8);
        let b: Vec<u8> = self.b.clone().into_iter().map(|x| x as u8).collect();
        println!("    {} {} {}   ", b[0], b[1], b[2]);
        let c: Vec<u8> = self.c.clone().into_iter().map(|x| x as u8).collect();
        println!("  {} {} {} {} {}", c[0], c[1], c[2], c[3], c[4]);
        let d: Vec<u8> = self.d.clone().into_iter().map(|x| x as u8).collect();
        println!("{} {} {} {} {} {} {}", d[0], d[1], d[2], d[3], d[4], d[5], d[6],);
    }

    fn check_coup_valide(&mut self, ligne_depart: char, col_depart: i32, ligne_arrive: char, col_arrive: i32) -> bool {
        match (ligne_depart, col_depart) { // Check point de départ
            ('a', 4) | ('b', 3..=5) | ('c', 2..=6) | ('d', 1..=7) => (),
            (_, _) => return false
        }
        if self.get(ligne_depart, col_depart as usize) != Some(true) {
            return false
        }
        match (ligne_arrive, col_arrive) { // Check point d'arrivée
            ('a', 4) | ('b', 3..=5) | ('c', 2..=6) | ('d', 1..=7) => (),
            (_, _) => return false
        }
        if self.get(ligne_arrive, col_arrive as usize) != Some(false) {
            return false
        }
        match (col_arrive - col_depart).abs() { // Check écart entre col départ arrivée
            2 | 0 => (),
            _ => return false
        }
        match (ligne_depart, ligne_arrive) { // Check écart en ligne départ arrivée
            ('a', 'c') | ('b', 'd') | ('b', 'b') | ('c', 'c') | ('d','d') | ('d', 'b') | ('c', 'a') => (),
            _ => return false
        }
        true
    }

    fn coup(&mut self, ligne_depart: char, col_depart: i32, ligne_arrive: char, col_arrive: i32) -> Result<(), &str> {
        if !self.check_coup_valide(ligne_depart, col_depart, ligne_arrive, col_arrive) {
            return Err("coup invalide")
        }
        
        self.update(ligne_depart, col_depart as usize, false);
        self.update(ligne_arrive, col_arrive as usize, true);

        let ligne_milieu = match(ligne_arrive, ligne_depart) {
            ('a', 'c') | ('b', 'b') | ('c', 'a')=> 'b',
            ('b', 'd') | ('c','c') | ('d', 'b') => 'c',
            ('d', 'd') => 'd',
            _ => panic!("not supposed to happen")
        };
        let dist = (col_arrive - col_depart)/2;
        self.update(ligne_milieu, (col_depart + dist) as usize, false);
        return Ok(())
    }
}

fn partie1() -> Vec<Coup> {
    vec![
        Coup{pos_depart: Position{ligne: 'c', col: 4} , pos_arrive: Position{ligne: 'a', col: 4}},
        Coup{pos_depart: Position{ligne: 'c', col: 2} , pos_arrive: Position{ligne: 'c', col: 4}},
        Coup{pos_depart: Position{ligne: 'd', col: 4} , pos_arrive: Position{ligne: 'b', col: 4}},
        Coup{pos_depart: Position{ligne: 'd', col: 6} , pos_arrive: Position{ligne: 'd', col: 4}},
        Coup{pos_depart: Position{ligne: 'b', col: 5} , pos_arrive: Position{ligne: 'd', col: 5}},
    ]
}