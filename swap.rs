use std::io;
fn main() 
{
println!("enter the first number");
let mut num1 = String::new();

io::stdin()
.read_line(&mut num1)
.expect("failed to get input");


println!("enter the second number");
let mut num2 = String::new();

io::stdin()
.read_line(&mut num2)
.expect("failed to get input");

println!("The numbers are {} {}",num1,num2);

let mut temp = num1;
num1=num2;
num2 = temp;
println!("The swapped numbers are {} {}",num1,num2);
    
}
