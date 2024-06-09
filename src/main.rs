// Importing the JSON module from Rocket's serde package for JSON serialization and deserialization
use rocket::{response::content::RawHtml, serde::json::Json};

// Macro to enable external crates like Rocket
#[macro_use] extern crate rocket;

// Deriving Responder for automatic conversion to HTTP response
#[derive(Responder)]
// Specifying the default status code and content type for responses of this type
#[response(status = 200, content_type = "json")]
struct SuccessResponse<T: rocket::serde::Serialize> {
    // Wrapping the data in a Json object for serialization
    data: Json<T>,
}

// Deriving Serialize for automatic conversion to JSON
#[derive(serde::Serialize)]
struct FizzBuzzResult {
    // Fields to store the input number and the computed FizzBuzz result
    number: i128,
    result: String,
}

// Handler for the root path ("/")
#[get("/")]
fn home() -> RawHtml<String> {
    // Return the index.html file as a simple example and showcase of the FizzBuzz API
    RawHtml(String::from(include_str!("../web/index.html")))
}

// Handler for the "/fizzbuzz/<number>" path, capturing the number as an i128
#[get("/fizzbuzz/<number>")]
fn fizzbuzz_single(number: i128) -> SuccessResponse<FizzBuzzResult> {
    // Computing the FizzBuzz result for the given number
    let result = fizzbuzz(number);
    // Creating a SuccessResponse with the FizzBuzzResult
    SuccessResponse {
        data: Json(FizzBuzzResult {
            number,
            result,
        }),
    }
}

// Handler for the "/html/fizzbuzz/<number>" path, capturing the number as an i128
// This handler returns an HTML response instead of JSON which is useful for libraries like htmx
#[get("/html/fizzbuzz/<number>")]
fn fizzbuzz_single_html(number: i128) -> String {
    // Computing the FizzBuzz result for the given number
    let result = fizzbuzz(number);
    // Creating an HTML response with the FizzBuzz result
    format!("<div class=\"fbapi_resp\"><p class=\"fbapi_resp_num\">{}</p><p class=\"fbapi_resp_res\"{}</p></div>", number, result)
}

// Utility function to compute the FizzBuzz result for a given number
fn fizzbuzz(number: i128) -> String {
    // Using pattern matching to determine the FizzBuzz result based on divisibility
    match (number % 3, number % 5) {
        (0, 0) => "FizzBuzz".into(), // Divisible by both 3 and 5
        (0, _) => "Fizz".into(),     // Divisible by 3 but not 5
        (_, 0) => "Buzz".into(),     // Divisible by 5 but not 3
        _ => number.to_string(),     // Not divisible by either 3 or 5
    }
}

// Entry point for launching the Rocket application
#[launch]
fn rocket() -> _ {
    // Building and configuring the Rocket instance
    rocket::build()
        // Mounting routes for the handlers
       .mount("/", routes![home, fizzbuzz_single, fizzbuzz_single_html])
}
