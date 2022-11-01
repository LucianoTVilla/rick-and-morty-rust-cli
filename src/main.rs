use std;
use reqwest;
use serde::{Deserialize, Serialize};
use dialoguer::{theme::ColorfulTheme, Select, Input};

#[derive(Serialize, Deserialize, Debug)]
pub struct CharactersResponse {
    #[serde(rename = "info")]
    info: Info,

    #[serde(rename = "results")]
    results: Vec<Character>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Info {
    #[serde(rename = "count")]
    count: i64,

    #[serde(rename = "pages")]
    pages: i64,

    #[serde(rename = "next")]
    next: String,

    #[serde(rename = "prev")]
    prev: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Character {
    #[serde(rename = "id")]
    id: i64,

    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "status")]
    status: Status,

    #[serde(rename = "species")]
    species: Species,

    #[serde(rename = "type")]
    result_type: String,

    #[serde(rename = "gender")]
    gender: Gender,

    #[serde(rename = "origin")]
    origin: Location,

    #[serde(rename = "location")]
    location: Location,

    #[serde(rename = "image")]
    image: String,

    #[serde(rename = "episode")]
    episode: Vec<String>,

    #[serde(rename = "url")]
    url: String,

    #[serde(rename = "created")]
    created: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "url")]
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Gender {
    #[serde(rename = "Female")]
    Female,

    #[serde(rename = "Male")]
    Male,

    #[serde(rename = "unknown")]
    Unknown,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Species {
    #[serde(rename = "Alien")]
    Alien,

    #[serde(rename = "Cronenberg")]
    Cronenberg,

    #[serde(rename = "Human")]
    Human,

    #[serde(rename = "Humanoid")]
    Humanoid,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Status {
    #[serde(rename = "Alive")]
    Alive,

    #[serde(rename = "Dead")]
    Dead,

    #[serde(rename = "unknown")]
    Unknown,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EpisodesResponse {
    #[serde(rename = "info")]
    info: Info,

    #[serde(rename = "results")]
    results: Vec<Episode>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Episode {
    #[serde(rename = "id")]
    id: i64,

    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "air_date")]
    air_date: String,

    #[serde(rename = "episode")]
    episode: String,

    #[serde(rename = "characters")]
    characters: Vec<String>,

    #[serde(rename = "url")]
    url: String,

    #[serde(rename = "created")]
    created: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LocationsResponse {
    #[serde(rename = "info")]
    info: Info,

    #[serde(rename = "results")]
    results: Vec<Locations>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Locations {
    #[serde(rename = "id")]
    id: i64,

    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "type")]
    result_type: String,

    #[serde(rename = "dimension")]
    dimension: String,

    #[serde(rename = "residents")]
    residents: Vec<String>,

    #[serde(rename = "url")]
    url: String,

    #[serde(rename = "created")]
    created: String,
}

const CHARACTERS_API_URL: &str = "https://rickandmortyapi.com/api/character";
const EPISODES_API_URL: &str = "https://rickandmortyapi.com/api/episode";
const LOCATIONS_API_URL: &str = "https://rickandmortyapi.com/api/location";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let selections = &[
        "Get all characters",
        "Get all episodes",
        "Get all locations",
        "Search a character by his name"
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose your option:")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    println!("You selected: {}", selections[selection]);

    if selection == 0 {
        let characters_response = reqwest::get(CHARACTERS_API_URL).await?;
        let characters: CharactersResponse = characters_response.json().await?;
        println!("{:#?}", characters);
    }
    
    if selection == 1 {
        let episodes_response = reqwest::get(EPISODES_API_URL).await?;
        let episodes: EpisodesResponse = episodes_response.json().await?;
        println!("{:#?}", episodes);
    }
    
    if selection == 2 {
        let locations_response = reqwest::get(LOCATIONS_API_URL).await?;
        let locations: LocationsResponse = locations_response.json().await?;
        println!("{:#?}", locations);
    }

    if selection == 3 {
        let input: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Type the character name: ")
            .interact_text()
            .unwrap();
        let custom_api_url = format!("{}/?name={}", CHARACTERS_API_URL, input);
        let character_response = reqwest::get(custom_api_url).await?;
        let character: CharactersResponse = character_response.json().await?;
        println!("{:#?}", character);
    }
    
    Ok(())
}