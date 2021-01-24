use stdx::{chrono::Utc, crypto, log::error, rand, sync::threadpool::spawn_blocking, ulid::Ulid};

use super::{Service, UpdateMyProfileInput};
use crate::{
    consts,
    entities::{Namespace, PendingEmail, User},
    errors::kernel::Error,
    Actor,
};

impl Service {
    pub async fn update_my_profile(&self, actor: Actor, input: UpdateMyProfileInput) -> Result<User, crate::Error> {
        let mut actor = self.current_user(actor)?;

        let mut namespace: Option<Namespace> = None;
        let mut pending_email: Option<PendingEmail> = None;
        let mut pending_email_code = String::new();
        let now = Utc::now();
        // var oldAvatarStorageKey string

        actor.updated_at = now;

        if let Some(mut description) = input.description {
            description = description.trim().to_string();
            self.validate_user_description(&description)?;
            actor.description = description;
        }

        if let Some(mut name) = input.name {
            name = name.trim().to_string();
            self.validate_user_name(&name)?;
            actor.name = name;
        }

        if let Some(mut username) = input.username {
            username = username.trim().to_lowercase();

            if username != actor.username {
                let mut namespace_to_update = self.repo.find_namespace_by_id(&self.db, actor.namespace_id).await?;
                namespace_to_update.updated_at = now;

                self.validate_username(&username)?;
                let namespace_exists = self.check_namespace_exists(&self.db, &username).await?;
                if namespace_exists {
                    return Err(Error::UsernameAlreadyExists.into());
                }
                actor.username = username.clone();
                namespace_to_update.path = username;
                namespace = Some(namespace_to_update);
            }
        }

        if let Some(mut email) = input.email {
            email = email.trim().to_lowercase();
            if email != actor.email {
                self.validate_email(&email, true)?;

                let (code, code_hash) = spawn_blocking(|| {
                    let code = rand::alphabet(consts::CODE_ALPHABET, consts::REGISTER_CODE_LENGTH);

                    let code_hash = match crypto::hash_password(&code) {
                        Ok(res) => res,
                        Err(err) => {
                            error!("kernel.update_my_profile: hashing code: {}", &err);
                            return Err(crate::Error::Internal(err.to_string()));
                        }
                    };

                    Ok((code, code_hash))
                })
                .await??;

                pending_email_code = code;

                pending_email = Some(PendingEmail {
                    id: Ulid::new().into(),
                    created_at: now,
                    updated_at: now,
                    email,
                    code_hash,
                    failed_attempts: 0,
                    user_id: actor.id,
                });
            }
        }

        // if input.Avatar != nil {
        //     var avatarSize int
        //     avatarBuffer := make([]byte, kernel.AvatarMaxSize)
        //     avatarID := uuid.New().String()
        //     var img image.Image
        //     var processedImageBuffer bytes.Buffer

        //     if actor.Avatar != nil {
        //         oldAvatarStorageKey = service.GetAvatardStorageKey(*actor.Avatar)
        //     }

        //     avatarSize, err = io.ReadFull(input.Avatar, avatarBuffer)
        //     if err != nil && !errors.Is(err, io.ErrUnexpectedEOF) {
        //         errMessage := "kernel.UpdateMyProfile: reading avatar"
        //         logger.Error(errMessage, log.Err("error", err))
        //         err = errors.Internal(errMessage, err)
        //         return
        //     }
        //     if avatarSize >= kernel.AvatarMaxSize {
        //         err = kernel.ErrAvatarIsTooLarge
        //         return
        //     }
        //     avatarImageFormat := service.GuessImageFormat(avatarBuffer)
        //     if avatarImageFormat != kernel.ImageFormatJpeg && avatarImageFormat != kernel.ImageFormatPng {
        //         err = kernel.ErrAvatarInvalidFormat
        //         return
        //     }

        //     // resize and convert avatar
        //     avatarReader := bytes.NewReader(avatarBuffer)
        //     img, _, err = image.Decode(avatarReader)
        //     if err != nil {
        //         errMessage := "kernel.UpdateMyProfile: decoding avatar"
        //         logger.Error(errMessage, log.Err("error", err))
        //         err = errors.Internal(errMessage, err)
        //         return
        //     }
        //     processedImage := imaging.Fill(img, kernel.AvatarSize, kernel.AvatarSize, imaging.Center, imaging.Lanczos)
        //     processedImageWriter := bufio.NewWriter(&processedImageBuffer)
        //     err = jpeg.Encode(processedImageWriter, processedImage, &jpeg.Options{Quality: kernel.AvatarJPEGQuality})
        //     if err != nil {
        //         errMessage := "kernel.UpdateMyProfile: encoding avatar to JPEG"
        //         logger.Error(errMessage, log.Err("error", err))
        //         err = errors.Internal(errMessage, err)
        //         return
        //     }

        //     // upload to storage
        //     processedImageReader := bytes.NewReader(processedImageBuffer.Bytes()) // ugluy, not proud
        //     avatarStorageKey := service.GetAvatardStorageKey(avatarID)
        //     err = service.storage.CreateObject(ctx, avatarStorageKey, processedImageReader)
        //     if err != nil {
        //         errMessage := "kernel.UpdateMyProfile: Uploading image to storage"
        //         logger.Error(errMessage, log.Err("error", err))
        //         err = errors.Internal(errMessage, err)
        //         return
        //     }

        //     actor.Avatar = &avatarID
        // }

        let mut tx = self.db.begin().await?;

        self.repo.update_user(&mut tx, &actor).await?;

        if let Some(namespace) = namespace {
            self.repo.update_namespace(&mut tx, &namespace).await?;
        }

        if let Some(ref pending_email) = pending_email {
            self.repo.create_pending_email(&mut tx, pending_email).await?;
        }

        tx.commit().await?;

        if let Some(pending_email) = pending_email {
            let job = crate::domain::messages::Message::KernelSendVerifyEmailEmail {
                email: pending_email.email,
                name: actor.name.clone(),
                code: pending_email_code,
            };
            let _ = self.queue.push(job, None).await; // TODO: log error?
        }

        // if oldAvatarStorageKey != "" {
        //     storageErr := service.storage.DeleteObject(ctx, oldAvatarStorageKey)
        //     if storageErr != nil {
        //         errMessage := "kernel.UpdateMyProfile: deleting old avatar"
        //         logger.Error(errMessage, log.Err("error", storageErr))
        //     }
        // }

        Ok(actor)
    }
}
