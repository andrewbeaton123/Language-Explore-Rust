use std::io;

fn main() {
    
    println!("Guess the number !");

    println!("Please input your integer guess.");

    let mut guess: String = String::new();
    io::stdin() 
    .read_line( &mut guess)
    .expect("Failed ot read line");
    

    let int_guess =  guess.trim().parse().unwrap_or(0) ;

    println!("You double what you have  is  {}", sum(int_guess, int_guess));
    //sum(int_guess, int_guess))
    let float_guess: f32 =  int_guess as f32 ;
    
    let half_flt_guess : f32 = float_guess/2 as f32;
    
    println!("Divided by two that numver is {}",half_flt_guess);

    if int_guess <=20  {
        let factorial_int_guess :i128 = factorial(int_guess as i128) ;

        println!("The factorial of your guess is {}", factorial_int_guess);
    }else {
        println!("The value entered would likely overflow an i128 an so no factorial                                                                           has been clalculated")
    }
    
    println!("The triple of that number is {}", list_sum(vec![int_guess,int_guess,int_guess])) ;

    print!("The prime factors of your value are {:?}", prime_factorization(int_guess)) ;
}

fn factorial(n_long: i128) -> i128{
    // returns the factorial of the entered number 
    let n_long: i128 = n_long as i128 ; 

    if n_long == 0 || n_long ==1{
        1 
    }else {
     n_long*factorial(n_long-1)
    }
}   

fn list_sum (v_n : Vec<i32>) -> i32 {
    
    let mut sum: i32  = 0; 
    for &num in &v_n {
        sum += num ; }
    sum 
} 


fn sum(n1: i32, n2 : i32) -> i32{

    // adds n1 and n2 and returns 
    n1 + n2
}

fn prime_factorization (mut n : i32) -> Vec<i32>{

    let mut factors = Vec :: new() ;
    let mut divisor = 2 ;

    while n > 1{
        while n % divisor ==0 {
            factors.push(divisor);
            n /=divisor ;
        }
        divisor +=1 ;
    }
    factors

}