#![allow(warnings)] //macro that gets rid of all warnings in the script.


pub const FLOUNDER:& 'static str = "Flounder"; 


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
        // println!("{} {}", POKEMON, SECONDS_IN_A_DAY); 
    }
constantinople();

pub mod cake {
    pub fn favorite(name: &str) -> bool {
        name == "Coconut"
    } // public function for comparing a literal string Coconut to whatever is inputted. Will return true or false, hence the "bool" portion of this
    
}
let guess = cake::favorite("Marble");  //this is path syntax - essentially a way to access a function in a module
println!("{}", guess);

mod ocean {
    pub const ATLANTIC: &'static str = "Atlantic"; //creates a public constant called ATLANTIC that is a global variable

    pub mod fish {
        pub fn print_flounder() {
            use crate::FLOUNDER;
            use super::ATLANTIC; 

            println!("A {FLOUNDER} in the {ATLANTIC}");


            }
            
        }
    } 
ocean::fish::print_flounder(); 
let x = 1; 
assert!(x>0, "Failed"); 
}