// use is basically import libraries
// std is a reference to the standard rust library
// ie is the specific library we are using to interface with the player
// rand is a referencen to the  rand crate, imported via dependencies toml
// this bit of the rust code is called a prelude ...very fancy
use rand::Rng;
use std::cmp::Ordering;
use std::io;
// start the main function, the entry point into the program
// it is called main and takes no parameters

fn main() {
    println!("Guess the nubmer:"); // macro to print string to console

    let secret_number = rand::thread_rng().gen_range(1..=100);
    //the gen_range method takes a
    // range and generates a random number from within that range.
    // Here the expression "start..=end" is used to define the range.
    // Not quite sure what thread_rng() is doing,  I believe it is action as a seed here
    loop {
        println!("Please input your guess."); // macro to print string to console
        let mut guess = String::new();
        //create a mutatable variable called guess which is a new empty string
        // here we are create a variable to store the user input with "let"
        // ie let apple = 5
        // because variables in rust are immutable (cannot be changed)
        // we add "mut" to indicate that this variable will change
        // We then name the variable "guess"
        // We then say that guess is equal to something now with the "="
        // this is called binding
        //this variable is equal to String::new()
        //"String" is a type defined in the standard library
        //"new()" is a function assosiated (::) with the "string" type
        //String::new() creates a new empty string
        io::stdin() // make a call to the function stdin() from the assosiated (::) io library
            .read_line(&mut guess) //read line is a method to read the input from the user
            //it takes a single property as an input, in this case our variable guess
            //we add it here as a reference to guess with the "&" but rather then pass &guess we need
            //to indicate that the reference itself may also change (is mutable) so add mut finising
            //with &mut guess
            .expect("Failed to read line");
        //expect handles the result output from the .read_line
        //method without it rust would not know what to do in the case of an error. The values from
        //read_line can be ok or err, if err we are telling rust to display "Failed to read line"
        let guess: u32 = guess.trim().parse().expect("Please type a number!");
        // when we create the variable guess we bound it to an empty string
        // however we will need to compare it to secret_number which is a u32
        // to do so we will have to make sure that guess is also the correct type
        // first we create a new variable with the same name (a shadow variable)
        // set its type as u32 and then bind it to a parsed verion of guess (currently a string)
        // here that is done by taking guess and adding:
        // .trim() to remove any whitespace
        // .parse() to convert a string (guess) into number - defined by the type (guess: u32)
        // .expect() to handle an error
        // we now have a variable, still called guess, but now of type u32
        println!("You guessed: {guess}"); //finally we print the value of our variable guess with some
                                          //flavour text
        match guess.cmp(&secret_number) {
            //here the method cmp is comparing two values, in this
            // guess and secret_number
            // it returns  a enumeration based on the ordering type.
            // we are then using the match expression to match that return and run specific code
            Ordering::Less => println!("Too small"), // if the return of cmp() matches
            //"Ordering::Less" then println  ("Too small")
            Ordering::Greater => println!("Too big!"), // if the return of cmp() matches
            //"Ordering::Greater" then println  ("Too big!")
            Ordering::Equal => {
                // if the return of cmp() matches
                //"Ordering::Equal" then println ("You win!") and break the loop
                println!("You win!");
                break;
            }
        }
    }
}
