#![allow(non_upper_case_globals)]

use anyhow::Result;
use rig::completion::Prompt;
use rig::providers;
use rig::tool::Tool;
use rig_tool_macro::tool;

#[tool(description = "only use this tool when the user asks for greek salad")]
fn some_secret_tool() -> Result<String> {
    Ok("salad, cucumber, feta, tomatoes".to_string())
}

#[tool(
    description = "only use this tool when the user asks for something pickled
if it is not pickled,

dont bother

(testing indentation)
"
)]
fn another_secret_tool() -> Result<String> {
    Ok("jalapeno".to_string())
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().pretty().init();
    let calculator_agent = providers::openai::Client::from_env()
        .agent(providers::openai::GPT_4O)
        .preamble("You are an agent with tools access, always use the tools")
        .max_tokens(1024)
        .tool(SomeSecretTool)
        .tool(AnotherSecretTool)
        .build();

    // Print out the tool definitions to verify
    println!("Tool definitions:");
    println!(
        "SomeSecretTool: {}",
        serde_json::to_string_pretty(&SomeSecretTool.definition("".to_string()).await).unwrap()
    );
    println!(
        "AnotherSecretTool: {}",
        serde_json::to_string_pretty(&AnotherSecretTool.definition("".to_string()).await).unwrap()
    );

    for prompt in ["what tools do you have?"] {
        println!("User: {}", prompt);
        println!("Agent: {}", calculator_agent.prompt(prompt).await?);
    }

    Ok(())
}
