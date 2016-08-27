use regex::Regex;
use telegram_bot::User;

use ::maybe_from::*;
use meetup::*;

use super::Command;

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct MeetNow {
    meetup_name: Option<String>
}

impl MeetNow {
    fn with_name(name: String) -> MeetNow {
        MeetNow {
            meetup_name: Some(name)
        }
    }
}

impl<'r> MaybeFrom<&'r str> for MeetNow {
    fn maybe_from(raw: &str) -> Option<Self> {
        let re = Regex::new(r"/now (.+)").unwrap();
        if let Some(join_caps) = re.captures(raw) {
            return Some(MeetNow::with_name(join_caps.at(1).unwrap().to_owned()));
        }
        if raw == "/now" {
            return Some(MeetNow::default());
        }
        None
    }
}

impl Command for MeetNow {
    fn execute(&self, user: User, meetup: &mut Meetup) -> String {
        meetup.vote(user, Vote::now());
        format!("Вы хотите устроить встречу СЕЙЧАС")
    }
}
