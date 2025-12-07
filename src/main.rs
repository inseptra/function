fn add(a: i32, b: i32) -> i32{ // we must call the Return type(-> i32) or Rust Won't treat it
    a + b
}
fn main() {
  let q_1 =  add(9, 4);
  println!("{q_1}");

  let five_10 = 5 * 10;
  let q_2 = add(five_10, 19);

  println!("{q_2}");

  
  let decimal = 14.95;
  let holder = decimal;

  let q_3 = add(holder as i32, 32); // In my opinion I said this keeps abstraction 
  println!("{q_3}");

  /*way 2
    let decimal = 14.95;
  let holder = decimal as i32; // I honestly prefer writing like this as it helps people understand that you're converting a type from the get go(In my opinion)
 
  let q_3 = add(holder, 32);
  println!("{q_3}");
}
   */
 
}


/*
  
  let decimal = 13.92; 
  println!("{decimal}"); // if you want the decimal you need to crate a new var and stuff it in there 
  decimal as i32;// Rust is a Static type lang so it read it as an i32 not an f64(

  let q_3 = add(decimal, 43);
  println!("{q_3}");

  this DID work I don't Know why vs code Changed its mind This code will not run discard the information above the line)

*/