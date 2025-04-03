fn main(){
//slice is a portion of stored collection like array,vector,string
let numbers=[2,3,6,7,1,9,5];
let sn=&numbers[1..5];
println!("{:?}",numbers);
println!("{:?}",sn);

//omit index
let ss=&numbers[..4];
println!("{:?}",ss);

let es=&numbers[3..];
println!("{:?}",es);

//reference the array with omit start and end both
let sea=&numbers[..];
println!("{:?}",sea);//return all 

//mutable slice
let mut colors=["red","green","blue","purple","yellow"];
println!("{:?}",colors);

let sc=&mut colors[1..4];
println!("{:?}",sc);

sc[2]="pink";
println!("{:?}",sc);

let mut string= String::from("Hello Nepal.");
// let mut string="Hello gaich";
let sls=&mut string[1..5];
println!("{}",sls);

// sls[3]="r";
// println!("{}",sls);

let vectors=vec!['a','e','o','u'];
let sv=&vectors[1..3];
println!("{:?}",vectors);
println!("{:?}",sv);

}