#[macro_use]
extern crate rocket;

use rocket::http::ContentType;
use rocket::tokio::fs::File;
use rocket::Request;
use rocket::response::stream::ReaderStream;

use uuid::Uuid;

use poc_cyberpunk_pyo3::cyberpunk_endpoint::CyberpunkEndpoint;
use poc_cyberpunk_pyo3::cyberpunk_endpoint::Transformations;


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/unsafe/<key>?<transformations..>")]
async fn unsafe_route(
    key: &str,
    transformations: Transformations<'_>,
) -> (ContentType, ReaderStream![File]) {
    println!("key {}", key);
    println!("transform {:?}", transformations.format);

    let endpoint = CyberpunkEndpoint {
        audio: key,
        hash: "unsafe",
        transformations,
    };


    let request_id = Uuid::new_v4();

    let (processed_key, ext) = endpoint.process(request_id);

    let file = File::open(format!("testdata/{}", processed_key))
        .await
        .unwrap();

    let stream = ReaderStream::one(file);
    (ContentType::new("audio", ext), stream)
}

#[catch(404)]
fn not_found(_req: &Request) -> &'static str {
    "404 Not Found"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .register("/", catchers![not_found])
        .mount("/", routes![index, unsafe_route])
}
