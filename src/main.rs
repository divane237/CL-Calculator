use std::io;



fn main() {


  loop {
    let mut input = String::new();
      println!("Please enter your calculation!");

      io::stdin().read_line(&mut input).expect("There was an error");

      let results: Result<f64, String> = calculate(&input);

      // println!("Result of expression: {:?}", results);
      match results {
        Ok(value) => println!("Results of operation is: {}", value),
        Err(msg)=> println!("{}", msg),
      }

  }
  
   
    }

  
  fn calculate(expression: &str) -> Result<f64, String> {

    let  vector: Vec<&str> = expression.trim().split(" ").collect(); 

    if vector.len() != 3 {
      return Err("Include a space between your values".into());
    }

    let left:f64 = match vector[0].parse() {
      Ok(num)=> num,
      Err(_)=> return Err("Not a number".into()),
    };

    let right:f64 = match vector[2].parse(){
      Ok(num) => num,
      Err(_) => return Err("Not a number".into()),

    };

    

    match vector[1] {
     "+" => Ok(left + right),
     "-" => Ok(left - right),
     "*" => Ok(left * right), 
     "/" => if right == 0.0 {
        return Err("Result is undefined".into());
     } else {
      Ok(left / right)
     }
     _ => return Err("Input a valid operator".into())
    }
  }