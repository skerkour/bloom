use super::FindChatboxPreferencesInput;
use crate::{entities::ChatboxPreferences, Service};
use kernel::Actor;

impl Service {
    pub async fn find_chatbox_preferences(
        &self,
        _actor: Actor,
        _input: FindChatboxPreferencesInput,
    ) -> Result<ChatboxPreferences, kernel::Error> {
        todo!();
    }
}
