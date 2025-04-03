fn main() {
    //create a float variable

    let float_val: f32 = 2.554;
    println!("{float_val}");
    //explicit type conversion
    let int_val = float_val as u8; //convert decimal to intiger value
    println!("{int_val}");

    let char_val: char = 'N';
    let ascii_val = char_val as u16;
    println!("{ascii_val}");

    let bool1: bool = true;
    let int_bool = bool1 as u32;
    println!("{int_bool}"); //it gives 1;

    let x = 'N';
    let x_val = x as u8;
    println!("{x_val}");

    let n:u8 = 65;
    let n_val = n as char;
    println!("64:{}", n_val);

    let is_open:bool=true;
    let b_val=is_open as i32;
    println!("is_open:{b_val}");

    //floating type can not be converted into character
    let ff:f32=66.45;
    // let f_val=ff as char;//this gives error so we have to convert into intiger type then can be convert
    // println!("{f_val}");
}
