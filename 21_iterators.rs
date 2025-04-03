//iteratior are the sequence of values which is iterable

fn main() {
    let num = [1, 2, 3, 5, 6, 7]; //this is primitiv type array

    for n in num {
        println!("{}", n);
    }

    //collection like Array,Vector,HashMap and HashSet are not iteratable by default
    //but in for loop which makes iterable 

    //1.iter method
    let mut number_iter = num.iter();

    //   for num in number_iter{
    //     println!("{}",num);
    //   }

    println!("{:?}", number_iter.next());
    println!("{:?}", number_iter.next());
    println!("{:?}", number_iter.next());

    let mut vect: Vec<&str> = Vec::new();
    vect.push("niroj");
    vect.push("thapa");
    vect.push("nirajan");
    // println!("{:?}",vect);
    // for name in vect.iter() {
    //     println!("{}", name);
    // }


    //2.iter_into method

    for name in vect.into_iter() {
        //the item collection moved into this scope
        //it mean the ownership is moved to name
        println!("{}", name);
    }


    // println!("{:?}", vect);//can not borrow it's ownership moved by into_iter()
    // to prevent this before move the ownership we can clone like this  for name in vect.clone().into_iter() {}

    //3.iter_mut method
    //this method on a collection will mutabally borrow each element of collection in each iteration
    //so this can be modified in the collection

    let mut colors=vec!["red","blue","green"];

    for color in colors.iter_mut(){
        *color="black";
        println!("{}",color);
    };

    println!("{:?}",colors);


    //iterator adaptor 
    //this is use to transform another type of iterator by altering it's behaviour

    let numbers:Vec<i32>=vec![1,2,3];
    //using the map iterator adaptor
    let even_numbers:Vec<i32>=numbers.iter().map(|i|i*2).collect();
    println!("{:?}",even_numbers);

    //loop through range
    for i in 1..5{
        println!("{}",i);
    }
}

