use actix::{Actor, StreamHandler, ActorContext, AsyncContext};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use std::time::{Duration, Instant};
pub struct MyWs{
    hb: Instant
}
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);
impl Actor for MyWs{
    type Context = ws::WebsocketContext<Self>;
    fn started(&mut self, ctx: &mut Self::Context){
        self.hb(ctx);
    }
}
impl MyWs{
    fn new() -> Self{
        Self{hb: Instant::now()}
    }
    fn hb(&self, ctx: &mut <Self as Actor>::Context){
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx|{
            if Instant::now().duration_since(act.hb) >CLIENT_TIMEOUT{
                println!("Websocket client timeout");
                ctx.stop();
                return;
            }
            ctx.ping(b"");
        });
    }
}
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs{
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>,ctx: &mut Self::Context){
        println!("WS: {:?}", msg);
        match msg{
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg)
            },
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}
pub async fn chat(r: HttpRequest, stream: web::Payload) -> Result<HttpResponse, actix_web::Error>{
    println!("{:?}", r);
    let res = ws::start(MyWs::new(),&r, stream);
    println!("{:?}",res);
    res
}