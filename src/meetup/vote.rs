use time::{Tm, Duration};

#[derive(Clone, Copy, Debug)]
pub enum Vote {
    Now(Tm),
    At(Tm)
}

impl Vote {
    pub fn now() -> Vote {
        Vote::Now(::time::now())
    }
    
    pub fn at(at: Tm) -> Vote {
        Vote::At(at)
    }
    
    pub fn desired_time(&self) -> Tm {
        match self {
            &Vote::Now(time) => time + Duration::minutes(10),
            &Vote::At(time) => time
        }
    }
}

impl Vote {
    pub fn description(&self) -> String {
        match self {
            &Vote::Now(_) => format!("устроить встречу СЕЙЧАС"),
            &Vote::At(time) => format!("устроить встречу в {:?}", time)
        }
    }
}
