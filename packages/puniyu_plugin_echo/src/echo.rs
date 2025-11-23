use puniyu_plugin::prelude::*;

#[command(
    name = "echo", 
    desc = "输出一段文本",
    args = ["msg"]
)]
async fn echo(bot: &BotContext, ev: &MessageContext) -> HandlerResult {
    if let Some(msg) = ev.arg("msg") && !msg.is_empty() {
        bot.reply(msg.into()).await?;
    }
    Ok(().into())
}
