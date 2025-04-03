fn main(){
    //it is just like switch in another language

    let age=36;

    match  age {
        0..= 12 => println!("you are child."),
        13..= 19=>println!("you are teenager"),
        20..= 60=>println!("you are youth"),
        60..= u8::MAX =>println!("you are old"),
        _ => println!("uncategorized"),
    }

    //three varient of color
    enum Color{
        RED,
        GREEN,
        BLUE
    }

    let my_color=Color::BLUE;//to get the value from enum we use namespace :: it's use for struct method 

    match my_color{
        Color::RED =>println!("your favourite color is red"),
        Color::GREEN =>println!("your favourite color is green"),
        Color::BLUE =>println!("your favourite color is blue"),
    }


    //pattern match for struct type
    struct Point{
        x:i32,
        y:i32
    }

    let point=Point{x_val:35,y_val:53};
    match point{
        Point{x_val:0,y_val}=>println!("On x-axis {}",x_val),
        Point{x_val,y_val:0}=>println!("On y-axis {}",y_val),
        Point{x_val:0,y_val:0}=>println!("On point at({},{})",x_val,y_val),

    }

    //matching Option and Result Types

    //Option has 2 types=>None,Some(T)
    //so my_option variable is a Option type which may contain Some varient with i32 value or None varient 
     let my_option:Option<i32>=Some(123);
     //the match expression compare the value of my_option to Some and None varient then bind the Some varient to the value
     match my_option{
        Some(value)=>println!("value of the option is {}",value),//if match found console this 
        None=>println!("there is option value"),
     }

    //Result had also 2 types=> Ok(T),Err(T)


    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

    // fn divide(numer:i32,denomr:i32)->Result<i32,String>{
    //     if denomr==0{
    //         Err(println!("can not devide by 0"))
    //     }else{
    //         Ok(numer/denomr)
    //     }
    // }
    // let result=divide(4,2);
    let res:Result<i32,&str>=Err("error occure");
    match res{
        Ok(value)=>println!("value:{}",value),
        Err(e)=>println!("Error:{}",e),
    }

    //if let expression for shorthand of match for only one pattern to match
    let mero_option:Option<i32>=Some(2058);

    if let Some(value)=mero_option {
        println!("value:{}",value);
    }else{
        println!("the option has no value");
    }



}