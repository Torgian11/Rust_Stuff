fn main() {
   println!("Converting temperatures!");
   let f_temp = 101;
   let c_temp = 31;
   println!("{} Farenheight is {}C", f_temp, convert_ftoc(f_temp));
   println!("{} Celcius is {}F", c_temp, convert_ctof(c_temp));
}

fn convert_ftoc(degrees: i16) -> i16 {
   let conversion = (degrees - 32) * 5/9;
   conversion
}

fn convert_ctof(degrees: i16) -> i16 {
   let conversion = (degrees * (9/5)) + 32;
   conversion
}
