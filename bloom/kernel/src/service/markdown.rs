use crate::{Actor, Error, Service};

impl Service {
    pub async fn markdown(&self, actor: Actor, input: String) -> Result<String, Error> {
        let _ = self.current_user(actor)?;

        let html = self.render_markdown(&input).await?;

        Ok(html)
    }
}
