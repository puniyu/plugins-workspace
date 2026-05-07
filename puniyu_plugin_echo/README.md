# puniyu_plugin_echo

一个用于 `puniyu` 的简单回显插件，提供 `echo` 命令，把输入的文本原样输出。

> [!NOTE]
> 这个插件适合作为学习 `puniyu_plugin` 命令定义方式的最小示例，也可以作为你编写自定义插件时的起点。

## 功能特性

- 提供 `echo` 命令
- 支持命令别名 `eh`
- 接收一个必填位置参数 `msg`
- 将输入内容原样回复

## 命令说明

### `echo`

输出一段文本。

**别名：** `eh`

| 参数 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- |
| `msg` | `string` | 是 | 要输出的内容 |

## 使用示例

```text
echo 你好，世界
```

输出：

```text
你好，世界
```

也可以使用别名：

```text
eh hello
```

输出：

```text
hello
```

## 实现方式

插件使用 `puniyu_plugin` 提供的宏与预导入能力实现：

- 通过 `#[plugin]` 注册插件入口
- 通过 `#[command(...)]` 声明命令元数据
- 通过 `#[arg(...)]` 定义命令参数
- 在命令函数中调用 `ctx.reply(...)` 回复消息

核心实现可以参考：

- [src/lib.rs](src/lib.rs)
- [src/echo.rs](src/echo.rs)

## 开发信息

当前包信息来自工作区配置：

- 包名：`puniyu_plugin_echo`
- Rust Edition：`2024`
- 最低 Rust 版本：`1.88.0`
- 依赖：`puniyu_plugin = 0.8.3`

## 适用场景

- 验证插件是否被正确加载
- 快速测试命令注册与参数解析流程
- 学习 `puniyu` 插件的最小实现方式
