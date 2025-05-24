use std::fs::File;
use std::io::prelude::*;
use std::env;
use std::io;
use std::io::stdin;
use std::collections::HashMap;
extern crate rand;
use rand::Rng;
mod multiple_files_function;
extern crate regex;
use regex::Regex;
use serde::Serialize;
use serde_derive::Serialize;
extern crate reqwest;
use std::process::Command;
extern crate serde_json;
use serde_json:: Value as JsonValue;
extern crate serde;
use serde_derive::Deserialize;
fn variables_mutables_prints(){
    let mut average_iq= 121;
    let mut albert_einstein_iq= 110;
    println!("The average IQ is not {}. Albert Einstein's IQ is  not {}. Lets fix them both!", average_iq, albert_einstein_iq);
    average_iq= 100;
    albert_einstein_iq= 160;
    println!("That does it! The average IQ is now {}. Albert Einstein's IQ is widely recognized as {}.", average_iq, albert_einstein_iq);
}
fn unicode(){
    println!("The character is {}.", '\u{221F}');
}
fn reference(){
    let mut x = 52;
    let mut xr = &mut x;
    *xr += 5;
    println!("{}",x);
}
fn arguments_functions_returns(num: u32) -> u32{
    println!("{}", num-35);
    return num;
}
fn tuples(){
    let tupl= (736,827,3729,"Jack is the best.", (4,3,7338));
    println!("{}", tupl.4.2);
}
fn infinite_loops(){
    let mut n = 0;
    loop{

        n+=1;
        if n>10{
            break;
        }
        if n==7{
            continue;
        }
        println!("The value of n is {}.", n)
    }
}
fn for_loops(){
    let animals = vec!["Human", "Rabbits", "Monkeys", "Apes", "Chimpanzees"];
    for (index,a) in animals.iter().enumerate(){
        println!("The animal is {}.", a);
        if index == 5{
            break;
        }
    }
    let animals = vec!["Human", "Rabbits", "Monkeys", "Apes", "Chimpanzees"];
    for a in animals.iter(){
        println!("The animal is {}.", a);
    }
    let numbers = 30..51;
    for i in numbers{
        println!("The value of i is {}.", i);
    }
    for i in 1..501{
        println!("The number is {}.", i);
    }
}
fn while_loops(){
    let mut n = 0;
    while n <= 45{
        n+=5;

        println!("The value of n is {}.", n);
    }
}
fn else_if_elif(){
    let n = 100;
    if n<30{
        println!("I never n was smaller than 30!");
    }
    else if 85 == n+5{
        println!("That's obvious.");
    }
}
fn constants(){
    const MAXIMUM_NUMBER: u8 = 21;
    for n in 1 ..MAXIMUM_NUMBER{
        println!("N is {}.", n)
    }
}
fn enum_types(){
    enum Direction{
        Up,
        Down,
        Left,
        Right
    }
    let player_direction:Direction = Direction::Up;
    match player_direction{
        Direction::Up=> println!("The plane is ascending."),
        Direction::Left=> println!("The plane is veering left."),
        Direction::Right=> println!("The plane is veering right."),
        Direction::Down=> println!("The plane is crashing!"),

    }
}
fn structs(){
    struct Color{
        red: u8,
        green: u8,
        blue: u8
    }
    let mut bg= Color {red: 255, green:0, blue:127};
    println!("{},{},{}", bg.red,bg.green,bg.blue);
    bg.green= 255;
    println!("{},{},{}", bg.red,bg.green,bg.blue);
}
fn _impl(){
    struct Rectangle{
        width: u32,
        height:u32
    }
    impl Rectangle{
        fn print_description(&self){
            println!("width:{} height:{}", self.width, self.height);
        }
        fn is_square(&self) -> bool {
            self.width == self.height
        }
    }
    let my_rect = Rectangle {width: 5, height:4};
    my_rect.print_description();
    println!("The fact that the rectangle is a square is {}.", my_rect.is_square());
}
fn tuple_structs(){
    struct Color(u8,u8,u8);
    let mut red = Color(255,0,0);
    println!("{},{},{}", red.0, red.1, red.2);
    red.2= 60;
    println!("{},{},{}", red.0, red.1, red.2);

}
//Start of pass by reference
struct Color{
    red: u8,
    green:u8,
    blue:u8
}
fn pass_by_reference(c:&Color){

    println!("Color - R:{}, G:{}, B:{}", c.red, c.green,c.blue)
}
//End of pass by reference
fn array(){
    let numbers= [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31];
    println!("{}", numbers[30])
    /* 
    let numbers= [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30];
    for n in numbers.iter(){
        println!("{}", n);
    }
    */
}
fn impl_traits(){
    impl ToString for Person{
        fn to_string(&self) -> String {
            return format!("My name is {} and I am {} years old.", self.name, self.age)
        }
    }
    struct Person {
        name:String,
        age:u8
    }
    let jack = Person {name: String::from("Jack"), age: 20};
    println!("{}", jack.to_string());
}
fn vectors(){
    let mut my_vector = vec![1,2,3,4];
    println!("{}",my_vector[2]);
    my_vector.push(5);
    println!("{}", my_vector[4]);
    for number in my_vector.iter(){
        println!("{}", number);
    }

}
fn read_file(){
    let mut file = File::open("/Users/jackvijaymane/HelloWorldRust/jack/ivost.txt").expect("Cannot open file.");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Oops! Cannot read the file.");
    println!("File contents: \n {}", contents);
    
}
fn strings(){
    let mut my_string = String::from("My name is Jack, and I am the best.");
    println!("Length: {}", my_string.len());
    println!("String is empty? {}",my_string.is_empty());
    for token in my_string.split_whitespace(){
        println!("{}", token);
    }
    println!("Does it contain Jack and best? The fact that it contains Jack is {}, and the fact it contains best is {}.", my_string.to_lowercase().contains("jack"), my_string.to_lowercase().contains("best"));
    my_string.push_str(" Sia is the worst.");
    println!("{}", my_string)
}

fn command_line_arguments(){
    let args: Vec<String> = env::args().collect();
    for arguments in args.iter(){
        println!("{}, ", arguments)
    }
}
fn writing_to_a_file(){
    let mut file = File::create("output.txt").expect("Cannot create file.");
    file.write_all(b"This is a example!").expect("Cannot write to file.")
}
fn define_traits(){
    struct Person{
        name:String,
        age:u8
    }
    trait HasVoiceBox {
        fn speak(&self);
        fn can_speak(&self) -> bool;
    }
    impl HasVoiceBox for Person {
        fn speak(&self) {
            println!("Hello, my name is {}.", self.name);
        }
        fn can_speak(&self) -> bool {
            if self.age >1 {
                return true;
            } return false;

        }
    }
    let person = Person {
        name: String::from("Albert Einstein"),
        age: 20
    };
    println!("Can {} speak? {}", person.name,person.can_speak());
}
fn switch_statement(){
    let number =5;
    /*match number {
        1=> println!("It is 1!"),
        2...20 =>  println!("It is greater than 1!"),
        _ => println!("It is not 1 or 2!")
    }*/
    let name = "Albert Einstein";
    match name {
        "Albert Einstein" => println!("Aren't you the person who discovered relativity?"),
        "Isaac Newton" => println!("Aren't you the person who discovered gravity?"),
        _ => println!("Nice to meet you!"),

    }
}
fn read_input(){
    let mut input = String::new();
    println!("What is your name? \n");
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("Hello, {}!", input);
        }
        Err(e) => println!("Oops! Something went wrong: {}", e)
    }
}

fn hash_maps(){
    let mut marks = HashMap::new();
    // Add values
    marks.insert("Math", "A++");
    marks.insert("Reading", "A++");
    marks.insert("Writing", "D++");
    marks.insert("Social Studies", "A-");
    marks.insert("Science", "A++");
    // Find length of HashMap
    println!("How many subjects do you have? {}", marks.len());
    //marks.remove("Reading");

    match marks.get("Math"){
        Some(mark) => println!("You got {} for math!", mark),
        None => println!("You didn't study math!")
    }
    for (subject, mark) in &marks{
        println!("For {} you got {}", subject, mark);
    }
    println!("Did you study Reading? {}", marks.contains_key("Reading"));
}
fn random_nubers_or_booleans(){
    /* 
    let mut which_number_is_it_on = 0;
    while which_number_is_it_on <= 10{
        let random_number = rand::thread_rng().gen_range(1,11);
        println!("The random number is {}.", random_number);
        which_number_is_it_on += 1;
    }
    */
    let mut which_number_is_it_on = 0;
    while which_number_is_it_on <= 4{
        let random_bool = rand::thread_rng().gen_weighted_bool(2);
        println!("The random boolean is {}.", random_bool);
        which_number_is_it_on += 1;
    }

}
fn string_replace() {
    let my_string = String::from("This is a cdoe dump!");
    println!("{}", my_string.replace("cdoe", "code"));
}

fn string_lines() {
    let my_string = String::from("This is a code dump!\nThis is a code dump!\nThis is a code dump!");
    println!("{}", my_string);
}

fn string_split() {
    let my_string = String::from("This+is+a+code+dump!");
    let tokens: Vec<&str> = my_string.split("+").collect();
    let index = 2;
    println!("At word {} is {}",index, tokens[index-1]);
    for token in tokens {
        println!(" {}", token);
    }
}


fn string_char_index() {
    let my_string = String::from("This is a code dump!");
    match my_string.chars().nth(4) {
        Some(c) => println!("Character at index 1: {}", c),
        None => println!("No character at index 4"),
    }
}

fn multiple_files(){
    multiple_files_function::print_message();
}
fn regex(){
    let re = Regex::new(r"(\w{5})").unwrap();
    let text = "Jack";
    println!("Found match? {}", re.is_match(text));
    match re.captures(text) {
        Some(caps) => println!("Found match: {}", caps.get(0).unwrap().as_str()),
        None => println!("Couldn't find a match")
    }
}
fn modules(){
    mod jack{
        use std::println;
        fn food(){
            println!("Food!")
        }
        pub fn print_message(){
            println!("This is a code dump!");
            food();
        }
        pub mod water{
            pub fn water_reminder(){
                println!("Drink water to stay hydrated!");
            }
        }
    }
    jack::print_message();
    jack::water::water_reminder();
}
fn enums(){
    /*let name = String::from("Isaac");
    println!("character at index 0: {}", match name.chars().nth(0) {
        Some(c) => c.to_string(),
        None => "No character at index 0!".to_string()
    });
    */
    fn get_occupation(name: &str) -> Option<&str>{
        match name {
            "Albert Einstein" => Some("The founder of relativity!"),
            "Isaac Newton" => Some("The founder of gravity!"),
            _ => None
        }
    }
    println!("He/She is the {}", match get_occupation("Jack") {
        Some(ivostw) => ivostw,
        None => "I don't know that person."
    })
}
#[tokio::main] // Required to run async function
async fn http() {
    /* 
    match reqwest::get("https://example.com").await {
        Ok(response) => {
            if response.status() == reqwest::StatusCode::OK {
                match response.text().await {
                    Ok(text) => println!("Response text: {}", text),
                    Err(_) => println!("Could not read response text."),
                }
            } else {
                println!("Response was not successful.");
            }
        }
        Err(_) => println!("Couldn't make request"),
    }
    */
    let response = reqwest::get("https://chatgpt.com/c/678e5520-27e0-800a-a3ec-37e4344b7498")
        .await
        .expect("Could not make request.");

    let response_text = response
        .text()
        .await
        .expect("Could not read response text.");
    println!("{}", response_text);
}
#[allow(dead_code)]
fn enum_methods(){
    enum Day {
        Monday, Tuesday, Wednesday, Thursday, Friday, Saturday, Sunday
    }
    impl Day {
        fn is_weekday(&self) -> bool{
            match self {
                &Day::Saturday | &Day::Sunday => return false,
                _ => return true
            }
        }
    }
    let d = Day::Sunday;
    println!("Is d a weekend? It is {} that it is.", d.is_weekday())
}
fn execute_cmd_commands(){
    let mut cmd = Command::new("python3");
    cmd.arg("./execute_cmd_command.py");
    match  cmd.output() {
        Ok(o) =>{
            unsafe {
                println!("Output: {}", String::from_utf8_unchecked(o.stdout));
            }
            

        }
        Err(e) => {
            println!("There was an error! {}", e)
        }
    }
}



fn parse_json(){
    #[macro_use]
    #[derive(Serialize, Deserialize)]
    struct  Person {
        name: String,
        age: u8,
        gender:String,
    }
    let json_str = r#"
    {
    "name": "Jack",
    "age": 20,
    "gender": "Male"
    }
    "#;

    let res = serde_json::from_str(json_str);
    if res.is_ok(){
        let p: Person = res.unwrap();
        
        println!("The person's name is {}. {} is {} years old. {} is a {}. ", p.name,p.name, p.age, p.name, p.gender);
    }
    else {
        println!("Couldn't parse json :(")
    }
}
fn main(){
    //unicode();
    //parse_json();
   //execute_cmd_commands();
    //enum_methods();
    //http();
    //enums();
    //modules();
    //regex();
    //multiple_files();
    //string_replace();
    //string_lines();
    //string_split();
    //string_char_index();
    //random_nubers_or_booleans();
    //hash_maps();
    //read_input();
    //switch_statement();
    //define_traits();
    //writing_to_a_file();;
    //command_line_arguments();
    //read_file();
    //vectors();
    //strings();
    //_impl();
    //array();
    //Start of pass by reference
    /*{
        let blue = Color {red: 0, green:0, blue: 255 };
        pass_by_reference(&blue);
    }
    */
    //End of pass by reference
    //tuple_structs();
    //structs();
    //reference();
    //let hi = arguments_functions_returns(50);
    //tuples();
    //constants();
    //enum_types();
    //for_loops();
    //while_loops();
    //infinite_loops();
    //variables_mutables_prints();

}
