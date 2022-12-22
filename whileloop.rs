fn main(){
 let mut i = 0;
 while i < 10 {
  println!("{}", i);
  i += 1;
 }

 let mut i = 0;
 loop {
  println!("{}", i);
  i += 1;
  if i >= 10 {
    break;
   }
  }
}
