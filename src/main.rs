extern crate dotenv;

use bson::{self, document, Document};
use dotenv::dotenv;
use mongodb::bson::{doc, Bson};
use mongodb::{options::ClientOptions, Client};
use reqwest::Error;
use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use std::env;
use url::form_urlencoded::{byte_serialize, parse};

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Rating {
    Source: String,
    Value: String,
}

#[derive(Debug)]
struct MovieSingle {
    id: i32,
    name: String,
    year: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Movie {
    Title: String,
    Year: String,
    Rated: String,
    Released: String,
    Runtime: String,
    Genre: String,
    Director: String,
    Writer: String,
    Actors: String,
    Plot: String,
    Language: String,
    Country: String,
    Awards: String,
    Poster: String,
    Ratings: Vec<Rating>,
    Metascore: String,
    imdbRating: String,
    imdbVotes: String,
    imdbID: String,
    Type: String,
    DVD: String,
    BoxOffice: String,
    Production: String,
    Website: String,
    Response: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();

    let apiKeyTuple: Vec<(String, String)> = (env::vars())
        .filter(|d| d.0 == "movieAPIKey")
        .take(1)
        .collect();
    println!("{:?}", apiKeyTuple[0]);
    let apiKey = &apiKeyTuple[0].1;

    let conn = Connection::open("movies.db").unwrap();
    let request_url = format!("http://www.omdbapi.com/?i=tt3896198&apikey={}", apiKey);

    println!("{}", request_url);
    let response = reqwest::get(&request_url).await?;
    let mut stmt = conn.prepare("SELECT * FROM Movies").unwrap();
    let person_iter = stmt
        .query_map(params![], |row| {
            Ok(MovieSingle {
                id: row.get(0).unwrap(),
                name: row.get(1).unwrap(),
                year: row.get(2).unwrap(),
            })
        })
        .unwrap();

    for movie in person_iter {
        let single_movie = movie.unwrap();
        let movie_id = single_movie.id;
        let movie_title = single_movie.name;

        call_movie_api(movie_title, apiKey).await.unwrap();
        // println!("Found Movie title {:?}", movie_title);
    }

    let users: Movie = response.json().await?;
    println!("{:?}", users);
    Ok(())
}

async fn call_movie_api(title: String, apiKey: &String) -> Result<(), Error> {
    let clean_movie = clean_text(title);
    let request_url = format!(
        "http://www.omdbapi.com/?t={}&apikey={}",
        clean_movie, apiKey
    );
    println!("{}", request_url);
    let response = reqwest::get(&request_url).await;
    match response {
        Err(why) => print!("hello"),
        Ok(res) => {
            println!("bye");
            let users: Result<Movie, Error> = res.json().await;
            match users {
                Ok(mov) => {
                    test_mongo_connection(&mov).await;
                    println!("{:?}", serde_json::to_string(&mov));
                }

                Err(why) => println!("ERROR: {}", why),
            }
        }
    }

    Ok(())
}

fn clean_text(movie_title: String) -> String {
    byte_serialize(movie_title.trim().as_bytes()).collect()
}

async fn test_mongo_connection(movie: &Movie) -> Result<(), mongodb::error::Error> {
    // Parse a connection string into an options struct.
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;

    // Manually set an option.
    client_options.app_name = Some("My App".to_string());

    // Get a handle to the deployment.
    let client = Client::with_options(client_options)?;

    // List the names of the databases in that deployment.
    for db_name in client.list_database_names(None, None).await? {
        println!("{}", db_name);
    }

    let db = client.database("movies");
    let collection = db.collection("singleMovies");
    let docs = bson::to_bson(&movie)
        .unwrap()
        .as_document()
        .unwrap()
        .to_owned();

    // Insert some documents into the "mydb.books" collection.
    collection.insert_one(docs, None).await?;

    Ok(())
}
