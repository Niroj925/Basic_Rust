fn main(){
    let mut age;

    age=18;
    if age>18{
        println!("you are valid");
    }else if age==18{
        println!("You are valid soon"); 
    }else{
        println!("you are not valid");
    }

    //nested conditions
    if age>17{
        if age==18{
            println!("valid soon");
        }else{
            println!("valid");
        }
    }
}