# Actix Web Boilerplate
An Actix Web boilerplate with R2D2 and Diesel ORM/Migrations support

## Requirements
- Rust Stable/Nightly
- Cargo

## Database Support
Currently the repository is set up to use PostgreSQL as its DBMS. 
  However, you can change this to other diesel-compatible DBMSes by 
  changing PgConnection in [src/db/mod.rs](./src/db/mod.rs) to your 
  preferred choice and importing necessary dependencies in Cargo.toml

## Folder Structure
The following are the notable folders with their descriptions:
* [/migrations](./migrations/) - Contains your diesel migrations
* [/src/app](./src/app/) - Contains most files related to your app
  * [/src/app/api](./src/app/api/) - Contains methods for handling requests
  * [/src/app/db](./src/app/db/) - Contains methods for connecting to the database and setting up connection pools
  * [/src/app/models](./src/app/models/) - Contains your diesel model files

## Configuration
- You can set configuration variables in [config.toml](./config.toml)
- You can also configuration variables via environment variables 
    which will override [config.toml](./config.toml)
- You can also declare environment variables in a .env file which will get loaded by dotenv
 
  Example of .env contents:
  ```
  APP_LISTEN_ADDRESS=localhost:9999
  APP_DATABASE_URL=postgres://user:pass@localhost:5432/dbname
  ```

## Building
Build using cargo:
```
cargo build
```

## Running
Run using cargo:
```
cargo run
```
