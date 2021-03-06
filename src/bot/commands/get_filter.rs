use super::Command;
use super::Message;
use crate::bot::telegram_client::Api;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::PgConnection;

static COMMAND: &str = "/get_filter";

pub struct GetFilter {}

impl GetFilter {
    pub fn execute(db_pool: Pool<ConnectionManager<PgConnection>>, api: Api, message: Message) {
        Self {}.execute(db_pool, api, message);
    }

    fn get_filter(
        &self,
        db_connection: &PgConnection,
        message: &Message,
        feed_url: String,
    ) -> String {
        match self.find_subscription(db_connection, message.chat.id, feed_url) {
            Err(message) => message,
            Ok(subscription) => match subscription.filter_words {
                None => "You did not set a filter for this subcription".to_string(),
                Some(filter_words) => filter_words.join(", "),
            },
        }
    }

    pub fn command() -> &'static str {
        COMMAND
    }
}

impl Command for GetFilter {
    fn response(
        &self,
        db_pool: Pool<ConnectionManager<PgConnection>>,
        message: &Message,
    ) -> String {
        match self.fetch_db_connection(db_pool) {
            Ok(connection) => {
                let text = message.text.as_ref().unwrap();
                let argument = self.parse_argument(text);
                self.get_filter(&connection, message, argument)
            }
            Err(error_message) => error_message,
        }
    }

    fn command(&self) -> &str {
        Self::command()
    }
}
