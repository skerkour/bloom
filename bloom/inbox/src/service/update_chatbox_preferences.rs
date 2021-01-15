use super::UpdateChatboxPreferencesInput;
use crate::{entities::ChatboxPreferences, Service};
use kernel::Actor;

impl Service {
    pub async fn update_chatbox_preferences(
        &self,
        _actor: Actor,
        _input: UpdateChatboxPreferencesInput,
    ) -> Result<ChatboxPreferences, kernel::Error> {
        todo!();
    }
}
