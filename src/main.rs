#![allow(non_snake_case)]

use actix_web::http::header::ContentType;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use async_stream::{try_stream, AsyncStream};
use bytes::Bytes;
use serde_derive::Deserialize;
use std::fs;
use std::fs::read_to_string;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process::exit;
use toml;

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
    notfound: String,
}
struct Pages {
    error: PathBuf,
    notimp: PathBuf,
    index: PathBuf,
}
//the config enum to easy declare an action for the config
enum Configs {
    Files,
    SSL,
    NotFound,
}
enum Response {
    OK,
    JS,
    CSS,
    NOTOK,
}
enum Image {
    PNG,
    JPG,
    ICO,
    GIF,
    OTF,
    TTF,
    MP4,
}
//The Premade Responses
fn response(x: String, y: Response) -> HttpResponse {
    match y {
        Response::OK => HttpResponse::Ok()
            .content_type(ContentType::html())
            .insert_header(("X-Hdr", "sample"))
            .body(x),
        Response::JS => HttpResponse::Ok()
            .content_type(mime::APPLICATION_JAVASCRIPT)
            .insert_header(("X-Hdr", "sample"))
            .body(x),
        Response::CSS => HttpResponse::Ok()
            .content_type(mime::TEXT_CSS)
            .insert_header(("X-Hdr", "sample"))
            .body(x),
        Response::NOTOK => HttpResponse::NotFound()
            .content_type(ContentType::html())
            .insert_header(("X-Hdr", "sample"))
            .body(x),
    }
}
fn img_response(x: PathBuf, y: Image) -> HttpResponse {
    let stream: AsyncStream<Result<Bytes, Error>, _> = try_stream! {
        let mut f = fs::File::open(x)?;
        let mut buffer = [0; 4096];
        loop{
            let n = f.read(&mut buffer[..])?;
            if n == 0 {
                break;
            }
            yield Bytes::copy_from_slice(&buffer[..n]);
        }
    };
    match y {
        Image::JPG => HttpResponse::Ok()
            .content_type("image/jpeg")
            .streaming(stream),
        Image::PNG => HttpResponse::Ok()
            .content_type("image/png")
            .streaming(stream),
        Image::ICO => HttpResponse::Ok()
            .content_type("image/x-icon")
            .streaming(stream),
        Image::GIF => HttpResponse::Ok()
            .content_type("image/gif")
            .streaming(stream),
        Image::OTF => HttpResponse::Ok()
            .content_type("font/otf")
            .streaming(stream),
        Image::TTF => HttpResponse::Ok()
            .content_type("font/ttf")
            .streaming(stream),
        Image::MP4 => HttpResponse::Ok()
            .content_type("video/mp4")
            .streaming(stream),
    }
}
//Convert the Content of a file into a single strig forthe ressponse function
fn open(x: PathBuf) -> String {
    fs::read_to_string(x).expect("Somthing went wrong")
}
fn read(types: Configs) -> std::string::String {
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
    match types {
        Configs::Files => data.config.files,
        Configs::SSL => data.config.ssl,
        Configs::NotFound => data.config.notfound,
    }
}

fn pages() -> Pages {
    let error_page = read(Configs::NotFound);
    let mut error_ = PathBuf::new();
    error_.push(&error_page);
    let mut notimplementet = PathBuf::new();
    notimplementet.push("notimplementet.html");
    //set the pathbuf for the index file
    let mut index_ = PathBuf::new();
    //check if it needs to be a index.html oder index.php file
    let readed_files = read(Configs::Files);
    match readed_files.as_str() {
        "html" => index_.push("index.html"),
        "php" => index_.push("index.php"),
        _ => index_.push(&error_page),
    }
    Pages {
        error: error_,
        notimp: notimplementet,
        index: index_,
    }
}

async fn index(req: HttpRequest) -> HttpResponse {
    //gets from the requestet url the path
    let mut path: PathBuf = req.match_info().query("file").parse().unwrap();
    if path.as_os_str().is_empty() {
        return response(open(pages().index), Response::OK);
    }

    if !path.exists() {
        return response(open(pages().error), Response::NOTOK);
    }
    //looks up if the pathbuf path is a directory or it ends with / if its is/does then it will get
    //rediretedt to the index file in the path
    if path.is_dir() || path.ends_with("/") {
        match read(Configs::Files).as_str() {
            "html" => path.push("index.html"),
            "php" => path.push("index.php"),
            _ => {
                return response(open(pages().error), Response::OK);
            }
        }
        //The response function get used for the http response the open function does convert the
        //inputed file into a single string what get parsed into the response function the Response
        //Enum is made to esaly tell the function with what for a response it should answer
        return response(open(path), Response::OK);
    }
    //if nothing aboovs fits it will check if the requeted file exist if not if will return the
    //notfound page
    match path.extension().unwrap().to_str().unwrap() {
        "js" => response(open(path), Response::JS),
        "png" => img_response(path, Image::PNG),
        "jpeg" => img_response(path, Image::JPG),
        "ico" => img_response(path, Image::ICO),
        "gif"=>  img_response(path, Image::GIF),
        "css" => response(open(path), Response::CSS),
        "otf" => img_response(path, Image::OTF),
        "ttf" => img_response(path, Image::TTF),
        "mp4" => img_response(path, Image::MP4),
        _ => response(open(path), Response::OK),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .route("/{file:.*}", web::get().to(index))
            .service(actix_files::Files::new("/", ".").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
