use typemap::*;
use chrono::duration::Duration;

use model::*;
use chat_room::*;

use std::collections::HashMap;

use super::*;
use self::vote::*;

mod vote;

pub struct VoteHandler {

}

impl VoteHandler {
    pub fn new() -> VoteHandler {
        VoteHandler {

        }
    }

    fn already_voted(user: &User, vote: Vote) -> OutgoingMessage {
        let text = format!("{} уже голосовал: {}", user.first_name, vote.description());
        OutgoingMessage::with_text(text)
    }

    fn insert_or_find_vote_results<'r>(type_map: &'r mut DebugMap) -> &'r mut VoteResults {
        if type_map.contains::<VoteResults>() {
            return match type_map.get_mut::<VoteResults>() {
                Some(vote_results) => vote_results,
                None => unreachable!()
            }
        }  
        
        type_map.insert::<VoteResults>(VoteResults::new());
        type_map.get_mut::<VoteResults>().unwrap()
    }
}

impl Handler for VoteHandler {
    fn handle(&self, message: &IncomingMessage, chat_room: &mut ChatRoom) -> Option<OutgoingMessage> {
        let vote_results = Self::insert_or_find_vote_results(&mut chat_room.type_map);
        vote_results.check_for_possible_reset();
        let user_id = message.from.id;
        if let Some(desired_vote) = Vote::from_str(&message.text[..]) {
            if let Some(already_existing_vote) = vote_results.get_vote(user_id) {
                return Some(Self::already_voted(&message.from, already_existing_vote));
            }
            return vote_results.add_vote(user_id, desired_vote, &chat_room.users);
        }
        None
    }
}

#[derive(Debug)]
struct VoteResults {
    votes: HashMap<UserId, Vote>,
    desired_time: Option<DateTime>
}

impl VoteResults {
    fn new() -> VoteResults {
        VoteResults {
            votes: HashMap::new(),
            desired_time: None
        }
    }

    fn check_for_possible_reset(&mut self) {
        if let Some(desired_time) = self.desired_time {
            if desired_time < ::model::now() {
                self.reset();
            }
        }
    }

    fn reset(&mut self) {
        self.votes = HashMap::new();
        self.desired_time = None;
    }

    fn get_vote(&self, user_id: UserId) -> Option<Vote> {
        self.votes.get(&user_id).cloned()
    }

    fn add_vote(&mut self, user_id: UserId, vote: Vote, users: &HashMap<UserId, User>) -> Option<OutgoingMessage> {
        match (vote, self.desired_time) {
            (Vote::Now, None) => {
                self.desired_time = Some(::model::now().checked_add(Duration::minutes(10)).unwrap());
            },
            (Vote::NotNow, None) => {
                return Some(OutgoingMessage::with_text("Еще никто не голосовал, голосование за откладывание встречи не имеет смысла.".to_owned())); 
            },
            (Vote::NotNow, Some(desired_time)) => {
                if !self.has_anyone_voted_not_now() {
                    self.desired_time = Some(desired_time.checked_add(Duration::minutes(20)).unwrap());
                }
            },
            _ => { }
        }
        self.votes.insert(user_id, vote);
        Some(OutgoingMessage::with_text(self.generate_status_message(users)))
    }

    fn has_anyone_voted_not_now(&self) -> bool {
        self.votes.values().any(|vote| *vote == Vote::NotNow)
    }

    fn generate_status_message(&self, users: &HashMap<UserId, User>) -> String {
        if let Some(desired_time) = self.desired_time {
            let mut message = String::with_capacity(100);
            for (user_id, vote) in &self.votes {
                if let Some(ref user) = users.get(user_id) {
                   message.push_str(&format!("{}: {}\n", user.first_name, vote.description()));
                } else {
                    warn!("unknown user: {:?}, {:?}", &self, users);
                    message.push_str(&format!("unknown user: {}\n", vote.description()));
                }
            }
            match ::model::format_remaining_time(desired_time) {
                (None, remaining_time_minutes) =>  message.push_str(&format!("Собираемся через {}", remaining_time_minutes)),
                (Some(time_formatted), remaining_time_hours_and_minutes) => message.push_str(&format!("Собираемся в {}, через {}", time_formatted, remaining_time_hours_and_minutes))
            }
           
            return message;
        }
        "Еще никто не голосовал.".to_owned()
    }
}

impl Key for VoteResults {
    type Value = VoteResults;
}

#[cfg(test)]
mod tests {

    use model::*;
    use chat_room::*;
    use super::*;
    use handler::vote::VoteResults;

    use handler::traits::*;

    #[test]
    fn handler_returns_none_when_message_is_not_a_command() {
        //given
        let incoming_message = test_incoming_message("just text");
        let mut chat_room = test_chat_room();
        let handler = test_handler(&mut chat_room);
        //when
        let outgoing_message = handler.handle(&incoming_message, &mut chat_room);
        //then
        assert!(outgoing_message.is_none());
    }

    #[test]
    fn voiting_not_now_does_not_make_sense_when_no_one_has_voted_yet() {
        //given
        let incoming_message = test_incoming_message("/not_now");
        let mut chat_room = test_chat_room();
        let handler = test_handler(&mut chat_room);
        //when
        let outgoing_message = handler.handle(&incoming_message, &mut chat_room);
        //then
        assert_eq!(&outgoing_message.unwrap().text, "Еще никто не голосовал, голосование за откладывание встречи не имеет смысла.");
    }

    #[test]
    fn any_message_after_desired_time_resets_vote_results() {
        //given
        let incoming_message = test_incoming_message("/not_now");
        let mut chat_room = test_chat_room();
        let handler = test_handler(&mut chat_room);
        {
            let mut vote_results = chat_room.type_map.get_mut::<VoteResults>().unwrap();
            vote_results.desired_time = Some(::model::now());
        }
        //when
        handler.handle(&incoming_message, &mut chat_room);
        //then
        let vote_results = chat_room.type_map.get_mut::<VoteResults>().unwrap();
        assert!(vote_results.desired_time.is_none());
    }

    fn test_incoming_message(text: &str) -> IncomingMessage {
        IncomingMessage {
            text: text.to_owned(),
            time: ::model::now(),
            from: test_user()
        }
    }

    fn test_user() -> User {
        User {
        id: Id::new(1),
        first_name: "test_user_first_name".to_owned(),
        last_name: None,
        username: None
        }
    }

    fn test_chat_room() -> ChatRoom {
        ChatRoom::new(Id::new(1))
    }

    fn test_handler(chat_room: &mut ChatRoom) -> VoteHandler {
        let handler = VoteHandler::new();
        chat_room.type_map.insert::<VoteResults>(VoteResults::new());
        handler
    }
}
