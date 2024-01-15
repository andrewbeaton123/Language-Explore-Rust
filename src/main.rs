use std::io;

fn main() {
    
    println!("Guess the number !");

    println!("Please input the guess.");

    let mut guess: String = String::new();
    io::stdin() 
    .read_line( &mut guess)
    .expect("Failed ot read line");
    

    let int_guess =  guess.trim().parse().unwrap_or(0) ;

    println!("You double what you have  is  {}", sum(int_guess, int_guess));
    //sum(int_guess, int_guess))

    let half_int_guess : i32 = int_guess.checked_div(2).unwrap_or(0);
    
    println!("Divided by two that numver is {}",half_int_guess);
}


fn sum(n1: i32, n2 : i32) -> i32{

    // adds n1 and n2 and returns 
    n1 + n2
}