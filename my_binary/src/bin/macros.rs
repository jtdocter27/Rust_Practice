fn main() {
    //general macro form is macro_rules! title {(input) => output}
// Declarative Macro -------------------------------------------------------------------------------------
    macro_rules! make_it {
        ($var:ident => //($var:ident) : captures any identifier and call it $var. 
        $($count:expr),+)=> {  // $($count:expr),+) : capture 1+ expressions separated by commas
            $($var.push($count);)+ //repeat this for each captured item 
        }
    }
let mut count = vec![]; 
make_it![count => u8::MIN, 1, 2]; //u8::MIN is a fancy way of saying 0 
println!("{count:?}"); 

// Procedeural Macro --------------------------------------------------------------------

// macro_for_fun!(); function like macro
// #[dance] Attribute macros, applies to the thing directly after it. 
// struct Human{ 
//     legs: u8, 
//     arms: u8, 
// }






}

