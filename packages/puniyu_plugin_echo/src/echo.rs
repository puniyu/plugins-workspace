use puniyu_plugin::prelude::*;

#[command(
    name = "echo", 
    desc = "输出一段文本",
    alias = ["eh", "输出"],
    args = [
      { name = "msg", mode = "positional", arg_type = "string" }
    ]
)]
async fn echo(bot: &BotContext, ev: &MessageContext) -> HandlerResult {
    if let Some(msg) = ev.arg("msg").and_then(|v| v.as_str()) {
        bot.reply(msg.into()).await?;
    }
    Ok(().into())
}
