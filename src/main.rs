use rustemon::pokemon::pokemon;
use rustemon::client::RustemonClient;
use rustemon::Follow;

#[tokio::main(flavor = "current_thread")] // unsure what this does
async fn main() {
    load_pokemon(99).await; // must be ended by .await otherwise nothing happens
}

async fn load_pokemon(number: i64) { // this should return a Pokemon struct
    // this reads from the pokéAPI because i'm a lazy cunt 
    let rustemon_client = RustemonClient::default();
    let pokemon = pokemon::get_by_id(number, &rustemon_client).await;

    let species = pokemon.unwrap().species;
    println!("{:#?}", species);
}

// STRUCTS  

#[derive(Debug)]
struct PokemonData {
    number: i32,
    type1: String,
    type2: String,
    ability: String,
    nature: i8,
}

#[derive(Debug)]
struct BattlingPokemon { // this is an instance of a pokémon in battle
    // immutable stuff that is nevertheless referenced
    number: i32,

    // its current stats
    hp: i32,

    move1: String,
    move1_pp: i32,
    move2: String,
    move2_pp: i32,
    move3: String,
    move3_pp: i32,
    move4: String,
    move4_pp: i32,

    status: String,

    is_active: bool,

    // TODO: add stuff like stat boosts and volatile statuses
}
