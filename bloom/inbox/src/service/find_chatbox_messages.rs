use super::FindChatboxMessagesInput;
use crate::{entities::Message, Service};
use kernel::Actor;

impl Service {
    pub async fn find_chatbox_messages(
        &self,
        _actor: Actor,
        _input: FindChatboxMessagesInput,
    ) -> Result<Vec<Message>, kernel::Error> {
        todo!();
        // httpCtx := httputil.HTTPCtxFromCtx(ctx)
        // if httpCtx.AnonymousID == nil {
        //     err = support.ErrInvalidAnonymousUser
        //     return
        // }
        // anonymousID := *httpCtx.AnonymousID

        // findVisitorInput := growth.FindOrCreateVisitorInput{
        //     AnonymousID: anonymousID,
        //     ProjectID:   input.ProjectID,
        // }
        // visitor, err := service.growthService.FindOrCreateVisitor(ctx, service.db, findVisitorInput)
        // if err != nil {
        //     return
        // }

        // conversation, err := service.supportRepo.FindConversationByVisitorID(ctx, service.db, visitor.ID)
        // if err != nil {
        //     if errors.Is(err, support.ErrConversationNotFound) {
        //         messages = []support.MessageWithAuthor{}
        //         err = nil
        //     }
        //     return
        // }

        // messages, err = service.supportRepo.FindMessagesForConversation(ctx, service.db, conversation.ID)
        // if err != nil {
        //     return
        // }
    }
}
