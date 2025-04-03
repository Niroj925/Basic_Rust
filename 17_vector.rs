fn main() {
    //vector are the dynamic (resizable) data structure that can store list of elements of same type
    //vector are grow and shrink at run time

    //to create vectoe we use vec! macro

    let vect = vec![-1, 3, -4, 5]; //by default vector type is Vec<i32>
    println!("{:?}", vect);

    let vect1:Vec<u8> = vec![4, 5, 2, 6];
    println!("{:?}", vect1);

    //accessing vector element
    let colors=vec!["red","green","blue"];
    println!("first color:{}",colors[0]);
    //accessing the vector element using get method
    println!("c1:{:?}",colors.get(1));
    let color_option=colors.get(1);
    let cc= match color_option{
        Some(user)=>user,
        None=>"Not found",
    };
    println!("color:{:?}",cc);

    //adding value to a vector
    let mut age=vec![25,14,24];
    age.push(17);
    age.push(34);
    println!("{:?}",age);

    //remove values from vectors
    age.remove(2);
    println!("{:?}",age);

    //looping through vector
    let mut index=0;

    for color in colors{
        println!("index:{},color:{}",index,color);
        index+=1;
    };

    //alternativly we can create a empty vector using Vec::new()

    let mut v:Vec<i32>=Vec::new();
    println!("{:?}",v);//this gives empty vector [] type i32
    v.push(10);
    v.push(15);
    println!("{:?}",v);
}
