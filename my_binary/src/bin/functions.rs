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
let inc = moonwater(increment, 2); 

let square = |a| a*a;  // let a variable take in "a" and multiply it by itself. 
let answer = square(2);
// println!("{}", answer);  

let number = 338; 
let output = move || println!("{number}"); //this defines a closure that prints number and infers it from the environment. move takes the value in number and puts it into output, which makes number no longer an accessible variable. 

// output(); 
pub mod iterator {
    pub fn demonstraction() {
        let pub numbers = [1,2,3];
        let mut numbers = numers.iter(); 


    }
  

}




}