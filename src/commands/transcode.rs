use crate::lib::ffmpeg::FfmpegTranscode;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    http::AttachmentType,
    model::prelude::*,
    prelude::*,
};
use std::{fs::remove_file, path::Path, process::ExitStatus};
use tokio::fs::File;

#[command]
#[aliases("mux")]
pub async fn remux(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let q: Vec<&str> = args.raw().collect::<Vec<&str>>();
    if q.len() < 2 || q.len() > 4 {
        msg.reply(
            ctx,
            "Please use the proper syntax: `xxremux <audio> <video> [audio_codec] [video_codec]` or attach something",
        )
        .await?;
        return Ok(());
    }

    let mut trans = FfmpegTranscode::default();

    trans.add_input(q[0]).add_input(q[1]);

    if q.len() > 2 {
        trans.set_acodec(q[2]);
    }

    if q.len() > 3 {
        trans.set_vcodec(q[3]);
    }

    let ext = Path::new(&q[1]).extension().unwrap().to_str().unwrap();
    let output = format!("/tmp/{}{}.{}", "singh4-", msg.id, ext);

    trans
        .add_flag("shortest")
        .set_output(&output)
        .add_arg("map", "0:a:0")
        .add_arg("map", "1:v:0");

    let mut m = msg.reply_ping(ctx, "Working").await?;
    let exit_code: ExitStatus = trans.run();
    let file: File = File::open(&output).await?;

    if !exit_code.success() {
        msg.reply(ctx, "Some error occurred, please check the inputs")
            .await?;
    } else if file.metadata().await.unwrap().len() > 8 * 1024 * 1024 {
        msg.reply(ctx, "Output file larger than 8 MB").await?;
    } else {
        m.edit(ctx, |m| m.content("Uploading")).await?;
        msg.channel_id
            .send_message(ctx, |m| {
                m.add_file(AttachmentType::File {
                    file: &file,
                    filename: format!("{}.{}", msg.id, ext),
                })
                .content(msg.author.mention())
            })
            .await?;
        m.delete(ctx).await?;
    }

    remove_file(output)?;

    Ok(())
}
