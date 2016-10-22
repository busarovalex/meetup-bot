use telegram_bot::*;

// use super::vote::*;

use std::collections::HashMap;

#[derive(Debug)]
pub struct Meetup {
    id: String,
    users: HashMap<Integer, User>,
}

impl Meetup {
    pub fn new() -> Meetup {
        Meetup {
            id: "".to_owned(),
            users: HashMap::new()
        }
    }
    
    pub fn vote(&mut self, user: User) -> String {
        unimplemented!();
        // self.update_meetups();
        // //хрень. перенести в команды
        // match (self.users.get(&user.id), self.next_meetup) {
        //     (None, None) => {
        //         let response = Self::success_vote(&user, &vote, vote.desired_time());
        //         self.users.insert(user.id, (user, vote));
        //         self.next_meetup = Some(vote.desired_time());
        //         return response;
        //     },
        //     (None, Some(time)) => {
        //         let desired_time = vote.desired_time();
        //         if desired_time > time {
        //             let response = Self::success_reschedule_vote(&user, &vote, vote.desired_time());
        //             self.next_meetup = Some(vote.desired_time());
        //             self.users.insert(user.id, (user, vote));
        //             return response;
        //         } else {
        //             return Self::failed_reschedule_vote(&user, &vote, time);
        //         }
        //     },
        //     (Some(_), _) => {
        //         return Self::failed_vote(&user);
        //     }
        // }
    }
    
    // fn update_meetups(&mut self) {
    //     if let Some(planned_time) = self.next_meetup {
    //         if planned_time < now() {
    //             self.previous_meetup = Some(planned_time);
    //             self.next_meetup = None;
    //             self.users.clear();
    //         }
    //     }
    // }
    
    // fn success_vote(user: &User, vote: &Vote, time: Tm) -> String {
    //     format!("{} хочет {}. Решено установить время {:?}", user.first_name, vote.description(), time)
    // }
    
    // fn success_reschedule_vote(user: &User, vote: &Vote, time: Tm) -> String {
    //     format!("{} хочет {}. Решено перенести время на {:?}", user.first_name, vote.description(), time)
    // }
    
    // fn failed_reschedule_vote(user: &User, vote: &Vote, time: Tm) -> String {
    //     format!("{} хочет {}. Решено не переносить время. Встреча назначена на {:?}", user.first_name, vote.description(), time)
    // }
    
    // fn failed_vote(user: &User) -> String {
    //     format!("{} уже голосовал(а) за следующую встречу. Можно голосовать только один раз", user.first_name)
    // }
}
