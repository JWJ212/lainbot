use rand::Rng;
use std::io;
fn main() {
    //define variables
    let mut lsc = rand::thread_rng();
    let mut question = String::new();
    //print opening argument
    println!("Ask lain a question.");
    //start the loop
    loop {
        //generate the score
        let lainscore: i32 = lsc.gen_range(0..1110);
        //print the prompt and take input (not that you do anything with it really)
        io::stdin().read_line(&mut question).expect("Try again.");
        //quit argument
        if question.contains("quit") {
            println!("Goodbye.");
            break;
        }
        //decide yes or no by checking if lainscore is over 555
        if lainscore > 555 {
            println!("Yes.");
        } else {
            println!("No.");
        }
        println!("Anything else?")
    }
}
