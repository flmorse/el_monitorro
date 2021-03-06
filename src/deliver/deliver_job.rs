use super::DeliverChatUpdatesJob;
use crate::db::telegram;
use fang::typetag;
use fang::Error as FangError;
use fang::PgConnection;
use fang::Queue;
use fang::Runnable;
use serde::{Deserialize, Serialize};

const CHATS_PER_PAGE: i64 = 100;

#[derive(Serialize, Deserialize)]
pub struct DeliverJob {}

impl Default for DeliverJob {
    fn default() -> Self {
        Self::new()
    }
}

impl DeliverJob {
    pub fn new() -> Self {
        DeliverJob {}
    }
}

#[typetag::serde]
impl Runnable for DeliverJob {
    fn run(&self, connection: &PgConnection) -> Result<(), FangError> {
        let mut current_chats: Vec<i64>;
        let mut page = 1;
        let mut total_chat_number = 0;

        log::info!("Started delivering feed items");

        loop {
            current_chats =
                match telegram::fetch_chats_with_subscriptions(connection, page, CHATS_PER_PAGE) {
                    Ok(chats) => chats,
                    Err(error) => {
                        let description = format!("{:?}", error);

                        return Err(FangError { description });
                    }
                };

            page += 1;

            if current_chats.is_empty() {
                break;
            }

            total_chat_number += current_chats.len();

            for chat_id in current_chats {
                Queue::push_task_query(connection, &DeliverChatUpdatesJob { chat_id }).unwrap();
            }
        }

        log::info!("Started checking delivery for {} chats", total_chat_number,);

        Ok(())
    }

    fn task_type(&self) -> String {
        super::JOB_TYPE.to_string()
    }
}
