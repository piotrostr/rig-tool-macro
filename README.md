# rig-tool-macro

Rather than satisfy the rig `Tool` trait explicitly, you can annotate the tools
with the `#[tool]` attribute. This will automatically generate the `Tool`
implementation for you.

```rust
#[tool]
fn how_many_rs(s: String) -> anyhow::Result<usize> {
    Ok(s.chars()
        .filter(|c| *c == 'r' || *c == 'R')
        .collect::<Vec<_>>()
        .len())
}
```

and then call it:

```rust
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let agent = providers::openai::Client::from_env()
        .agent(providers::openai::GPT_4O)
        .tool(HowManyRs) // <- generated by the macro
        .max_tokens(1024)

    let res = agent.prompt("how many Rs are in the word strawberry?").await?;
    println!("{}", res);

    Ok(())
}
```

You can also include descriptions for the tools

```rust
#[tool(description = "
    it is important to describe ambiguous tools to LLMs

    https://docs.anthropic.com/en/docs/build-with-claude/tool-use#best-practices-for-tool-definitions

    this works multi-line too!
")]
fn asdf() -> Result<()> {
    Ok(())
}


The current implementation supports standard types and non-nested inputs

Structs and nested stuff might come at some point, for now tools have to take
top level inputs comprised of standard types

Adding the macros to `impl` methods is also not yet supported, those have to be
top-level functions due to the global generation of the `Tool` trait impl
```
