fn main() {
    //Monad
    let vehicle = Some("bicycle"); //if you wrap a value, it has to be unwrapped to see what the actual value is
    let v = vehicle.unwrap(); 
    // println!("{v}"); 

    let biker = Some("Peugeot"); 
    let pedestrian: Option<&str> = None; 

    if biker.is_some() {
        println!("The biker is riding a {}", biker.unwrap())}
    
    println!("The pedestrian {}", pedestrian.unwrap_or("is walking")); 

//Result Monad-----------------------------------------------------------------------------
    fn less_than_5(number:i32) -> Result<bool, String>{ //Result wraps the number in both success and failure states. 
        if number <=0 {
            Err("negative Number".to_string()) //Err is a constructor. Each one of these is a block than can be executed. 
        } else if number < 5 {
            Ok(true) //ok(true) defines a success in this case, ebcause we defined it as a bool
        } else {
            Ok(false) //ok(false) defines a bool failure. It says that the input is valid but not what we're looking for. Err is for invalid inputs
        }
    }
// println!("{:?}", less_than_5(-5)); 

fn check_counts() -> Result<bool, String> {
    let count_a = less_than_5(3)?;
    let count_b = less_than_5(2)?;
    let count_c = less_than_5(-5)?; //The "?" operator unwraps the returned value, which is false, and then assigns it to count_c
    // without the ? operator, it crashes, because the value is Ok(false) and not the clean boolean true/false. 
    if count_a && count_b && count_c { //&& is the logical "and" operator. This lines says "if count_a and count_b and count_c are true"
      Ok(true)
    } else {
      Ok(false)
    }
  }
  
  let count_err = check_counts();
  println!("{:?}", count_err)
}


