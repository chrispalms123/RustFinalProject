use std::io;
use std::f64;


fn main(){
  /// Declaring a mutable String value for the user to enter their choice
  let mut key=String::new();
  println!("Enter 1 for addition, 2 for subtraction, 3 for multiplication, 4 for division, 5 for squaring, 6 for cubing, 7 for a square root, 8 if you want to know the square root, 9 for remainder, 10 for cube root, 11 for log, 12 for sin, 13 for cosin, 14 for tangent, 15 for factorial" );
  
  /// Using readline to retrieve user's input
  io::stdin().read_line( &mut key).ok();
  let key: i32=key.trim().parse().unwrap();
 
  /// Long if / else if / else statement to determine which option the user picks, it will then perform the proper function for the choice.
  if key== 1 {
    println!("You chose addition", );
  add();
  }
  else if key== 2  {
    println!("You chose subtraction", );
    sub();
  }
  else if key== 3  {
    println!("You chose multiplication", );
    mul();
  }
 else if key== 4 {
    println!("You chose division", );
    div();
  }
  else if key== 5 {
    println!("You want to square a number", );
    square();
  }
  else if key== 6  {
    println!("You want to cube a number", );
    
    cube();
  }
  else if key== 7  {
    println!("You want to perform the square root of a number", );
    
    cube();
  }
  else if key== 8  {
    println!("You want to know the square root of a number", );
     squareroot();
  }
  else if key== 9  {
    println!("You want to know ther remainder", );
    modules();
  }
  else if key== 10  {
    println!("You want to know the cube root of a number", );
    cuberoot();
  }
   else if key== 11  {
    println!("You want to know the log value of a number", );
    log();
  }
  else if key== 12  {
    println!("You want to know the sin value", );
    sinfun();
  }
  else if key== 13  {
    println!("You want to know the cosin value", );
    cosfun();
  }
   else if key== 14  {
    println!("You want to know the tangent value", );
    tanfun();
  }
   else if key== 15  {
    println!("You want to know this number's factorial", );
    factorial();
  }
   
/// This is for if you enter the wrong option
  else {
    println!("This is not an option" );
  }
  

}
/// Initializing an add function to add two numbers together
fn add() {
  let mut no1=String::new();
  println!("Enter a number: " );
  io::stdin().read_line(&mut no1).ok();
  let no1: i32=no1.trim().parse().unwrap();
 
  let mut no2=String::new();
  println!("Enter a number: " );
  io::stdin().read_line(&mut no2).ok();
  let  no2: i32=no2.trim().parse().unwrap();

  println!("Result is {}",(no1+no2) );
 
  
  
  
}

/// Initializing a subtraction function to subtract two numbers together.
fn sub() {
  let mut no1=String::new();
  println!("enter  number" );
  io::stdin().read_line(&mut no1).ok();
  let no1: i32=no1.trim().parse().unwrap();
 
  let mut no2=String::new();
  println!("enter  number" );
  io::stdin().read_line(&mut no2).ok();
  let  no2: i32=no2.trim().parse().unwrap();

  println!("Result is {}",(no1-no2) );
 
  
}

/// Initializing and definding a multiplication function to multiply two numbers together
fn mul() {
  let mut no1=String::new();
  println!("enter  number" );
  io::stdin().read_line(&mut no1).ok();
  let no1: i32=no1.trim().parse().unwrap();
 
  let mut no2=String::new();
  println!("enter  number" );
  io::stdin().read_line(&mut no2).ok();
  let  no2: i32=no2.trim().parse().unwrap();

  println!("Result is {}",(no1*no2) );
 
  
}

/// Initializing and defining a division function to divide two numbers together.
fn div() {
  let mut no1=String::new();
  println!("enter  number" );
  io::stdin().read_line(&mut no1).ok();
  let no1: f64=no1.trim().parse().unwrap();
 
  let mut no2=String::new();
  println!("enter  number" );
  io::stdin().read_line(&mut no2).ok();
  let  no2: f64=no2.trim().parse().unwrap();

  println!("Result is {}",(no1/no2) );
 
  
}

/// Initializing and defining a function to square a number
fn square() {
  let mut no1=String::new();
  println!("enter  number" );
  io::stdin().read_line(&mut no1).ok();
  let no1: f64=no1.trim().parse().unwrap();
 
  

  println!("Result is {}",no1.powi(2) );
 
  
}

/// Initializing and defining a function to cube a number
fn cube() {
  let mut no1=String::new();
  println!("enter  number" );
  io::stdin().read_line(&mut no1).ok();
  let no1: f64=no1.trim().parse().unwrap();
 
  

  println!("Result is {}",(no1*no1*no1) );
 
  
}

/// Initializing and defining a a function to display the square root of a number
fn squareroot() {
  let mut no1=String::new();
  println!("enter  number" );
  io::stdin().read_line(&mut no1).ok();
  let no1: f64=no1.trim().parse().unwrap();
 
  

  println!("Result is {}",no1.sqrt());
 
  
}

/// Initializing and defining a function to display the module of a number
fn modules() {
  let mut no1=String::new();
  println!("enter  number" );
  io::stdin().read_line(&mut no1).ok();
  let no1: f64=no1.trim().parse().unwrap();
 let mut no2=String::new();
  println!("enter  number" );
  io::stdin().read_line(&mut no2).ok();
  let no2: f64=no2.trim().parse().unwrap();
 
  

  println!("Result is {}",no1%no2);

 
  
}

/// Initializing and defining a function to display the cube root of a number
fn cuberoot() {
  let mut no1=String::new();
  println!("enter  number" );
  io::stdin().read_line(&mut no1).ok();
  let no1: f64=no1.trim().parse().unwrap();
 
 
  

  println!("Result is {}",no1.cbrt());

 
  
}

/// Initializing and defining a function to display the log of a number
fn log() {
  let mut no1=String::new();
  println!("enter  number" );
  io::stdin().read_line(&mut no1).ok();
  let no1: f64=no1.trim().parse().unwrap();
 
 
  

  println!("Result is {}",no1.log10());

 
  
}

/// Initializing and defining a function to display the sin value of a number
fn sinfun() {
  let mut no1=String::new();
  println!("enter  number" );
  io::stdin().read_line(&mut no1).ok();
  let no1: f32=no1.trim().parse().unwrap();
  
 
  

  println!("Result is {}",no1.sin());

 
  
}

/// Initializing and defining a function to display the cosin value of a number
fn cosfun() {
  let mut no1=String::new();
  println!("enter  number" );
  io::stdin().read_line(&mut no1).ok();
  let no1: f32=no1.trim().parse().unwrap();
 
 
  

  println!("Result is {}",no1.cos());

 
  
}

/// Initializing and defining a function to display the tangent value of a number
fn tanfun() {
  let mut no1=String::new();
  println!("enter  number" );
  io::stdin().read_line(&mut no1).ok();
  let no1: f64=no1.trim().parse().unwrap();
 
 
  

  println!("Result is {}",no1.tan());

 
  
}

/// Initializing and defining a function to display the factorial result of a number
fn factorial() {
  let mut no1=String::new();
  println!("enter  number" );
  io::stdin().read_line(&mut no1).ok();
  let mut no1: i32=no1.trim().parse().unwrap();
  let mut fact=1;
while no1>0 {
    fact=fact*no1;
    no1=no1-1;
}
println!("Factorial of the number is {} ",fact);
}
 
 
  

  

 
  






