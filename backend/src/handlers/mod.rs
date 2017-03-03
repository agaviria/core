pub mod ping;
// pub mod events;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::Error;

use std::fmt;
use std::io::Read;
use std::sync::Mutex;
use std::fmt::Display;
use std::str::FromStr;

use serde_json;
use iron::modifier::Modifier;
use iron::{Response, Request, IronResult};
use iron::status;

lazy_static! {
    static ref STATE: Mutex<State> = Mutex::new(State::new());
}

#[derive(Clone, Debug)]
struct Date {
    year: i32,
    month: i32,
    day: i32,
}

#[derive(Clone, Debug)]
struct DateParseError;

impl FromStr for Date {
    type Err = DateParseError;

    fn from_str(s: &str) -> Result<Date, DateParseError> {
        let parts = s.split('-').collect::<Vec<_>>();
        if parts.len() == 3 {
            Ok(Date {
                year: parts[0].parse::<i32>().map_err(|_| DateParseError)?,
                month: parts[1].parse::<i32>().map_err(|_| DateParseError)?,
                day: parts[2].parse::<i32>().map_err(|_| DateParseError)?,
            })
        } else {
            Err(DateParseError)
        }
    }
}

impl Deserialize for Date {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer
    {
        let date_str: String = try!(Deserialize::deserialize(deserializer));
        date_str.parse::<Date>()
            .map_err(|_| D::Error::custom("invalid date format"))
    }
}

impl Serialize for Date {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        format!("{}", self).serialize(serializer)
    }
}

#[allow(unused_must_use)]
impl Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day);
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Event {
    id: Option<i32>,
    name: String,
    description: Option<String>,
    date: Option<Date>,
}

impl Event {
    fn new(id: i32, name: &str, description: &str, date_str: &str) -> Event {
        Event {
            id: Some(id),
            name: name.to_owned(),
            description: Some(description.to_owned()),
            date: Some(date_str.parse::<Date>().unwrap()),
        }
    }
}

macro_rules! Events {
    [$(($id: expr, $name: expr, $desc: expr, $date_str: expr)),+] => (
        vec![$(Event::new($id, $name, $desc, $date_str)),+]
    )
}


struct State {
    events: Vec<Event>,
}

impl State {
    fn new() -> Self {
        let events =
            Events![(1, "RxJs", "Learn RxJS this year ", "2017-03-02"),
                    (2, "VueJS", "VueJS with RxJS", "2017-03-02"),
                    (3, "Futures", "Learn streams and subcribe futures with tokio.", "2017-03-01")];
        State { events: events }
    }
}

pub fn handle_get_many_events(_: &mut Request) -> IronResult<Response> {
    let json = {
        let state = STATE.lock().unwrap();
        serde_json::to_string_pretty(&state.events).unwrap()
    };
    Ok(Response::with((mime!(Application / Json), status::Ok, json)))
}

pub fn handle_post_new_event(request: &mut Request) -> IronResult<Response> {
    let mut content = String::new();
    match request.body.read_to_string(&mut content) {
        Ok(_) => {
            match serde_json::from_str::<Event>(&mut content) {
                Ok(mut event) => {
                    let mut state = STATE.lock().unwrap();
                    event.id = Some(find_next_id(&state.events));
                    let json = serde_json::to_string_pretty(&event).unwrap();
                    state.events.push(event);
                    Ok(Response::with((mime!(Application / Json), status::Ok, json)))
                }
                Err(_) => {
                    println!("ERROR: Failed to parse request\n{}", content);
                    Ok(Response::with((status::BadRequest,
                                       ErrorResponse::new(100, "Invalide request body"))))
                }
            }
        }
        Err(_) => {
            println!("FATAL: Failed to read request body");
            Ok(Response::with((status::InternalServerError)))
        }
    }
}

fn find_next_id(events: &Vec<Event>) -> i32 {
    events.iter()
        .fold(1, |id, event| {
            if let Some(id_) = event.id {
                if id <= id_ {
                    id_ + 1
                } else {
                    id
                }
            } else {
                id
            }
        })
}

#[derive(Clone, Debug, Serialize)]
struct ErrorResponse {
    #[serde(rename="errorCode")]
    code: i32,
    message: String,
}

impl ErrorResponse {
    fn new<S>(code: i32, message: S) -> Self
        where S: Into<String>
    {
        ErrorResponse {
            code: code,
            message: message.into(),
        }
    }
}

impl Modifier<Response> for ErrorResponse {
    fn modify(self, response: &mut Response) {
        mime!(Application / Json).modify(response);
        let json = serde_json::to_string_pretty(&self).unwrap();
        json.modify(response);
    }
}
