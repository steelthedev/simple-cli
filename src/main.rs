use clap::{Parser, Subcommand};
mod arithmetic;

#[derive(Parser)]
struct Value{
   #[clap(subcommand)]
   commands: Commands,
}


#[derive(Subcommand)]
enum Commands {
    Add { number_one: i32, number_two: i32 },
    Substract { number_one: i32, number_two: i32 },
}

fn main() {
    let value = Value::parse();

 match &value.commands {
     Commands::Add { number_one, number_two }=>{
        let answer = arithmetic::add::add_numbers(*number_one, *number_two);
        println!("The answer is {:?}", answer )
     },
     Commands::Substract { number_one, number_two } => {
        let answer = arithmetic::add::add_numbers(*number_one, *number_two);
        println!("The answer is {:?}", answer );
     }
 }
}