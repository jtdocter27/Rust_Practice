fn main() { // Entry point of the program 
    fn double(num: u128) -> u128 {num *2} //takes a parameter of u128 and returns a u128 multiplied by 2

    let _int: i32 = 32; //explicitly types as as i32, 32-bit signed integer
    let big_int = 10; //rust determines the datatype here
    let _float = 1.2; //float is the same as python, it's a decimal

    let outcome = double(big_int);
    let double_outcome = outcome;

    println!("{}", double_outcome); 
    
}
