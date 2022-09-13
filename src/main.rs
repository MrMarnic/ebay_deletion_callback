use rocket::http::RawStr;
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
fn user_deletion(challenge_code:String) -> rocket::response::content::RawJson<String> {
    let mut hasher = Sha256::new();

    hasher.update(challenge_code);
    hasher.update("7683ae4d-87e4-4e82-a415-9def4be3bd70");
    hasher.update("https://ebay-account-deletion-callback.onrender.com/callback");

    let response = hasher.finalize();

    return RawJson(serde_json::to_string(&EBayResponse { challengeResponse: String::from_utf8_lossy(&response[..]).parse().unwrap() }).unwrap());
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

#[derive(Serialize, Deserialize)]
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