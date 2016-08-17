use regex::Regex;

use std::convert::From;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Command {
    JoinMeetup(String),
    LeaveMeetup(Option<String>),
    ListMeetups,
    None
}

impl<'r> From<&'r str> for Command {
    fn from(raw: &str) -> Self {
        let re = Regex::new(r"/join (.+)").unwrap();
        if let Some(join_caps) = re.captures(raw) {
            return Command::JoinMeetup(join_caps.at(1).unwrap().to_owned());
        }
        let re = Regex::new(r"/leave (.*)").unwrap();
        if let Some(leave_caps) = re.captures(raw) {
            return Command::LeaveMeetup(leave_caps.at(1).map(From::from));
        }
        if raw == "/leave" {
            return Command::LeaveMeetup(None);
        }
        if raw == "/list" {
            return Command::ListMeetups;
        }
        Command::None
    }
}

#[cfg(test)]
mod tests {
    
    use super::*;
    
    #[test]
    fn test_join_command() {
        let text = "/join dinner";
        assert_eq!(Command::from(text), Command::JoinMeetup("dinner".into()));
    }
    
    #[test]
    fn test_join_command_when_empty_secret() {
        let text = "/join";
        assert_eq!(Command::from(text), Command::None);
    }
    
    #[test]
    fn test_leave_command() {
        let text = "/leave supper";
        assert_eq!(Command::from(text), Command::LeaveMeetup(Some("supper".into())));
    }
    
    #[test]
    fn test_leave_command_when_empty_secret() {
        let text = "/leave";
        assert_eq!(Command::from(text), Command::LeaveMeetup(None));
    }
}
