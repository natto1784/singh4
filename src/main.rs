mod commands;
mod handler;
mod lib;
use commands::general::*;
use commands::transcode::*;
use handler::Handler;
use serenity::{
    client::bridge::gateway::ShardManager,
    framework::{
        standard::{
            help_commands,
            macros::{group, help},
            Args, CommandGroup, CommandResult, HelpOptions,
        },
        StandardFramework,
    },
    http::Http,
    model::{channel::Message, id::UserId},
    prelude::*,
};
use std::{collections::HashSet, env, sync::Arc};
use tracing::error;

struct ShardManagerContainer;
impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

#[group]
#[commands(remux)]
struct Transcode;

#[group]
#[commands(ping)]
struct General;

#[help]
#[max_levenshtein_distance(2)]
#[indention_prefix = "+"]
#[lacking_role = "Nothing"]
#[wrong_channel = "Strike"]
async fn my_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Token daal madarchod");
    let http = Http::new_with_token(&token);
    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);
            match http.get_current_user().await {
                Ok(bot_id) => (owners, bot_id.id),
                Err(why) => panic!("Could not access the bot id: {:?}", why),
            }
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| c.owners(owners).prefix("xx"))
        .help(&MY_HELP)
        .group(&GENERAL_GROUP)
        .group(&TRANSCODE_GROUP);

    let mut client = Client::builder(&token)
        .framework(framework)
        .event_handler(Handler)
        .application_id(*bot_id.as_u64())
        .await
        .expect("Client no wokey");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
