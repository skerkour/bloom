use stdx::chrono::Utc;

use super::{Service, UpdateGroupProfileInput};
use crate::{
    entities::{Group, Namespace},
    errors::kernel::Error,
    Actor,
};

impl Service {
    pub async fn update_group_profile(
        &self,
        actor: Actor,
        input: UpdateGroupProfileInput,
    ) -> Result<Group, crate::Error> {
        let actor = self.current_user(actor)?;

        let now = Utc::now();
        let mut namespace_to_update: Option<Namespace> = None;
        // var oldAvatarStorageKey string

        // check group membership
        let (mut group, _) = self
            .find_group_and_membership(&self.db, actor.id, input.group_id)
            .await?;

        group.updated_at = now;

        if let Some(mut name) = input.name {
            name = name.trim().to_string();
            self.validate_group_name(&name)?;
            group.name = name;
        }

        if let Some(mut description) = input.description {
            description = description.trim().to_string();
            self.validate_group_description(&description)?;
            group.description = description;
        }

        if let Some(mut path) = input.path {
            path = path.trim().to_lowercase();

            if path != group.path {
                let mut namespace = self.repo.find_namespace_by_id(&self.db, group.namespace_id).await?;
                namespace.updated_at = now;

                self.validate_namespace(&path)?;
                let namespace_exists = self.check_namespace_exists(&self.db, &path).await?;
                if namespace_exists {
                    return Err(Error::NamespaceAlreadyExists.into());
                }
                group.path = path.clone();
                namespace.path = path;
                namespace_to_update = Some(namespace);
            }
        }

        // if input.Avatar != nil {
        //     var avatarSize int
        //     avatarBuffer := make([]byte, kernel.AvatarMaxSize)
        //     avatarID := uuid.New().String()
        //     var img image.Image
        //     var processedImageBuffer bytes.Buffer

        //     if ret.Group.Avatar != nil {
        //         oldAvatarStorageKey = service.GetAvatardStorageKey(*ret.Group.Avatar)
        //     }

        //     avatarSize, err = io.ReadFull(input.Avatar, avatarBuffer)
        //     if err != nil && !errors.Is(err, io.ErrUnexpectedEOF) {
        //         errMessage := "kernel.UpdateGroupProfile: reading avatar"
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
        //         errMessage := "kernel.UpdateGroupProfile: decoding avatar"
        //         logger.Error(errMessage, log.Err("error", err))
        //         err = errors.Internal(errMessage, err)
        //         return
        //     }
        //     processedImage := imaging.Fill(img, kernel.AvatarSize, kernel.AvatarSize, imaging.Center, imaging.Lanczos)
        //     processedImageWriter := bufio.NewWriter(&processedImageBuffer)
        //     err = jpeg.Encode(processedImageWriter, processedImage, &jpeg.Options{Quality: kernel.AvatarJPEGQuality})
        //     if err != nil {
        //         errMessage := "kernel.UpdateGroupProfile: encoding avatar to JPEG"
        //         logger.Error(errMessage, log.Err("error", err))
        //         err = errors.Internal(errMessage, err)
        //         return
        //     }

        //     // upload to storage
        //     processedImageReader := bytes.NewReader(processedImageBuffer.Bytes()) // ugluy, not proud
        //     avatarStorageKey := service.GetAvatardStorageKey(avatarID)
        //     err = service.storage.CreateObject(ctx, avatarStorageKey, processedImageReader)
        //     if err != nil {
        //         errMessage := "kernel.UpdateGroupProfile: Uploading image to storage"
        //         logger.Error(errMessage, log.Err("error", err))
        //         err = errors.Internal(errMessage, err)
        //         return
        //     }

        //     ret.Group.Avatar = &avatarID
        // }

        let mut tx = self.db.begin().await?;

        self.repo.update_group(&mut tx, &group).await?;

        if let Some(namespace) = namespace_to_update {
            self.repo.update_namespace(&mut tx, &namespace).await?;
        }

        tx.commit().await?;

        Ok(group)
    }
}
