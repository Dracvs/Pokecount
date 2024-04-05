#[macro_use] 
extern crate rocket;


use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use std::time::{Duration, Instant};


// #[derive(Deserialize)]
// #[serde(crate = "rocket::serde")]
// struct Poke_list{
//    items: Vec<Pokemon>
// }

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct PokeElement{
    name: String,
    url: String
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Pokemon{
    number : i32,
    name : String
}

#[get("/")]
fn index() -> &'static str {
    "Gotta Catch 'em all"
}

#[post("/pokecount", data = "<pokeelement>", format = "json")]
fn pokecount(pokeelement: Json<Vec<PokeElement>>) -> String {
    let start = Instant::now();
    
    let mut pokedex: Vec<Pokemon> = vec![];
    for number in 1..100_000{
        let mut counter = 1;
        for element in pokeelement.iter() {
            pokedex.push(Pokemon{number: counter, name: String::from(&element.name)});
            counter += 1;
        }
    } 

    let duration = start.elapsed().as_millis();   
    //println!("{}", pokeelement[0].name);
    format!("Gotta catch em all in: {:?} ms", duration)
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![pokecount])
}

// fn main() {
//     println!("Hello, world!");
// }
