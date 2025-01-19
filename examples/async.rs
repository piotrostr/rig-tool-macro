#![allow(non_upper_case_globals)]

use anyhow::Result;
use rig::completion::Prompt;
use rig::providers;
use rig_tool_macro::tool;

#[tool]
async fn weather(city: String) -> Result<String> {
    // sleep for 1 second to simulate a long running task
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    Ok(format!("The weather in {} is 25°C", city))
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().pretty().init();
    let calculator_agent = providers::openai::Client::from_env()
        .agent(providers::openai::GPT_4O)
        .preamble("You are an agent with tools access, always use the tools")
        .max_tokens(1024)
        .tool(Weather)
        .build();

    for prompt in ["What tools do you have?", "What is the weather in London?"] {
        println!("User: {}", prompt);
        println!("Agent: {}", calculator_agent.prompt(prompt).await?);
    }

    Ok(())
}
