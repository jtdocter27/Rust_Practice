fn main() {
    fn and() {
        let pi = 3.41; 
        let _funny_number = &pi; //first reference to pi 
        let still_funny = &&pi; //second reference to pi 
        println!("{still_funny}");
    }
    fn dereference() {
        let mut year = 3020; //creates a mutable variable
        let y = &mut year; //This reference is allowed to make changes. Have to tell Rust this explicitly. 
        *y +=10; // dereferences the variable y, which is necessary to using it. otherwise, compiler will just say you can't change a part of the code that is pointing to the variable
        println!("The year is {year}");
    }
dereference(); 

    fn space() {
        let starship: Option<String> = Some("Forward unto Dawn".to_string()); //Some goes in tandem with Option. It just means there is a value there. This is Rust's way of exception handling when there is missing data 
        match starship {Some(ref name) => println!("{}", name), None => {}} //Checks to see if starship contains some variable, when it does, thie code extracts the inner value from Some and binds the value to the variable name, then prints. Match checks for some vs. none. 
        // println!("{:?}", starship);
    }

space();

    fn sloice() {
        let s = String::from("Hello World"); 
        let hello = &s[6..11]; //slices have to be referenced, they point to existing data. Slicing isn't changing the strings contents, it's saying "The string has these values! look!"
        println!("{hello}"); 
    }

sloice(); 

    fn variable(num: u128) -> u128 {
        let variable = "This is a string"; 
        let equation = num *2; 
        let test: u8 = 28; 
        println!("{}", equation); 
        equation //this returns the value
    }

variable(10); 


    fn patterns() {
        let (a, b) = (10, "pie"); 
        println!("{} {}", a, b); 
    }

patterns(); 
}