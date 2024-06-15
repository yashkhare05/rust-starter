// we have concept of borrowers for a variable(variable not dependent on borrower)

fn main(){
  let s1 = String::from("hello");
  let s2 = &s1;

  println!("{}",s2);
  println!("{}",s1); // both are valid statements as reference is passed just a borrower
}

fn main(){
  let mut s1 = String::from("hello");
  update_string(&mut s1);
  println!("{}",s1);
}

fn update_string(str: &mut String){
  str.push_str(" world");
}