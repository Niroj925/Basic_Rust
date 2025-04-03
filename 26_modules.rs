//modules helps to split the program for better readability and organize

//modules is a collection of struct,function and other module too

//mod keywors use to define the module

mod config{
    //items inside module can be private or public
    //by default module is private 
    fn print(){
        println!("Namaste Nepal from config module.");
    }

    //for publicly accessible we use pub keyword
    pub fn show(){
        println!("This is public item of config module.");
    }
}

mod player{
    fn focus(){
        println!("player is focused");
    }

  pub  fn shift(){
        println!("player shift forward");
    }

  pub  fn Jump(){
      focus();//private but can be accessed inside module
      shift();
      println!("player is jumping");
    }
}

//nested modules
pub mod collection{
   pub mod play{
    pub fn dance(){
        println!("This is dance fn from nested module");
    }
   } 
}

//use keyword to bring items inside the module into current scope
//helps to call the function by their name no need to give full path after one time use it

use collection::play::dance;//bring the dance function inside scope

fn main(){
    //call modules
    //public items of modules can be access from outside module
    config::show();
    // :: operation use to seprate module name to item to call inside module

    // config::print();//this is private can not accessible

    player::Jump();

    //cal nested module
    // collection::play::dance();//
    dance();
}