use axum::{
  extract::{Form, State},
  response::{Html, Redirect},
};
use axum_extra::extract::cookie::{Cookie, CookieJar};

//module for HTML templating
use askama::Template;
use crate::UserStore;

// Login page template struct
#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate {
  pub error: String,         // empty string => no error
}
// Struct to receive form data
#[derive(serde::Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

// Signup page template struct
#[derive(Template)]
#[template(path = "signup.html")]
pub struct SignupTemplate {
  pub error: String, // empty means no error
}

#[derive(serde::Deserialize)]
pub struct SignupForm {
    pub username: String,
    pub password: String,
}


// ---------- GET /login ----------
pub async fn show_login() -> Html<String> {
  let page = LoginTemplate { error: String::new() };
  Html(page.render().unwrap())
}

// ---------- POST /login ----------
pub async fn process_login(
  // Extract shared state (the user store: Arc<Mutex<HashMap<String, String>>>)
  State(users): State<UserStore>,
  // Extract the cookies for this request (used to set a session cookie)
  jar: CookieJar,
  // Extract the submitted form data as a LoginForm struct
  Form(form): Form<LoginForm>,
) -> Result<(CookieJar, Redirect), Html<String>> {
  // Lock the users HashMap for safe access (since multiple requests can happen at once)
  let users = users.lock().unwrap();

  // Look for the given username in the HashMap
  match users.get(&form.username) {
      // If user is found AND the stored password matches the submitted one
      Some(stored) if stored == &form.password => {
          // Create a new cookie named "username" with the user's name as value
          let cookie = Cookie::new("username", form.username.clone());
          // Add the cookie to the response and redirect to the /apod page
          Ok((jar.add(cookie), Redirect::to("/apod")))
      }
      // Otherwise (user not found, or password doesn't match)
      _ => {
          // Prepare the login template with an error message
          let page = LoginTemplate {
              error: "Invalid username or password".to_string(),
          };
          // Return the rendered login page with the error
          Err(Html(page.render().unwrap()))
      }
  }
}

// ---------- GET /signup ----------
pub async fn show_signup() -> Html<String> {
  let page = SignupTemplate { error: String::new() };
  Html(page.render().unwrap())
}

// ---------- POST /signup ----------
pub async fn process_signup(
  State(users): State<UserStore>,
  Form(form): Form<SignupForm>,
) -> Result<Redirect, Html<String>> {
  let mut users = users.lock().unwrap();

  if users.contains_key(&form.username) {
      // Username already exists
      let page = SignupTemplate {
          error: "Username already exists".to_string(),
      };
      Err(Html(page.render().unwrap()))
  } else {
      // Add the user
      users.insert(form.username, form.password);
      // Redirect to login so the user can sign in
      Ok(Redirect::to("/login"))
  }
}

// ---------- GET /logout ----------
pub async fn logout(jar: CookieJar) -> (CookieJar, Redirect) {
  // Remove the session cookie (set it to empty, with max_age 0)
  let jar = jar.remove(Cookie::from("username"));
  // Redirect to login page
  (jar, Redirect::to("/login"))
}