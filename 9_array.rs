fn main(){
    let num=[1,2,3,4,6,7];
    println!("{:?}",num);

    let numbers:[i32;5]=[5,2,3,9,7];
    println!("{:?}",numbers);

    //initialization array with default value
    let da:[i32;5]=[6;5];//6 ,5 times repeat
    println!("{:?}",da);

    let color=["red","green","blue"];
    println!("{}",color[1]);

    //mutable array
    let mut mutnum:[i32;5]=[6,8,2,4,9];
    mutnum[2]=7;
    println!("{:?}",mutnum);

    //array on loop
    for index in 0..3{
        println!("index.{}:{}",index,color[index]);
    }
}