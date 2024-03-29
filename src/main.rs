use rustemon::pokemon::pokemon;
use rustemon::client::RustemonClient;
use rustemon::Follow;

#[tokio::main(flavor = "current_thread")] // unsure what this does
async fn main() {
    let mut user_team: Vec<PokemonData> = Vec::new();
    let mut foe_team: Vec<PokemonData> = Vec::new();

    println!("Your team:");
    user_team.push( load_pokemon(681).await );
    user_team.push( load_pokemon(609).await ); 
    user_team.push( load_pokemon(94).await );

    println!("Enemy team:");
    foe_team.push( load_pokemon(620).await );
    foe_team.push( load_pokemon(766).await );
    foe_team.push( load_pokemon(903).await );

    // lv100 will be assumed for my sake
    load_battle_team(user_team);
}

async fn load_battle_team(team: Vec<PokemonData>) -> Vec<BattlingPokemon> {
    let mut battle_team: Vec<BattlingPokemon> = Vec::new();

    for i in 0..team.len() {
        // HP = (0.01(2*Base+IV+(0.25*EV))*Level)+Level+10
        battle_team[i].hp = (0.01*(2*team[i].hp as f64+31*100)+100+10).floor();

        battle_team[i].att = (0.01*(2*team[i].att+31)*100+5).floor();
        battle_team[i].def = (0.01*(2*team[i].def+31)*100+5).floor();
        battle_team[i].spa = (0.01*(2*team[i].spa+31)*100+5).floor();
        battle_team[i].spd = (0.01*(2*team[i].spd+31)*100+5).floor();
        battle_team[i].spe = (0.01*(2*team[i].spe+31)*100+5).floor();
    }

    battle_team
}

async fn load_pokemon(number: i64) -> PokemonData { // this should return a Pokemon struct
    // this reads from the pokéAPI because i'm a lazy cunt 
    let rustemon_client = RustemonClient::default();

    let pokemon_data = pokemon::get_by_id(number, &rustemon_client).await;

    let species = &pokemon_data.as_ref().unwrap().species.name;
    let types =  &pokemon_data.as_ref().unwrap().types;
    let stats = &pokemon_data.as_ref().unwrap().stats;

    let mut pokemon_obj; // need to declare this out here otherwise it disappears at the end of the
    // if statement
    if types.len() > 1 { // TODO: add abilities you dumbass
    // it would be cooler if it could be done within the struct declaratio but oh well
        pokemon_obj = PokemonData {
            name: String::from(species),
            type1: types[0].type_.name.clone(), // must be cloned because it can't be copied - why?
            type2: types[1].type_.name.clone(), // otherwise compiler shits itself
            ability: "".to_string(),

            hp: stats[0].base_stat.clone(),
            att: stats[1].base_stat.clone(),
            def: stats[2].base_stat.clone(),
            spa: stats[3].base_stat.clone(),
            spd: stats[4].base_stat.clone(),
            spe: stats[5].base_stat.clone(),
        };
    } else {
        pokemon_obj = PokemonData {
            name: String::from(species),
            type1: types[0].type_.name.clone(),
            type2: "".to_string(),
            ability: "".to_string(),

            hp: stats[0].base_stat.clone(),
            att: stats[1].base_stat.clone(),
            def: stats[2].base_stat.clone(),
            spa: stats[3].base_stat.clone(),
            spd: stats[4].base_stat.clone(),
            spe: stats[5].base_stat.clone(),
        };
    }

    // parsing stuff
    pokemon_obj.name = remove_quotes(pokemon_obj.name);
    

    println!("Loaded Pokémon {:#?} successfully!", pokemon_obj.name);
    pokemon_obj
}

fn remove_quotes(mut string: String) -> String {
    string = string.trim_end_matches("\"").to_string();
    string = string.trim_start_matches("\"").to_string();

    // return!!
    string
}

// STRUCTS  

#[derive(Debug)]
struct PokemonData {
    name: String,
    type1: String,
    type2: String,
    ability: String,

    hp: i64,
    att: i64,
    def: i64,
    spa: i64,
    spd: i64,
    spe: i64,

    // all natures will be assumed as neutral. given it's just an added hassle and i have no
    // intention of adding any of it
}

#[derive(Debug)]
struct BattlingPokemon { // this is an instance of a pokémon in battle

    // its current stats
    hp: f32,
    att: f32,
    def: f32,
    spa: f32,
    spd: f32,
    spe: f32,

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
    item: String,

    // TODO: add stuff like stat boosts and volatile statuses
}
