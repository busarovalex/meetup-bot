use super::meetup::*;

pub struct MeetupList {
    meetups: Vec<Meetup>
}

impl MeetupList {
    pub fn new() -> MeetupList {
        MeetupList {
            meetups: Vec::new()
        }
    }
    
    pub fn get(&mut self, name: &str) -> Option<&mut Meetup> {
        self.meetups.iter_mut().find(|m| m.name.as_ref().map(|n| &n[..]) == Some(name))
    }
    
    pub fn get_default(&mut self) -> Option<&mut Meetup> {
        self.meetups.get_mut(0)
    }
}
