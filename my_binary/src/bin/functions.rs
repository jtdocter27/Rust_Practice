pub mod iterator {
    pub fn demonstraction() {
        let numbers = [1,2,3]; //this is a collection 
        let mut numbers = numbers.iter(); 
        if let Some(first) = numbers.next(){ //rusts shorthand for pattern matching. If there's a next number in the numbers collection, will assign to the variable
            println!("{first}"); 
        }
        if let Some(second) = numbers.next() {
            println!("{second}")
        }
        if let Some(third) = numbers.next() {
            println!("{third}")
        }
    }}

fn main() {
    fn themathahead(input: u8) -> u8 {
        let answer = input * 2 ; 
        println!("{}", answer); 
        return answer
    }
// themathahead(2);
    fn summer_comes_to_multiply(first:u8, second:u8) -> u8 {
        let answer = first * second; 
        println!("{}", answer); 
        return answer //this will not compile if there is no return, interestingly. 
    }
// summer_comes_to_multiply(2,2); 
    fn increment(num:u8) -> u8 {
        num + 1
    }
    fn moonwater(f: fn(u8) -> u8, num: u8) -> u8 { //this is a function calling a function. First we define the function and it's return value, then add in the another variable
        let answer = f(num); 
        println!("{}", answer); 
        return answer; 
    }
// let inc = moonwater(increment, 2); 

let square = |a| a*a;  // let a variable take in "a" and multiply it by itself. 
let answer = square(2);
// println!("{}", answer);  

let number = 338; 
let output = move || println!("{number}"); //this defines a closure that prints number and infers it from the environment. move takes the value in number and puts it into output, which makes number no longer an accessible variable. 

// output(); 

iterator::demonstraction();

let mut numbers = [10, 20, 30].iter(); 
numbers.next(); 
numbers.next(); 
let remaining: Vec<&u32> = numbers.collect(); //. collect consumes the rest of the iterator and builds a collection from it. Vec<&u32> is the type annotation for a vector 
println!("{:?}", remaining); // we use & above because you have to reference the the numbers being iterated. 
// ------------------------------------------------------------- Map
let numbers = [1, 2, 3];
let nums: Vec<i32> = numbers.iter().map(|x| x*4).collect(); //no & here because the map creates new values. 

println!("{:?}", nums); 

//---------------------------------------------------------------- Filter 
let chars = ['a', '1', 'E', 'F']; 
let filtered: Vec<&char> = chars.iter().filter(|c| c.is_uppercase()).collect(); //collect appears to be necessary 

println!("{filtered:?}"); 



//Enumerate ------------------------------------------------------------------
let people = ["Li", "Patrick", "Spongebob"]; 
let enumerated: Vec<(usize, &&str)> = people.iter().enumerate().filter( |(i, n)| i> &1).collect();

println!("{enumerated:?}"); 


}
