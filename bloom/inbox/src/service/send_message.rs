use super::SendMessageInput;
use crate::{entities::Message, Error, Service};
use kernel::Actor;

impl Service {
    pub async fn send_message(&self, actor: Actor, input: SendMessageInput) -> Result<Message, kernel::Error> {
        let actor = self.kernel_service.current_user(actor)?;

        let conversation = self
            .repo
            .find_conversation_by_id(&self.db, input.conversation_id)
            .await?;

        self.kernel_service
            .check_namespace_membership(&self.db, actor.id, conversation.namespace_id)
            .await
            .map_err(|_| Error::ConversationNotFound)?;

        // // contact, err := service.growthRepo.FindContactByConversationID(ctx, service.db, input.ConversationID)
        // // if err != nil {
        // // 	return
        // // }

        // // we can ignore error as it's not important
        // conversation.LastMessageReceivedAt = now
        // _ = service.supportRepo.UpdateConversation(ctx, service.db, conversation)

        // err = service.ValidateMessage(input.Body)
        // if err != nil {
        //     return
        // }

        // bodyHTML := service.xssSanitizer.Sanitize(service.xssSanitizer.Escape(input.Body))

        // message = support.MessageWithAuthor{
        //     Message: support.Message{
        //         ID:             uuid.New(),
        //         CreatedAt:      now,
        //         UpdatedAt:      now,
        //         Body:           input.Body,
        //         BodyHTML:       bodyHTML,
        //         AuthorID:       &me.ID,
        //         ConversationID: conversation.ID,
        //     },
        //     MessageAuthor: support.MessageAuthor{
        //         Name:     &me.Name,
        //         Avatar:   me.Avatar,
        //         Username: &me.Username,
        //     },
        // }
        // err = service.supportRepo.CreateMessage(ctx, service.db, message.Message)
        // if err != nil {
        //     return
        // }
        // return
        todo!();
    }
}
