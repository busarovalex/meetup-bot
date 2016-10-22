#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Vote {
    Now,
    NotNow
}

impl Vote {
    pub fn now() -> Vote {
        Vote::Now
    }
    
    pub fn not_now() -> Vote {
        Vote::NotNow
    }

    pub fn description(&self) -> String {
        match self {
            &Vote::Now => format!("устроить встречу СЕЙЧАС"),
            &Vote::NotNow => format!("устроить встречу позже")
        }
    }

    pub fn from_str(text: &str) -> Option<Vote> {
        match text {
            "/now" => Some(Self::now()),
            "/not_now" => Some(Self::not_now()),
            _ => None
        }
    }
}
