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
}

impl Handler for VoteHandler {
    fn handle(&self, message: &IncomingMessage, chat_room: &mut ChatRoom) -> Option<OutgoingMessage> {
        let vote_results = chat_room.type_map.get_mut::<VoteResults>().unwrap();
        vote_results.check_for_possible_reset();
        let user_id = message.from.id;
        if let Some(desired_vote) = Vote::from_str(&message.text[..]) {
            if let Some(already_existing_vote) = vote_results.get_vote(user_id) {
                return Some(Self::already_voted(&message.from, already_existing_vote));
            }
            vote_results.add_vote(user_id, desired_vote, &chat_room.users);
            
        }
        None
    }

    fn on_chat_room_create(&self, chat_room: &mut ChatRoom) {
        chat_room.type_map.insert::<VoteResults>(VoteResults::new());
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
                   message.push_str(&format!("{}: {}", user.first_name, vote.description()));
                } else {
                    warn!("unknown user: {:?}, {:?}", &self, users);
                    message.push_str(&format!("unknown user: {}", vote.description()));
                }
            }
            message.push_str(&format!("Собираемся в {}, через {}", desired_time.format("%Y-%m-%d %H:%M:%S"), ::model::sub_formatted(desired_time, ::model::now())));
            return message;
        }
        "Еще никто не голосовал.".to_owned()
    }
}

impl Key for VoteResults {
    type Value = VoteResults;
}
