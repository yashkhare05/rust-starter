// OWNERSHIP
// Set of rules that govern how a Rust program manages memory
// ownership is meant for things stored on heap

// Stack Variables (easy to define)
fn main(){
  let x = 1;
  let y=3;
  println("{}",sum(x,y));
}

fn sum(a:i32,b:i32)-> i32{
  let c : i32 = a+b;
  return c;
}

// Heap Variables(want to have single owner if owner dies we die)
// when owner dies go out of scope
fn main(){
  let s1 = String::from("hello");
  let s2 = s1; //move to s2, dont copy hello, s1 will die  
  println("{}",s1) // error as s1 has died
}

// Another example

fn main(){
  let my_string = String::from("hello");
  takes_ownership(my_string);
  println("{}",my_string) // will give error as my_string has died ownership moved to another function
}

fn takes_ownership(some_string: String){
  println!("{}",some_string)
}