use regex::Regex;
use time::Tm;

use ::maybe_from::*;
use chatroom::*;

use super::Command;

use telegram_bot::User;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MeetAt {
    meetup_name: Option<String>,
    at: Tm
}

impl MeetAt {
    fn new(at: Tm) -> MeetAt {
        MeetAt {
            meetup_name: None,
            at: at
        }
    }
    
    fn with_name(name: String, at: Tm) -> MeetAt {
        MeetAt {
            meetup_name: Some(name),
            at: at
        }
    }
}

impl<'r> MaybeFrom<&'r str> for MeetAt {
    fn maybe_from(raw: &str) -> Option<Self> {
        let re = Regex::new(r"/at (.+)").unwrap();
        if let Some(join_caps) = re.captures(raw) {
            return Some(MeetAt::with_name(join_caps.at(1).unwrap().to_owned(), ::time::now()));
        }
        if raw == "/at" {
            return Some(MeetAt::new(::time::now()));
        }
        None
    }
}

impl Command for MeetAt {
    fn execute(&self, _: User, _: &mut ChatRoom) -> String {
        format!("Вы хотите устроить встречу В")
    }
}
