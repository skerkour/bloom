use super::SendChatboxMessageInput;
use crate::{entities::Message, Service};
use kernel::Actor;

impl Service {
    pub async fn send_chatbox_message(
        &self,
        actor: Actor,
        input: SendChatboxMessageInput,
    ) -> Result<Message, kernel::Error> {
        let anonymous_id = self.kernel_service.current_anonymous_id(actor)?;
        // findVisitorInput := growth.FindOrCreateVisitorInput{
        //     AnonymousID: anonymousID,
        //     ProjectID:   input.ProjectID,
        // }
        // visitor, err := service.growthService.FindOrCreateVisitor(ctx, service.db, findVisitorInput)
        // if err != nil {
        //     return
        // }

        // contact, visitor, err := service.growthService.FindOrCreateContactByVisitor(ctx, service.db, visitor)
        // if err != nil {
        //     return
        // }

        // if contact.ProjectID != input.ProjectID {
        //     err = kernel.ErrPermissionDenied
        //     return
        // }

        // findOrCreateConversationInput := support.FindOrCreateConversationInput{
        //     ContactID: contact.ID,
        //     ProjectID: input.ProjectID,
        // }
        // conversation, err := service.FindOrCreateConversation(ctx, service.db, findOrCreateConversationInput)
        // if err != nil {
        //     return
        // }

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
        //         AuthorID:       nil,
        //         ConversationID: conversation.ID,
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
