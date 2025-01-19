#![allow(non_upper_case_globals)]

use rig::tool::Tool;
use serde::{Deserialize, Serialize};

use anyhow::Result;
use rig::completion::Prompt;
use rig::providers;
use rig_tool_macro::tool;

#[derive(Debug, Serialize, Deserialize)]
pub struct PowInput {
    a: i64,
    b: i64,
}

#[derive(Serialize)]
pub struct PowResult {
    input: (i64, i64),
    result: i64,
}

/// tool macro also works with structs (but not as good as with loose params)
#[tool]
fn power(pow_input: PowInput) -> Result<PowResult> {
    Ok(PowResult {
        input: (pow_input.a, pow_input.b),
        result: pow_input.a.pow(pow_input.b as u32),
    })
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().pretty().init();
    let calculator_agent = providers::openai::Client::from_env()
        .agent(providers::openai::GPT_4O)
        .preamble("You are an agent with tools access, always use them")
        .max_tokens(1024)
        .build();

    println!(
        "{}",
        serde_json::to_string_pretty(&Power.definition("".to_string()).await)?
    );

    for prompt in [
        "What tools do you have?",
        "Calculate 2 ^ 10", // structs, non-deterministic - sometimes work sometimes not
    ] {
        println!("User: {}", prompt);
        println!("Agent: {}", calculator_agent.prompt(prompt).await?);
    }

    Ok(())
}
