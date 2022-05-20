use forlok::*;
use tokio::sync::{RwLock, mpsc};
use tokio::time::timeout;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

/// Some server context
#[derive(Clone, Default)]
pub struct Context {
    sessions: Arc<RwLock<HashMap<String, mpsc::Sender<String>>>>    
}

pub enum MessageError {
    ChannelClosed,
    SessionIdNotActive,
}


impl Context {


    async fn start_session(&self, id: String) {
        let mut w = self.sessions.write().await;
        let (tx, mut rx)  = mpsc::channel(16);
        w.insert(id.clone(), tx.clone());
        tokio::task::spawn(async move {
            let mut state = GameState::default();
            while let Ok(Some(msg)) = timeout(Duration::new(600,0), rx.recv()).await {
                update(&mut state, msg);
            }

            // Save game state here if we've timed out (message not came in 10 minutes). 
            // 
            // After this exits receiver will be dropped and this task will disappear
        });
        
        let sesh_handle = self.sessions.clone();
        // I just though of this
        tokio::task::spawn(async move {
            // Once receiver closes this statement moves on and we remove the session from our list
            // of active sessions in the map
            tx.closed().await;
            let mut w = sesh_handle.write().await;
            w.remove(&id);
        });
    }


    async fn send_message(&self, id: &str, message: String) -> Result<(), MessageError> {
        let r = self.sessions.read().await;

        if let Some(sender) = r.get(id) {
            sender.send(message).await.map_err(|_| MessageError::ChannelClosed)
        } else {
            Err(MessageError::SessionIdNotActive)
        }
    }

}

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    // So now create the context, have your discord handling stuff and just call `start_session`
    // for new sessions or `send_message` for messages incoming from a channel and the messages
    // will be received by `update(&mut GameState, msg)` in the order the channel gets them.
    //
    // And every session with this after 10 minutes of inactivity will cease to exist
}
