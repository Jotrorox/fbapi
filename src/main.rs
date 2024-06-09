use rocket::{http::ContentType, serde::json::Json, Data};
use std::io::{self};
use crate::rocket::tokio::io::AsyncReadExt;

#[macro_use] extern crate rocket;

#[derive(Responder)]
#[response(status = 400, content_type = "json")]
struct ErrorResponse {
    error: String,
}

#[derive(Responder)]
#[response(status = 200, content_type = "json")]
struct SuccessResponse<T: rocket::serde::Serialize> {
    data: Json<T>,
}

#[derive(serde::Serialize)]
struct FizzBuzzResult {
    number: i32,
    result: String,
}

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[get("/fizzbuzz/<number>")]
fn fizzbuzz_single(number: i32) -> SuccessResponse<FizzBuzzResult> {
    let result = fizzbuzz(number);
    SuccessResponse {
        data: Json(FizzBuzzResult {
            number,
            result,
        }),
    }
}

fn fizzbuzz(number: i32) -> String {
    if number % 3 == 0 && number % 5 == 0 {
        "FizzBuzz".into()
    } else if number % 3 == 0 {
        "Fizz".into()
    } else if number % 5 == 0 {
        "Buzz".into()
    } else {
        number.to_string()
    }
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![hello, fizzbuzz_single])
}