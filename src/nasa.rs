use axum::{
  response::{Html,Redirect}
};
use axum_extra::extract::cookie::CookieJar;
use serde::Deserialize;
//module for HTML templating
use askama::Template;

// Apod page template struct
#[derive(Template)]
#[template(path = "apod.html")]
pub struct ApodTemplate {
  pub title: String,
  pub date: String,
  pub url: String,
  pub explanation: String,
  pub media_type: String,
}

// Matches the JSON from NASA (can reuse the same struct)
#[derive(Deserialize)]
struct ApodApiResponse {
    title: String,
    date: String,
    url: String,
    explanation: String,
    media_type: String,
}

pub async fn show_apod(jar: CookieJar) -> Result<Html<String>, Redirect> {

  // 1. Check for "username" cookie
  if jar.get("username").is_none() {
    // 2. If not found, redirect to login page
    return Err(Redirect::to("/login"));
}
 // 3. If logged in, fetch APOD data as before
  let api_url = "https://api.nasa.gov/planetary/apod?api_key=DEMO_KEY";

  let resp = reqwest::get(api_url).await;
  let resp = match resp {
    Ok(r) => r.json::<ApodApiResponse>().await,
    Err(e) => {
        return Ok(Html(format!("<h1>Error fetching APOD: {}</h1>", e)));
    }
};

  match resp {
      Ok(data) => {
          let template = ApodTemplate {
              title: data.title,
              date: data.date,
              url: data.url,
              explanation: data.explanation,
              media_type: data.media_type,
          };
          Ok(Html(template.render().unwrap()))
      }
      
        Err(_) => Ok(Html("<h1>Could not load NASA APOD</h1>".to_string())),
      
  }
}