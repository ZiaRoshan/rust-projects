fn main() {
    // In Rust you must use ;
    
    //Constant Variable
        let a = 25; //Constant
        println!("The value of constant a is - {}", a);

    //Mutable Variables
        let mut b = 32; //This is a simple variable
        println!("The value of b before change is - {}", b);
        b = 55; //You can chage it after making it mutable
        println!("The value of b after change is - {}", b);

    //Now lets Discuss the data types
    //Rust supports (Int, Float, Char, String)
    //The range can be from 8 upto 128 (8, 16, 32, 64, 128)
    //while using a datatype you dont have to define datatypes but its a good practice to do so

    //Functions
    println!("The function returned - {}", return_function()); //Here we called a function which can return value
    single_attribute_function("Hello my name is Zia"); //Here we called a function and passed a single parameter in it  
    multiple_attribute_function("Zia", "Roshan", 22); //Here we called a function and passed a multiple parameters in it 

    let funrets = single_attribute_function_with_return(2); //Here we stored the value the function returned into the variable
    println!("The Single attribute function return - {}", funrets);

    let funretm = multiple_attribute_function_with_return(4, 5); //Here we stored the value the function returned into the variable
    println!("The Multiple attributes function return - {}", funretm);

    let counted_itterations = loop_function(25); //Here we called the loop
    println!("The loop executed the statements {} times", counted_itterations);

    for_loop(); //Here we called for loop

    let while_itterations = while_function(12); //Here we called while loop
    println!("The while loop executed {} times", while_itterations);
}


//A Function which can return value

/*
    when defining a function you must define its return type
    fn [Function name] () -> [return datatype] {
        Statements
        // And function must contain a return
        return [Some value]
    }
*/

fn return_function() -> i32 {
    return 25;
}


// A Function which have a single attribute

/*
    fn [Function name] ([atrribute name]: [datatype]){ // You must define the attributes datatype
        Statements
    }
*/

fn single_attribute_function(attribute: &str) { //Here we dont have to define the return type because function is not returning value
    println!("{}", attribute);
}

// A Function which have multiple attributes

fn multiple_attribute_function(first_name: &str, last_name: &str, age: i32) { //We gave function multiple attributes here
    println!("Hello My name is {first_name} {last_name} and i am {age} years old"); //Here we passed the variable into the string directly
    /*
        You can do it like this too
        println!("Hello My name is {} {} and i am {} years old", first_name, last_name, age);
    */
}

// A Function which have single or multiple attributes and return a value

/*
    when defining a function you must define its return type
    fn [Function name] ([atrribute name]: [datatype]) -> [return datatype] { // You must define the attributes datatype
        Statements
    }
*/

fn single_attribute_function_with_return(singleattribute: i32) -> i32 {
    return singleattribute + 2; 
}


fn multiple_attribute_function_with_return(attribute1: i32, attribute2: i32) -> i32 {
    return attribute1 + attribute2;
}

//loop in a function

fn loop_function(itterations: i32) -> i32 {
    let mut i = 1; //Here we defined a simple variable for breaking the loop
    loop {
        if i > itterations{ //Condition to break the loop
            break;
        }
        println!("The value of i is - {}", i);
        i += 1; //Increment by one in i with every itteration
    }

    return itterations;
}

//for loop in a function

fn for_loop() {
    let x = [2, 5, 6, 18];
    for i in x {
        println!("The x on current index is - {}", i);
    }
}

//while loop in a function

fn while_function(itterations: i32) -> i32 {
    let mut i = 1; //Here we defined a simple variable for conditional loop

    while i <= itterations { //Condition when not satisfied the loop will be terminated
        println!("The value of i is - {}", i);
        i += 1; //Increment by one in i with every itteration
    }

    return itterations;
}
