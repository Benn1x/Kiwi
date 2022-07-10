#![allow(non_snake_case)]

use actix_web::{ web, App, HttpRequest, HttpServer, Result};
use std::path::PathBuf;
use actix_files::NamedFile;
use actix_files as fs;
use serde_derive::Deserialize;
use std::process::exit;
use toml;
use std::fs::read_to_string;


#[derive(Deserialize)]
struct Data {
    config: Config,
}

#[derive(Deserialize)]
struct Config {
    files: String,
    ssl: String,
    notfound: String
}


enum Configs{
    Files,
    SSL,
    notFound
}


fn read(types: Configs) -> std::string::String{
    let filename = "config.toml";
    let contents = match read_to_string(filename) {
        // If successful return the files text as `contents`.
        // `c` is a local variable.
        Ok(c) => c,
        // Handle the `error` case.
        Err(_) => {
            // Write `msg` to `stderr`.
            eprintln!("Could not read file `{}`", filename);
            // Exit the program with exit code `1`.
            exit(1);
        }
    };
    let data: Data = match toml::from_str(&contents) {
        // If successful, return data as `Data` struct.
        // `d` is a local variable.
        Ok(d) => d,
        // Handle the `error` case.
        Err(_) => {
            // Write `msg` to `stderr`.
            eprintln!("Unable to load data from `{}`", filename);
            // Exit the program with exit code `1`.
            exit(1);
        }
    };
    match types{
        Configs::Files => {
            data.config.files
        }
        Configs::SSL => {
            data.config.ssl
        }
        Configs::notFound =>{
            data.config.notfound
        }
    }

}

async fn index(req: HttpRequest) -> Result<NamedFile> {
    let mut path: PathBuf = req.match_info().query("file").parse().unwrap();
    let  error_page = read(Configs::notFound);
    let mut error = PathBuf::new();
    error.push(&error_page);
    let mut  notimplementet = PathBuf::new();
    notimplementet.push("notimplementet.html");
    let mut index = PathBuf::new();
    let readed_files = read(Configs::Files);
    match readed_files.as_str(){
      "html" =>  index.push("index.html"),
      "php" => index.push("index.php"),
      _ => index.push(&error_page),
    }
    if path.is_dir() || path.ends_with("/") {
        match readed_files.as_str(){
            "html" =>  path.push("index.html"),
            "php" => path.push("index.php"),
            _ => {path.clear();
                path.push(&error_page);
            },
          }
        return Ok(NamedFile::open(path)?)
    }
    if path.as_os_str().is_empty()  {
      return Ok(NamedFile::open(index)?);
    }

    if path.exists() {
        if path.extension().unwrap().to_str().unwrap() == "php" || path.extension().unwrap().to_str().unwrap() == "js" {
            return Ok(NamedFile::open(notimplementet).unwrap());
        }   
        Ok(NamedFile::open(path).unwrap())
    } else {

        Ok(NamedFile::open(error).unwrap())
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .route("/{file:.*}", web::get().to(index))
            .service(fs::Files::new("/", ".").index_file("index.html"))
        })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
