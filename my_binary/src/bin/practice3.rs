fn main() {
    fn numbersprac() {
        let three: i8 = 3; 
        let mut any_number = 20; 
        any_number += 1;
        println!("{}", any_number); 
        any_number -= 3; 
        println!("{}", any_number)

    }
numbersprac();
    fn interior() {
        #[derive(Debug)]
        struct Coordinate {x: i8, y: i8}

        let mut coord = Coordinate {x: 20, y:20}; 
        coord.x = 12; 
        coord.y = 13; 
        dbg!(&coord); 
    }
interior(); 
    fn constantinople() { 
        const POKEMON: &str = "Bulbasaur"; //must be screaming snake case and requires a type declaration, 
        const SECONDS_IN_A_DAY: usize = 60*60*25; //constants can be assigned to any expression. usize is an unsigned integer that autmatically macthes memory requirements.  
        println!("{} {}", POKEMON, SECONDS_IN_A_DAY); 





    }
constantinople();


}