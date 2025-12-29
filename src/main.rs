use std::error::Error;

struct Response {
    body: String,
    status: u16,
}

impl Response {
    fn response(uri: &str) -> Response {
        println!("{:?} {}", {}, uri);
        let response = reqwest::blocking::get("http://127.0.0.1:30061/hummingbird/in/orientation/Shake/A");

        match response {
            Ok(response) => {
                if response.status().is_success() {
                    let status = response.status().as_u16();

                    match response.text() {
                        Ok(text) => return Response { body: text, status: status },
                        Err(e) => eprintln!("Error reading body: {}", e),
                    }
                } else {
                    eprintln!("API returned non-success status: {}", response.status());
                }
            },
            Err(err) => {
                eprintln!("Request failed: {}", err);
            }
        }

        Response { body: "".to_string(), status: 500 }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let response = Response::response("http://127.0.0.1:30061/hummingbird/in/orientation/Shake/A");

    println!("Status: {}", response.status);
    println!("Body: {}", response.body);

    Ok(())
}
