fn main() {
    let array: [i32; 4] = [1,2,3,4]; //this is an array 
    let person: (String, i32, bool) = ("Alice".to_string(), 30, true); //this is a typle
    println!("{:?}", array); //ok I don't know why but this weird :? notation is needed to print tuples and arrays. 
    println!("{:?}", person);
    
    struct Starters { //creates a structure called "starters" with 3 fields of type string
        bulbasaur: String, 
        charmander: String, 
        squirtle: String,
    }

let pokemon = Starters { //creates an actual struct variable from starters with the given strings in each field
    bulbasaur: String::from("plant"), //words are &str's by defualt. This is why we have to convert to 
    charmander: String::from("fire"), 
    squirtle: String::from("water"),
}; 

println!("{}", pokemon.charmander);

fn concat() {
    let a = "I "; 
    let b = "Am "; 
    let c = "A Narwal "; 

    let sentence = [a,b,c].concat();
    println!("{}", sentence)
}

concat(); 


    }
