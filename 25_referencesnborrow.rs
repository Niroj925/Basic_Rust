//reference allow us to point a resource (value) without owning it
//the original resource of remain same

//reference is helpful when pass value to function where we do not change the ownership
//creating refernce is borrowing in rust

fn main() {
    let str = String::from("Namaste");

    //call function with reference String value
    let len = calc_len(&str);

    println!("length of string:{}", len);

    let mut strr = String::from("Hello ");
    //pass mutable string as reference
    change_str(&mut strr);
    println!("{}", strr);

    //there is only one mutable reference value at a time
    //mutable references
    let  ref1 = &mut strr;

    // let ref2 = &mut strr; //can not borrow this mutable reference already taken by ref1

    // println!("{},{}",ref1,ref2);
}

//this take a reference of string as an argument
fn calc_len(s: &String) -> usize {
    //s is reference to string and it does not take the ownership of actual string value
    s.len()
}
//after s out of scope it's value does not drop because it does not have ownership value

//the action of creating reference is known as borrowing

//borrowing mean borrow sth and done sth and return back
//it does not make us the owner of the data

//& represent reference which refer to some value without taking ownership

//Modifying the reference
//by default reference are immutable
//to make reference mutable we have & mut the reference

fn change_str(s: &mut String) {
    s.push_str("nepal.");
}
