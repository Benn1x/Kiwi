#![allow(non_snake_case)]

use actix_web::{ web, App, HttpRequest,HttpResponse, HttpServer, Result};
use std::path::PathBuf;
use actix_files::NamedFile;
use actix_files as fs;
use serde_derive::Deserialize;
use std::process::exit;
use toml;
use std::fs::read_to_string;
use actix_web::http::header::ContentType;

//structure for the config.toml
#[derive(Deserialize)]
struct Data {
    config: Config,
}
//structure for the config part
#[derive(Deserialize)]
struct Config {
    files: String,
    ssl: String,
    notfound: String
}

//the config enum to easy declare an action for the config
enum Configs{
    Files,
    SSL,
    NotFound
}
enum Response{
    OK,
    NOTOK
}
fn response(x: NamedFile, y:Response)->HttpResponse{
    let boxed: Box<NamedFile> = Box::new(x);
    match y{
        Response::OK => {
        HttpResponse::Ok()
        .content_type(ContentType::html())
        .insert_header(("X-Hdr", "sample"))
        .body(*boxed)
        }
        Response::NOTOK => {
            HttpResponse::NotFound()
        .content_type(ContentType::html())
        .insert_header(("X-Hdr", "sample"))
        .body(*boxed)
        }
    }
}
fn open(x: PathBuf) ->NamedFile{
    return fs::NamedFile::open(x).unwrap();
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
        Configs::NotFound =>{
            data.config.notfound
        }
    }

}

async fn index(req: HttpRequest) -> HttpResponse {
    //TODO: Use httpResponse insted of NamedFIle 
    //gets from the requestet url the path
    let mut path: PathBuf = req.match_info().query("file").parse().unwrap();
    //set the error page, and get the path from the config.toml
    let  error_page = read(Configs::NotFound);
    let mut error = PathBuf::new();
    error.push(&error_page);
    //set the noimplementet path
    let mut  notimplementet = PathBuf::new();
    notimplementet.push("notimplementet.html");
    //set the pathbuf for the index file 
    let mut index = PathBuf::new();
    //check if it needs to be a index.html oder index.php file
    let readed_files = read(Configs::Files);
    match readed_files.as_str(){
      "html" =>  index.push("index.html"),
      "php" => index.push("index.php"),
      _ => index.push(&error_page),
    }
   
    //looks up if the pathbuf path is a directory or it ends with / if its is/does then it will get
    //rediretedt to the index file in the path
    if path.is_dir() || path.ends_with("/") {
        match readed_files.as_str(){
            "html" =>  path.push("index.html"),
            "php" => path.push("index.php"),
            _ => {path.clear();
                path.push(&error_page);
            },
          }
        return response(open(path),Response::OK);
    }
    //if path is empty it will return the main file at the filr root
    if path.as_os_str().is_empty()  {
        return response(open(index), Response::OK);
    }
    //if nothing aboovs fits it will check if the requeted file exist if not if will return the
    //notfound page
    if path.exists() {
        if path.extension().unwrap().to_str().unwrap() == "php" || path.extension().unwrap().to_str().unwrap() == "js" {
           return response(open(notimplementet), Response::OK);
        }   
        return response(open(path), Response::OK);
    } else {

        return response(open(error), Response::NOTOK);
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
