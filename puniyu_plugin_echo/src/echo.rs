use puniyu_plugin::prelude::*;

#[command(name = "echo", desc = "输出一段文本", alias = ["eh"])]
#[arg(
    name = "msg",
    r#type = "string",
    mode = "positional",
    desc = "要输出的内容",
    required = true
)]
async fn echo(ctx: &MessageContext<'_>) -> puniyu_plugin::Result<CommandAction> {
    if let Some(msg) = ctx.arg("msg").and_then(|v| v.as_str()) {
        ctx.reply(msg).await?;
    }   
    Ok(().into())
}
