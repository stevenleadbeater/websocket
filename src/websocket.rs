use actix::prelude::*;
use actix_web_actors::ws;
use serde::{Serialize, Deserialize};
use std::fmt::Debug;
use log::debug;

pub struct WebSocket { }

impl Actor for WebSocket {
    type Context = ws::WebsocketContext<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        self.start(ctx);
    }
}

#[derive(Message, Deserialize, Debug)]
#[rtype(result = "()")]
pub struct Payload<T> {
    pub payload: T,
}

impl<T> Handler<Payload<T>> for WebSocket where T: Serialize + Deserialize<'static> + Debug {
    type Result = ();

    fn handle(&mut self, msg: Payload<T>, ctx: &mut Self::Context) {
        debug!("handle {:?}", msg.payload);
        ctx.text(serde_json::to_string(&msg.payload).expect("Cannot serialize"));
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocket {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        // process websocket messages
        debug!("WS: {:?}", msg);
        match msg {
            Ok(ws::Message::Text(_)) => {
                debug!("send hb");
            }
            Ok(ws::Message::Close(_)) => {
                ctx.stop();
            }
            _ => {
                ctx.stop();
            }
        }
    }
}

impl WebSocket {
    fn start(&mut self, _ctx: &mut <Self as Actor>::Context) {
        debug!("WebSocket start called");
    }
}
