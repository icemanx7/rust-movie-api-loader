# Movie API Loader
A simple rust script that runs through a collection of Movie titles and queries the OMDB movie API for more detailed information about the movie. I made it as a helper script for my movie review application.

# Introduction

I created a simple Rust script that queries the OMDB api by movie title and then accepts and consumes the data and attributes for that movie. The list of movies where stored in a Sqlite Database so in this script I read from that database, queried the API and then stored the data in a Mongo DB instance.

## Where is the API Key?

The API key is store in a private .env file or in your OS environment variables. This allows for safe and secure retrieval of the API without hard coding it in the source code.

# Why Rust?
My first choice was going to be Python but I soon realized that I would prefer to have a type safe language. This will ensure that the JSON I'm getting back is type and I don't insert some random JSON response. Therefore it will result in more consistent data. 

# Why Mongo?

As you can see from the Movie Model Type the object is quite big and I needed to use a DB structure where queries such a big object wouldn't be a problem.

# Libraries and frameworks used

* Serde: JSON conversion
* Tokio: For async operations
* dotenv: Reading environment files where the Api Keys will be store

# REMEMBER TO ADD YOUR .ENV file with the API KEY
