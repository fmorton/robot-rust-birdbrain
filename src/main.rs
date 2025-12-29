use std::error::Error;

struct Response2 {
    body: String,
    status: u16,
}

impl Response2 {
    fn response(uri: &str) -> Response2 {
        println!("{:?} {}", {}, uri);
        let response2 = reqwest::blocking::get("http://127.0.0.1:30061/hummingbird/in/orientation/Shake/A");

        match response2 {
            Ok(response2) => {
                if response2.status().is_success() {
                    let status = response2.status().as_u16();

                    match response2.text() {
                        Ok(text) => return Response2 { body: text, status: status },
                        Err(e) => eprintln!("Error reading body: {}", e),
                    }
                } else {
                    eprintln!("API returned non-success status: {}", response2.status());
                }
            },
            Err(err) => {
                eprintln!("Request failed: {}", err);
            }
        }

        Response2 { body: "".to_string(), status: 500 }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let _another = Response2::response("http://127.0.0.1:30061/hummingbird/in/orientation/Shake/A");

    println!("Status: {}", _another.status);
    println!("Body: {}", _another.body);

    Ok(())
}
