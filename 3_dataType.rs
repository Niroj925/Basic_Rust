fn main(){

    let sc:char;
    sc='$';
    println!("char:{sc}");
    // valid="ok"; //string is not assignable

    let signed_int:i32=-200;//this is unsigned variable take all +ve and -ve values
    println!("{signed_int}");
    let unsigned_int:u32=300;//take only positive value
    println!("{unsigned_int}");

    let float_value:f32=21.24445;
    println!("{float_value}");

    let is_valid:bool=false;
    println!("{is_valid}");


   let inference_value=32;//by default it is i32
   println!("{inference_value}");
}