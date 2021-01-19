use reqwest::Error;
use rusqlite::NO_PARAMS;
use rusqlite::{params, Connection, Result};
use serde::Deserialize;
use std::env;
use url::form_urlencoded::{byte_serialize, parse};

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
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
    let conn = Connection::open("movies.db").unwrap();
    let request_url = format!("");

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

        call_movie_api(movie_title).await.unwrap();
        // println!("Found Movie title {:?}", movie_title);
    }

    let users: Movie = response.json().await?;
    println!("{:?}", users);
    Ok(())
}

fn initDatabase(db_name: String) -> Result<()> {
    let conn = Connection::open("movieDetails.db")?;
    conn.execute(
        "create table if not exists MovieDetails (
             id integer primary key,
             name text not null unique
         )",
        NO_PARAMS,
    )?;

    Ok(())
}

async fn call_movie_api(title: String) -> Result<(), Error> {
    let clean_movie = clean_text(title);
    let request_url = format!("", clean_movie);
    println!("{}", request_url);
    let response = reqwest::get(&request_url).await;
    match response {
        Err(why) => print!("hello"),
        Ok(res) => {
            println!("bye");
            let users: Result<Movie, Error> = res.json().await;
            match users {
                Ok(mov) => println!("{:?}", mov),
                Err(why) => println!("ERROR: {}", why),
            }
        }
    }

    // let users: Movie = response.json().await.unwrap();
    // println!("{:?}", users);
    Ok(())
}

fn clean_text(movie_title: String) -> String {
    byte_serialize(movie_title.trim().as_bytes()).collect()
}
