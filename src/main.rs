use std::io;
use rand::prelude::*;


fn main() {
   let guess_list = ["apple","banana","pomegranate","mango","Kiwi","papaya"];
   loop {
    let mut rng = thread_rng();

    let index = rng.gen_range(0..guess_list.len());
    let random_fruit = guess_list[index];
    println!("Random Fruit:{}",random_fruit);
 
    let mut input = String::new();
 
     match io::stdin().read_line(&mut input){
         Ok(_)=>{
            let fruit_selected = input.trim().to_lowercase();
            println!("Fruit selected,{}",fruit_selected);
 
            if !guess_list.contains(&fruit_selected.as_str()){
               println!("Fruit entered does not found");
               continue;
            }
 
            if guess_checker(&fruit_selected,random_fruit){
                println!("Congratulations you are a winner!");
                break;
            } else {
                println!("You Lost it, Retry!");
            }
 
         } Err(error)=>{
            println!("Error,{}",error);
         }
       }
 }
   }
  

fn guess_checker(fruit_selected: &str, random_fruit: &str) -> bool{
    return fruit_selected == random_fruit;
}