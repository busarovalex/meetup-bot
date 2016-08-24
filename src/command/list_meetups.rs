use telegram_bot::User;

use ::maybe_from::*;
use chatroom::*;

use super::Command;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct ListMeetups;

impl<'r> MaybeFrom<&'r str> for ListMeetups {
    fn maybe_from(raw: &str) -> Option<Self> {
        if raw == "/list" {
            return Some(ListMeetups);
        }
        None
    }
}

impl Command for ListMeetups {
    fn execute(&self, _: User, _: &mut ChatRoom) -> String {
        format!("Список встреч:")
    }
}
