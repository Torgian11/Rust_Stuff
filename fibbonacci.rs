fn main() {
   println!("Fibbonacci:");
   println!("The fib for the {} place in fibbonacci is {}", 9, fibby(9));
}

fn fibby(n: i8) -> i16 {
   let mut a = 0;
   let mut b = 1;
   let mut c;

   if n == 0 { return a; }

   for _i in 1..n {
      c = a + b;
      a = b;
      b = c;
   }
   return b;
}
