// ch02/src/main.rs
use std::io::{Error, ErrorKind};
use std::str::FromStr;
use serde::Serialize;
use warp::filters::method::method;
use warp::{Filter, reject::Reject, Rejection, Reply, http::StatusCode, http::Method};

#[derive(Debug)] 
struct InvalidId;
impl Reject for InvalidId {}


#[derive(Debug, Serialize)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

async fn get_questions() -> Result<impl warp::Reply, warp::Rejection> {
    let question = Question::new(
        QuestionId::from_str("1").expect("No id provided"), 
        "First Question".to_string(),
    "Content of question".to_string(), 
    Some(vec!["Faq".to_string()]));
    
    match question.id.0.parse::<i32>() {
        Err(_) => {
            Err(warp::reject::custom(InvalidId))
        },
        Ok(_) => {
            Ok(warp::reply::json(&question))
        }
    }
}

async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    println!("{:?}", r);
    if let Some(_InvalidId) = r.find::<InvalidId>() {
        Ok(warp::reply::with_status("No valid Id presented", StatusCode::UNPROCESSABLE_ENTITY,))
        
    }
    else {
        Ok(warp::reply::with_status("Route not Found", StatusCode::NOT_FOUND))
    }
}
#[derive(Debug, Serialize)]
struct QuestionId(String);

impl Question {
    fn new(
        id: QuestionId,
        title: String,
        content: String,
        tags: Option<Vec<String>>
    ) -> Self {
        Question {
            id,
            title,
            content,
            tags,
        }
    }
}

impl std::fmt::Display for Question {
    fn fmt(&self, f: &mut std::fmt::Formatter)
        -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}, title: {}, content: {}, tags: {:?}",
            self.id, self.title, self.content, self.tags
        )
    }
}

impl std::fmt::Display for QuestionId {
    fn fmt(&self, f: &mut std::fmt::Formatter) 
        -> Result<(), std::fmt::Error> {
        write!(f, "id: {}", self.0)
    }
}

impl FromStr for QuestionId {
    type Err = std::io::Error;

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() {
            false => Ok(QuestionId(id.to_string())),
            true => Err(
                Error::new(ErrorKind::InvalidInput, "No id provided")
            ),
        }
    }
}

#[tokio::main]
async fn main() {
    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("not-in-the-request")
        .allow_methods(&[Method::PUT, Method::DELETE, Method::POST]);

    let get_items  = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and_then(get_questions)
        .recover(return_error);

    let routes = get_items.with(cors).recover(return_error);
    
    // let hello = warp::get().map(|| format!("Hello, World!"));
    warp::serve(routes)
    .run(([127, 0, 0, 1], 3030))
    .await;
}