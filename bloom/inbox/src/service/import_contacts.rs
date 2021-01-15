use super::ImportContactsInput;
use crate::{consts, entities::Contact, Error, Service};
use kernel::Actor;

impl Service {
    pub async fn import_contacts(
        &self,
        actor: Actor,
        input: ImportContactsInput,
    ) -> Result<Vec<Contact>, kernel::Error> {
        let actor = self.kernel_service.current_user(actor)?;

        let namespace_id = input.namespace_id;

        self.kernel_service
            .check_namespace_membership(&self.db, actor.id, namespace_id)
            .await?;

        let list = if let Some(list_id) = input.list_id {
            let list = self.repo.find_newsletter_list_by_id(&self.db, list_id).await?;
            Some(list)
        } else {
            None
        };

        if let Some(ref list) = list {
            if list.namespace_id != namespace_id {
                return Err(Error::PermissionDenied.into());
            }
        }

        if input.conatcts_csv.len() > consts::MAX_IMPORT_CONTACTS_CSV_LENGTH {
            return Err(Error::ContactsCsvTooLarge.into());
        }

        // csvReader := csv.NewReader(strings.NewReader(input.Contacts))
        // contactsRecords, err := csvReader.ReadAll()
        // if err != nil {
        //     const errMessage = "growth.ImportContacts: parsing contacts CSV"
        //     logger.Warn(errMessage, log.Err("error", err), log.UUID("project.id", project.ID))
        //     err = growth.ErrContactsMalformed
        //     return
        // }

        // if len(contactsRecords) == 0 {
        //     return
        // }

        // csvColumnsCount := len(contactsRecords[0])
        // if csvColumnsCount != 1 && csvColumnsCount != 2 {
        //     err = growth.ErrContactsMalformed
        //     return
        // }

        // // dedup
        // contactsMap := make(map[string]string, 0)
        // for _, row := range contactsRecords {
        //     if len(row) != csvColumnsCount {
        //         err = growth.ErrContactsMalformed
        //         return
        //     }

        //     if csvColumnsCount == 1 {
        //         email := strings.ToLower(strings.TrimSpace(row[0]))
        //         contactsMap[email] = ""
        //     } else {
        //         // csvColumnsCount == 2: name,email
        //         email := strings.ToLower(strings.TrimSpace(row[1]))
        //         name := strings.TrimSpace(row[0])
        //         contactsMap[email] = name
        //     }
        // }

        // for contactToImportEmail, contactToImportName := range contactsMap {
        //     err = service.kernelService.ValidateEmail(contactToImportEmail, false)
        //     if err != nil {
        //         return
        //     }

        //     err = service.ValidateContactName(contactToImportName)
        //     if err != nil {
        //         return
        //     }
        // }

        // defaultList, err := service.growthRepo.FindDefaultListForProject(ctx, service.db, project.ID)
        // if err != nil {
        //     return
        // }

        // err = service.db.Transaction(ctx, func(tx db.Queryer) (err error) {
        //     for contactToImportEmail, contactToImportName := range contactsMap {
        //         var contact growth.Contact
        //         now := time.Now().UTC()

        //         contact, err = service.growthRepo.FindContactByEmail(ctx, tx, project.ID, contactToImportEmail)
        //         if err != nil {
        //             if errors.Is(err, growth.ErrContactNotFound) {
        //                 // create contact
        //                 err = nil
        //                 contact = growth.Contact{
        //                     ID:        uuid.New(),
        //                     CreatedAt: now,
        //                     UpdatedAt: now,
        //                     Name:      contactToImportName,
        //                     Email:     contactToImportEmail,
        //                     Country:   "",
        //                     Plan:      "",
        //                     UserID:    "",

        //                     ProjectID: project.ID,
        //                 }
        //                 err = service.growthRepo.CreateContact(ctx, tx, contact)
        //                 if err != nil {
        //                     return
        //                 }

        //                 relation := growth.ListContactRelation{
        //                     ContactID: contact.ID,
        //                     ListID:    defaultList.ID,
        //                 }
        //                 err = service.growthRepo.CreateListContactRelation(ctx, tx, relation)
        //                 if err != nil {
        //                     return
        //                 }

        //                 contacts = append(contacts, contact)

        //             } else {
        //                 // other than not found error, return
        //                 return
        //             }
        //         } else if contactToImportName != "" && contactToImportName != contact.Name {
        //             // if contact exists and contactToImportName != ""
        //             contact.Name = contactToImportName
        //             err = service.growthRepo.UpdateContact(ctx, tx, contact)
        //             if err != nil {
        //                 return
        //             }
        //             contacts = append(contacts, contact)
        //         }
        //     }

        //     return
        // })
        // if err != nil {
        //     return
        // }
        todo!();
    }
}
