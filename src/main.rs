// Importing the JSON module from Rocket's serde package for JSON serialization and deserialization
use rocket::serde::json::Json;

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
    number: i32,
    result: String,
}

// Handler for the root path ("/")
#[get("/")]
fn hello() -> &'static str {
    // Returning a static string with information about the API
    "This is a FizzBuzz API. Try /fizzbuzz/<number>. For more information, visit https://github.com/Jotrorox/fbapi."
}

// Handler for the "/fizzbuzz/<number>" path, capturing the number as an i32
#[get("/fizzbuzz/<number>")]
fn fizzbuzz_single(number: i32) -> SuccessResponse<FizzBuzzResult> {
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

// Utility function to compute the FizzBuzz result for a given number
fn fizzbuzz(number: i32) -> String {
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
       .mount("/", routes![hello, fizzbuzz_single])
}
