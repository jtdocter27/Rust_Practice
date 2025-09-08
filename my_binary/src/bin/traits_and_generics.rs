fn main() {
   use std::fmt::Display; // brings the Display trait into scope from the standard library so it can be used. 
   const TEAMMATE: &'static str = "Cooperberg"; // declares a constant as a static string. This means TEAMMATE is immutable and will live for the lifetime of the program 

   pub fn pass_to<'a, T: Display>(name: &'a T) { //this declares a function called pass to that has a lifetime annotation and takes in any data type. 
    println!("Passing the ball to {}", name)
   }

   let right_field: &str = "Gene"; 

   pass_to(&TEAMMATE.to_string());
   pass_to(&right_field.to_string()); 

   //Type Alias'ing --------------------------------------------------------
    use std::collections::BTreeMap as Tree;

    type Key = u64;
    type Value = u64;

    fn to_tree(entries: &Vec<(Key, Value)>) -> Tree<Key, Value> { //this takes "entries" as a type vector which contains a key and a value of type u64
        let mut tree = Tree::new(); // calls the associated function "new" on type "Tree" 
        for (k, v) in entries {
        tree.insert(*k, *v);  // derefences the data so we actually use the values in the variable. 
        }
        tree
    }
        
    let pairs = vec![(2, 3),(800, 900)]; //defining a vector called pairs. Actual data 
    let tree = to_tree(&pairs); // creates a reference to pairs and feeds it into the function 
    println!("{:?}", pairs); 
    println!("{:?}", tree);

    // Traitss and Generics -----------------------------------------------------------------

    trait Describable {
        fn describe(&self) -> String; //This is a method. &self is just a generic way of saying "input some data (instance) here" and then the function returns a string. 
        fn summary(&self) -> String {
            format!("Summary: {}", self.describe()) //macro that creates formatted strings and returns it. 
        }
        struct Organism {
            name: String, 
            species: String, 
        }


    }





}
