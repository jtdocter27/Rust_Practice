fn main() {
let array: [i32; 4] = [1,2,3,4]; //this is an array 
let person: (String, i32, bool) = ("Alice".to_string(), 30, true); //this is a tuple
println!("{:?}", array); //ok I don't know why but this weird :? notation is needed to print tuples and arrays. 
println!("{:?}", person);
//Structures -----------------------------------------------------------------------------

struct Guest {
    name: &'static str,
    rsvp: bool,
    costume: bool,
  }
let guest = Guest {
    name: "Anand",
    rsvp: true,
    costume: false,
};

let guest1 = Guest {
    name: "Kim",
    rsvp: true,
    costume: true,
  };
match guest1 {
Guest { name: "Kim", rsvp: true, costume: true } => println!("Selena is wearing a costume to the party!"),
Guest { name: "Selena", rsvp: true, costume: false } => println!("Selena is not dressing up for the party."),
_ => {}
} //this is a complex bit of code. So the match function looks at the values in guest1, then compares them to the types in Guest struct and the values after it. If there's a match, it prints. the _ => is a catch all that handles outside values and does nothing

impl Guest { //the point here is that you can create sort of a custom block with custom functions dedicated to a certain structure. 
    pub fn print_name(&self) {
        println!("{}", self.name); 

    }
}

guest1.print_name();

//Arrays -----------------------------------------------------------------------------
fn concat() {
    let a = "I "; 
    let b = "Am "; 
    let c = "A Narwal "; 

    let sentence = [a,b,c].concat();
    println!("{}", sentence)
}

concat(); 

let many_e = ['e'; 20]; //a way of creating 20 of something in an array. 
println!("{:?}", many_e); 

let alpha = ['a', 'b', 'c', 'd', 'e', 'f', 'g']; 
let a_index = alpha[0]; //finds the value at the first position in the array
println!("{a_index}");

let multi = &alpha[1..3]; //this is how you access a range in an array 
println!("{:?}",multi); 

alpha.iter().map(|c| println!("{c}")); 
//Vectors-------------------------------------------------------------

let new_vec: Vec<char> = vec![]; //creates a new vector using a macro. 
let alpha_vec = vec!['a', 'b', 'c', 'd', 'e']; //creates a new vector with values. These two examples are unrelated. 

//tuples --------------------------------------------------------------
let cat_tuple: (i8, &str, &str) = (10, "Meowzer", "Von meowzenberg"); //declares a tuple
println!("{cat_tuple:?}");
let (age, name, last_name) = cat_tuple; 
println!("This is {name} {last_name}, he is {age} years old."); 

//Enums -------------------------------------------------------------------------------
enum RarePokemon {
    Kangeskan, 
    Tangela, 
    Magmar, 
    Kabuto, 
}

let rare_fire = RarePokemon::Magmar;

match rare_fire {
    RarePokemon::Tangela => println!("Critical hit!"), 
    RarePokemon::Kangeskan => println!("Little effect"), 
    RarePokemon::Kabuto => println!("Water type, no effect"),
    RarePokemon::Magmar =>println!("Magmar vs. Magmar!"), 

}; 

enum Meal {
    Pasta, 
    StirFry(Vec<String>), // this is a vector of strings. Remember, vecs are expandable. 
    Burrito {
        beans: bool, 
        rice: bool, 
    }, 
}

let dinner = Meal::StirFry(vec!["carrots".to_string()]); //defines the thing we want to match against



match dinner { // matches dinner against all possible enums. 
    Meal::Pasta => println!("Pasta!!"), //if the meal is simply pasta, it prints pasta
    Meal::Burrito {beans, rice} => { // if the meal is burrito, it tells you if it has boolean true or false for beans and rice
        println!("With beans: {beans}"); 
        println!("with rice: {rice}"); 
    }
    Meal::StirFry(veggies) => { // if the meal is stirfry, it takes in the vector string and prints it out. The key here is the defining of the vector string. 
        println!{"{veggies:?}"}; 
    }
}
    }
