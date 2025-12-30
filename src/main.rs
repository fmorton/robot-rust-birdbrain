mod request;

use std::error::Error;
use crate::request::Request;

fn main() -> Result<(), Box<dyn Error>> {
    //let uri = Request::request_uri_from_vector(&vec!["hummingbird", "in", "orientation", "Shake", "A"]);

    //let response_using_uri = Request::response_from_uri(&uri);

    //println!("URI: {}", uri);
    //println!("Status: {}", response_using_uri.status);
    //println!("Body: {}", response_using_uri.body);

    let response = Request::response(&vec!["hummingbird", "in", "orientation", "Shake", "A"]);

    println!("Status: {}", response.status);
    println!("Body: {}", response.body);

    Ok(())
}
