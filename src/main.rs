use rocket::{
    routes,
    response,
    response::Responder,
    Request,
    Response,
    fs::FileServer,
    serde::json::{Json, serde_json::json, Value},
    get, http::Status
};

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {

    let _rocket = rocket::build()
        .mount("/", FileServer::from("www/"))
        .mount("/", routes![example])
        .launch()
        .await?;
    Ok(())
}

#[get("/test")]
fn example() -> Json<Value> {
    let x = json!({
            "id": 20,
            "price": 11.06
        });
    
    Json(x)
}

/// Allows request from all origins with the 
/// status passed to the struct. This is needed
/// for open apis, otherwise your client might
/// get annoyed at the CORS errors.
///
/// This will work as a default return type for 
/// http methods.
///
/// You can use this directly or just copy / modify
/// it for your needs
struct OpenApiResponse {
    status: Status
}
impl<'r> Responder<'r, 'static> for OpenApiResponse {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        Response::build()
            .status(self.status)
            .raw_header("Access-Control-Allow-Origin", "*")
            .ok()
    }
}
