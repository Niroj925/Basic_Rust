//the unwrap and expect untility method to work with Result and Option type.
use std::num::ParseIntError;

fn main() {
    //1.Unwrap() method
    //In rust it return the result of the opration for Result and Option enums

    //function to find the user by username and return the result Option type
    fn get_user(username: &str) -> Option<&str> {
        if (username.is_empty()) {
            return None;
        }
        return Some(username);
    };

    //return option
    let option_user = get_user("Niroj");

    //use match expression to get the result out of the option
    let result = match option_user {
        Some(user) => user,
        None => "not found",
    };

    println!("{}", result);

    //use unwrap method to get the result of Option enum
    let res_name = get_user("Thapa").unwrap();
    // let res_name=get_user("").unwrap();//this unwrap method return panic
    println!("{}", res_name);

    //both panic and match gives the same result but unwrap method gives panic if return value is None

    //expect method
    //it is same as unwrap method with addition of custom panic message as an argument
    let result = get_user("Kaji").expect("fetch user");
    // let result=get_user("").expect("fetch user");//return panic messsage e
    println!("{:?}", result);

    //the question mark operator ?
    //short hand for return the result and it can only apply for Result<T,E> abd Option<T> type;

    //function to parse an intiger
    fn parse_int() -> Result<i32, ParseIntError> {
        let age: i32 = "23".parse()?; //value is unwrap

        let roll: i32 = "34y".parse()?; //return error immediately

        return Ok(age + roll); //does not reach this line
    }

    let parsed_val = parse_int();
    println!("{:?}", parsed_val);//this is the better way to handle error in cleaner and easier to read
}
