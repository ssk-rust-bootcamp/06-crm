use tonic::Status;
use tracing::warn;

use crate::pb::{send_request::Msg, EmailMessage, SendRequest, SendResponse};

use super::{to_ts, Sender};

//empty file
impl Sender for EmailMessage {
    async fn send(
        self,
        svc: crate::NotificationService,
    ) -> Result<crate::pb::SendResponse, tonic::Status> {
        let message_id = self.message_id.clone();

        svc.sender.send(Msg::Email(self)).await.map_err(|e| {
            warn!("Failed to send email message {:?}", e);
            Status::internal("Failed to send message")
        })?;

        Ok(SendResponse {
            message_id,
            timestamp: Some(to_ts()),
        })
    }
}
impl From<EmailMessage> for Msg {
    fn from(value: EmailMessage) -> Self {
        Msg::Email(value)
    }
}
impl From<EmailMessage> for SendRequest {
    fn from(value: EmailMessage) -> Self {
        let msg = value.into();
        SendRequest { msg: Some(msg) }
    }
}

#[cfg(test)]
impl EmailMessage {
    pub fn fake() -> Self {
        use fake::faker::internet::en::SafeEmail;
        use fake::Fake;
        use uuid::Uuid;
        Self {
            message_id: Uuid::new_v4().to_string(),
            subject: "Hello".to_string(),
            sender: SafeEmail().fake(),
            recipients: vec![SafeEmail().fake()],
            body: "Hello, this is a test email".to_string(),
        }
    }
}
