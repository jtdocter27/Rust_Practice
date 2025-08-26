fn main() {
    let number =10;
{
let number = 22;
println!("{}", number);
}

println!("{}", number);

fn abc() -> String { // defines a function called abc, declares the return type "String", then declares the string 'abc' and converts the string slice to it
     "abc".to_string() //looks like functions don't need to end with ;. Statements and expressions do. 
}

let letters = abc(); 
let letters = String::from("New Letters");

// let cloned_letters = abc().clone(); //this costs extra memory and CPU but avoids ownership issues. 

println!("{}", letters);
// println!("{}", cloned_letters);

}