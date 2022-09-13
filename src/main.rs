use rocket::http::{RawStr, Status};
use rocket::http::uri::Uri;
use rocket::Request;
use rocket::response::content::RawJson;
use rocket::response::status;
use rocket::serde::json::Json;
use serde::{Serialize, Deserialize};
use sha2::{Sha256,Digest};
use sha2::digest::Mac;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "EBay User Deletion Callback. Hello"
}

#[get("/callback?<challenge_code>")]
fn user_deletion(challenge_code:String) -> EBayResponse {
    let mut hasher = Sha256::new();

    hasher.update(challenge_code.as_bytes());
    hasher.update("7683ae4d-87e4-4e82-a415-9def4be3bd70".as_bytes());
    hasher.update("https://ebay-account-deletion-callback.onrender.com/callback".as_bytes());

    let response = hasher.finalize();
    let bytes = &response[..];
    let hex = hex::encode(bytes);

    return EBayResponse { challengeResponse: hex };
}

#[post("/callback", format = "application/json", data = "<request>")]
fn user_deletion_request(request:Json<EbayUserDeletionRequest>) -> status::Accepted<String> {
    println!("{}", serde_json::to_string(&request.0).unwrap());
    return status::Accepted(Some("Notification acknowledged!".to_string()))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, user_deletion, user_deletion_request])
}

#[derive(Responder)]
#[response(status = 200, content_type = "json")]
struct EBayResponse {
    pub challengeResponse: String
}
#[derive(Serialize, Deserialize)]
struct EbayUserDeletionRequest {
    pub metadata: Metadata,
    pub notification: Notification
}
#[derive(Serialize, Deserialize)]
struct Metadata {
    pub topic: String,
    pub schemaVersion: String,
    pub deprecated: bool
}
#[derive(Serialize, Deserialize)]
struct Notification {
    pub notificationId : String,
    pub eventDate : String,
    pub publishDate: String,
    pub publishAttemptCount: i32,
    pub data: UserData
}
#[derive(Serialize, Deserialize)]
struct UserData {
    pub username: String,
    pub userId: String,
    pub eiasToken: String
}