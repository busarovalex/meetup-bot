use telegram_bot::*;

#[derive(Debug, Clone)]
pub struct MeetupError {
    pub destination: ErrorDestination,
    pub error_type: ErrorType
}

#[derive(Debug, Clone)]
pub enum ErrorType {
    ChatIsNotGroup,
    NotATextMessage
}

#[derive(Debug, Clone)]
pub struct ErrorDestination {
    pub chat_id: Integer,
    pub user: User
}

impl MeetupError {
    pub fn new(destination: ErrorDestination,
               error_type: ErrorType) -> MeetupError {
        MeetupError {
            destination: destination,
            error_type: error_type
        }
    }
    
    pub fn chat_is_not_group(destination: ErrorDestination) -> MeetupError {
        MeetupError::new(destination, ErrorType::ChatIsNotGroup)
    }
    
    pub fn not_a_text_message(destination: ErrorDestination) -> MeetupError {
        MeetupError::new(destination, ErrorType::NotATextMessage)
    }
}

impl ErrorDestination {
    pub fn new(user: User, chat: Integer) -> ErrorDestination {
            ErrorDestination {
            chat_id: chat,
            user: user
        }
    }
}

impl ErrorType {
    pub fn description(&self) -> String {
        match self {
            &ErrorType::ChatIsNotGroup => {
                format!("Бот работает только в групповых чатах, добавьте бота в собеседники группового чата.")
            },
            &ErrorType::NotATextMessage => {
                format!("Бот обрабатывает только текстовые сообщения.")
            }
        }
    }
    
    pub fn is_response_needed(&self) -> bool {
        match self {
            &ErrorType::NotATextMessage => false,
            _ => true
        }
    }
}
