//empty file

use tonic::Status;
use tracing::warn;

use crate::{
    pb::{send_request::Msg, SendRequest, SendResponse, SmsMessage},
    NotificationService,
};

use super::{to_ts, Sender};

impl Sender for SmsMessage {
    async fn send(self, svc: NotificationService) -> Result<SendResponse, Status> {
        let message_id = self.message_id.clone();
        svc.sender.send(Msg::Sms(self)).await.map_err(|e| {
            warn!("Failed to send SMS message {}: {}", message_id, e);
            Status::internal("Failed to send SMS message")
        })?;
        Ok(SendResponse {
            message_id,
            timestamp: Some(to_ts()),
        })
    }
}

impl From<SmsMessage> for Msg {
    fn from(sms: SmsMessage) -> Self {
        Msg::Sms(sms)
    }
}

impl From<SmsMessage> for SendRequest {
    fn from(value: SmsMessage) -> Self {
        let msg = value.into();
        SendRequest { msg: Some(msg) }
    }
}

#[cfg(test)]
impl SmsMessage {
    pub fn fake() -> Self {
        use fake::{faker::phone_number::en::PhoneNumber, Fake};
        use uuid::Uuid;
        Self {
            message_id: Uuid::new_v4().to_string(),
            sender: PhoneNumber().fake(),
            recipients: vec![PhoneNumber().fake()],
            body: "Hello, world!".to_string(),
        }
    }
}
