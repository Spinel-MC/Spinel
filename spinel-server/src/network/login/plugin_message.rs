use crate::network::client::instance::Client;
use spinel_core::network::clientbound::login::login_plugin_request::LoginPluginRequestPacket;
use std::collections::HashMap;
use std::io;
use std::sync::atomic::{AtomicI32, Ordering};
use std::time::{Duration, Instant};
use tokio::sync::oneshot;

static NEXT_LOGIN_PLUGIN_MESSAGE_ID: AtomicI32 = AtomicI32::new(0);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LoginPluginResponse {
    channel: String,
    data: Option<Vec<u8>>,
}

pub type LoginPluginResponseFuture = oneshot::Receiver<LoginPluginResponse>;

struct PendingLoginPluginRequest {
    channel: String,
    response_sender: oneshot::Sender<LoginPluginResponse>,
}

pub struct LoginPluginMessageProcessor {
    pending_requests: HashMap<i32, PendingLoginPluginRequest>,
    pending_request_deadline: Option<Instant>,
}

impl LoginPluginMessageProcessor {
    pub fn new() -> Self {
        Self {
            pending_requests: HashMap::new(),
            pending_request_deadline: None,
        }
    }

    pub fn request(
        &mut self,
        client: &mut Client,
        channel: impl Into<String>,
        request_payload: Option<Vec<u8>>,
    ) -> io::Result<LoginPluginResponseFuture> {
        let message_id = NEXT_LOGIN_PLUGIN_MESSAGE_ID.fetch_add(1, Ordering::Relaxed);
        let channel = channel.into();
        let (response_sender, response_receiver) = oneshot::channel();
        self.pending_requests.insert(
            message_id,
            PendingLoginPluginRequest {
                channel: channel.clone(),
                response_sender,
            },
        );
        self.pending_request_deadline = Some(Instant::now() + Duration::from_secs(5));
        let dispatch_result = LoginPluginRequestPacket {
            message_id,
            channel,
            data: request_payload.unwrap_or_default(),
        }
        .dispatch(client);
        if let Err(error) = dispatch_result {
            self.pending_requests.remove(&message_id);
            if self.pending_requests.is_empty() {
                self.pending_request_deadline = None;
            }
            return Err(error);
        }
        Ok(response_receiver)
    }

    pub fn complete(
        &mut self,
        message_id: i32,
        response_data: Option<Vec<u8>>,
    ) -> Result<(), UnexpectedLoginPluginResponseError> {
        let Some(pending_request) = self.pending_requests.remove(&message_id) else {
            return Err(UnexpectedLoginPluginResponseError { message_id });
        };
        if self.pending_requests.is_empty() {
            self.pending_request_deadline = None;
        }
        let _ = pending_request.response_sender.send(LoginPluginResponse {
            channel: pending_request.channel,
            data: response_data,
        });
        Ok(())
    }

    pub fn has_pending_requests(&self) -> bool {
        !self.pending_requests.is_empty()
    }

    pub fn has_timed_out(&self, now: Instant) -> bool {
        self.pending_request_deadline
            .is_some_and(|deadline| now >= deadline)
    }

    #[cfg(test)]
    pub(crate) fn pending_message_ids(&self) -> Vec<i32> {
        self.pending_requests.keys().copied().collect()
    }
}

impl Default for LoginPluginMessageProcessor {
    fn default() -> Self {
        Self::new()
    }
}

impl LoginPluginResponse {
    pub fn channel(&self) -> &str {
        &self.channel
    }

    pub fn data(&self) -> Option<&[u8]> {
        self.data.as_deref()
    }
}

#[derive(Debug, thiserror::Error)]
#[error("received unexpected login plugin response id {message_id}")]
pub struct UnexpectedLoginPluginResponseError {
    message_id: i32,
}
