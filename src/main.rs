fn main() {
  println!("Hello, world!");
  let mut x=4;
  println!("x is {}",x);
  x=5;
  println!("x is {}",x);

  let y=4;
  println!("y is {}",y);
  let y=5;
  println!("y is {}",y);

  let tup : (i32,bool,char) = (1,true,'a');
  println!("{}",tup.0);

  let arr : [i32;5] = [1,2,3,4,5];
  println!("{}",arr[2]);

  if x>y || 1<2{    
    println!("x is greater than y")
  }else{
    println!("dflkjklgjl")
  }

  for name in arr{
    println!("{}",name);
  }

  let name : String = String::from("Hello world");
  println!("{}",name);

  // println!("{}",name[0]); ->  will give an error because rust doesnt let you access maybe null points
}