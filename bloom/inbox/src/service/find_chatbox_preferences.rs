use super::FindChatboxPreferencesInput;
use crate::{entities::ChatboxPreferences, Error, Service};
use kernel::Actor;

impl Service {
    pub async fn find_chatbox_preferences(
        &self,
        actor: Actor,
        input: FindChatboxPreferencesInput,
    ) -> Result<ChatboxPreferences, kernel::Error> {
        if actor.is_none() {
            return Err(Error::PermissionDenied.into());
        }

        let preferences = self
            .repo
            .find_chatbox_preferences_for_namespace(&self.db, input.namespace_id)
            .await?;
        Ok(preferences)
        // TODO
        // project, err := service.projectsService.FindProjectByIDUnauthenticated(ctx, service.db, input.ProjectID)
        // if err != nil {
        //     return
        // }

        // ret = support.ChatboxPreferencesAndProjectPublicData{
        //     ChatboxPreferences: preferences,
        //     AvatarURL:          service.kernelService.ProjectAvatarURL(project.Avatar),
        //     TwitterURL:         project.TwitterURL,
        //     FacebookURL:        project.FacebookURL,
        //     PublicEmail:        project.PublicEmail,
        //     InstagramURL:       project.InstagramURL,
        //     WhatsappNumber:     project.WhatsappNumber,
        //     MastodonURL:        project.MastodonURL,
        //     HomepageURL:        project.HomepageURL,
        // }
        // return
    }
}
