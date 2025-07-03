
use axum::{
  routing::get,
  routing::get_service,
  serve,
  Router,
  
};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
// Import the 'SocketAddr' type for defining the server's IP address and port
use std::{net::SocketAddr, collections::HashMap, sync::{Arc, Mutex}};
mod auth;
mod nasa;

pub type UserStore = Arc<Mutex<HashMap<String, String>>>;
// Marks the main function as asynchronous and sets up the Tokio runtime (needed for async code in Rust)
#[tokio::main]
async fn main() {

  // Initialize a shared user store using Arc and Mutex for thread-safe access
  let users: UserStore = Arc::new(Mutex::new(HashMap::new()));

  // route configuration for the application using Axum's Router
  let app = Router::new()
  .route("/login", get(auth::show_login).post(auth::process_login))
  .route("/signup", get(auth::show_signup).post(auth::process_signup))
  .route("/apod", get(nasa::show_apod))
  .route("/logout", get(auth::logout))
  .nest_service("/static", get_service(ServeDir::new("static"))
  .handle_error(|error| async move {
    (axum::http::StatusCode::INTERNAL_SERVER_ERROR, format!("Static file error: {}", error))
    
})).with_state(users.clone());

  

  // Define the socket address the server will listen on (127.0.0.1:3000 = localhost:3000)
  let port = std::env::var("PORT")
  .ok()
  .and_then(|s| s.parse().ok())
  .unwrap_or(3000u16);
let addr = SocketAddr::from(([0, 0, 0, 0], port));

// ── listener + serve ─────────────────────────────────────────
println!("Listening on {addr}");
let listener = TcpListener::bind(addr).await.unwrap();
serve(listener, app).await.unwrap();
}



