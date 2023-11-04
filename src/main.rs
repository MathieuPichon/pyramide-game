fn main() {
    let pyramide = Pyramide::new();
    pyramide.print();


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

    fn print(&self) {
        println!("      {}      ", self.a[0] as u8);
        let b: Vec<u8> = self.b.clone().into_iter().map(|x| x as u8).collect();
        println!("    {} {} {}   ", b[0], b[1], b[2]);
        let c: Vec<u8> = self.c.clone().into_iter().map(|x| x as u8).collect();
        println!("  {} {} {} {} {}", c[0], c[1], c[2], c[3], c[4]);
        let d: Vec<u8> = self.d.clone().into_iter().map(|x| x as u8).collect();
        println!("{} {} {} {} {} {} {}", d[0], d[1], d[2], d[3], d[4], d[5], d[6],);
    }

    fn check_coup_valide(&self, rang_depart: char, col_depart: i32, rang_arrive: char, col_arrive: i32) -> bool {
        if (col_arrive - col_depart).abs() != 2 && (col_arrive - col_depart).abs() != 0 {
            return false
        }
        true
    }
}