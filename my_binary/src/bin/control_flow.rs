#[derive(PartialEq, Eq)] // this allows for matching comparisons. 
enum Stars {Sun, Planet, Moon} //creates a type "ennumerate"

fn main() {
    fn stars() {}
        let stars = Stars::Moon; 
        if stars == Stars::Sun {
            println!("I am the Sun")
        }
        else if stars != Stars::Planet {
            println!("We are the planets")
        }
        else {
            println!{"Mother is the Moon"}
        }
    fn mootch() {
        let birds = true; 

        match birds {
            true => println!("Yes!"),
            false => println!("No, no birds"),   }
        }
mootch(); 
        fn mootch2() {
            let plant = "lilac"; 

            match plant {"dandelion" | "almond" | "apple" => println!("Rose Family"), //this is an example of an Or-pattern. The _ is a wildcard. 
            "tomato" | "pepper" | "potato" => println!("Nightshade Family"), 
            _ => println!("not found")
            }

        }
mootch2();
        fn fish_destructuring() {
            let guess = Some("_"); 

            match guess { //this matching block and "arms" goes through each one of these cases and checks. The Some(x) binds the guess variable to x and the checks conditions
                Some(x) if x.len() > 7 => {
                    println!("incorrect"); 
                    println!("hint: people order our patties"); 
                }
                Some(x) if x == "herring" => println!("Correct"), 
                Some(_) => println!("inccorect"), 
                None => {println!("No Answer")}
            }
        }
fish_destructuring();
        fn looping_practice() {
            'first:  loop {
                println!("First loop!"); 
                'second: loop {
                     println!("Second Loop!"); 
                     // continue; // this restarts at the begining of the loop. "break" stops the loop 
                    break 'second; 
                } 
            }
        }

looping_practice(); 
        

}