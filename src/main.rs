extern crate telegram_bot;
extern crate regex;

mod meetup;
mod command;
mod manager;
mod bot;

use telegram_bot::*;

fn main() {
    let api = Api::from_env("TELEGRAM_BOT_TOKEN").unwrap();
    println!("getMe: {:?}", api.get_me());
    let res = listen(api);
    if let Err(e) = res {
        println!("An error occured: {}", e);
    }
}

fn listen(api: Api) -> Result<()> {
    let mut listener = api.listener(ListeningMethod::LongPoll(None));
    // Fetch new updates via long poll method
    let mut bot = bot::Bot::new();
    let res = listener.listen(|u| {
        if let Some(message) = u.message {
            bot.process_message(&api, message);
        }
        Ok(ListeningAction::Continue)
    });
    return res;
}
