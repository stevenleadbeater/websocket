use actix::MailboxError;
use log::info;

pub trait WebSocketError<T: 'static> where T: Send {
    fn is_websocket_closed(&self) -> bool;
}

impl<T: 'static> WebSocketError<T> for Result<T, MailboxError> where T: Send {
    fn is_websocket_closed(&self) -> bool {
        if let Err(error) = self {
            info!("Cannot send on websocket: {:?}", error);
            if let MailboxError::Closed = error {
                info!("Stop");
                return true;
            }
        }
        false
    }
}


#[cfg(test)]
mod tests {
    use actix::MailboxError;
    use crate::websocket_error::WebSocketError;

    #[derive(Debug)]
    struct Record1 {}

    #[test]
    fn closed_error_returns_true() {
        let payload: Result<Record1, MailboxError> = Err(MailboxError::Closed);
        assert!(payload.is_websocket_closed());
    }

    #[test]
    fn timeout_error_returns_false() {
        let payload: Result<Record1, MailboxError> = Err(MailboxError::Timeout);
        assert!(!payload.is_websocket_closed());
    }

    #[test]
    fn ok_returns_false() {
        let payload: Result<Record1, MailboxError> = Ok(Record1 {});
        assert!(!payload.is_websocket_closed());
    }
}