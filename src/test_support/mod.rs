use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{Ordering, AtomicI8};
use actix::{Handler, Actor, StreamHandler};
use actix::prelude::*;
use actix_web_actors::ws;
use log::debug;
use serde::{Serialize, Deserialize};
use crate::websocket::Payload;

#[derive(Clone)]
pub struct WebSocket<T: 'static> where T: Serialize + Deserialize<'static> + Debug + Unpin {
    pub payloads: Arc<Mutex<Vec<T>>>,
    pub capacity: usize,
    pub count: Arc<AtomicI8>,
}

impl<T: 'static> Actor for WebSocket<T> where T: Serialize + Deserialize<'static> + Debug + Unpin {
    type Context = actix::Context<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        self.start(ctx);
    }
}

impl<T: 'static> Handler<Payload<T>> for WebSocket<T> where T: Serialize + Deserialize<'static> + Debug + Unpin {
    type Result = ();

    fn handle(&mut self, msg: Payload<T>, ctx: &mut Self::Context) {
        debug!("handle {:?}", msg.payload);
        let payloads = self.payloads
            .clone();
        let mut payloads = payloads
            .lock()
            .unwrap();
        payloads.push(msg.payload);
        self.count.fetch_add(1, Ordering::SeqCst);
        if payloads.len() >= self.capacity {
            ctx.stop();
        }
    }
}

impl<T: 'static> StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocket<T> where T: Serialize + Deserialize<'static> + Debug + Unpin {
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

impl<T: 'static> WebSocket<T> where T: Serialize + Deserialize<'static> + Debug + Unpin {
    fn start(&mut self, _ctx: &mut <Self as Actor>::Context) {
        debug!("WebSocket start called");
    }
}