use time::Tm;

#[derive(Clone, Copy, Debug)]
pub enum Vote {
    Now(Tm),
    At(Tm)
}

impl Vote {
    pub fn now() -> Vote {
        Vote::Now(::time::now())
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
