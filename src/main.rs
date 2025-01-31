use async_trait::async_trait;
use maelstrom::protocol::Message;
use maelstrom::{done, Node, Result, Runtime};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub(crate) fn main() -> Result<()> {
    Runtime::init(try_main())
}

async fn try_main() -> Result<()> {
    let handler = Arc::new(Handler::default());
    Runtime::new().with_handler(handler).run().await
}

#[derive(Clone, Default)]
struct Handler {}

#[async_trait]
impl Node for Handler {
    async fn process(&self, runtime: Runtime, req: Message) -> Result<()> {
        let body: Request = req.body.as_obj()?;
        match body {
            Request::Echo { echo } => {
                runtime.reply(req, Response::EchoOk { echo }).await
            }
            _ => done(runtime, req),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
enum Request {
    // Init is sent to the node when it first joins the cluster, and it must be ACKed otherwise
    // maelstrom will error out.
    // calling `done(runtime, req)` will ACK it.
    Init {},

    Echo {
        echo: String,
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
enum Response {
    EchoOk {
        echo: String,
    },
}
