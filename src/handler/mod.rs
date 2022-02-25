use serenity::{
    async_trait,
    model::{event::ResumedEvent, gateway::Ready},
    prelude::*,
};
use tracing::info;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} connected bhay", ready.user.name);
    }
    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("how th when the");
    }
}
