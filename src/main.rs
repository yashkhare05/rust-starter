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
}