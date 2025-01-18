use anyhow::Result;
use rig::completion::Prompt;
use rig::providers;
use rig_tool_macro::tool;
use serde::Serialize;

#[tool]
fn add(a: u64, b: u64) -> Result<u64> {
    Ok(a + b)
}

#[tool]
fn subtract(a: i64, b: i64) -> Result<i64> {
    Ok(a - b)
}

#[tool]
fn multiply(a: i64, b: i64) -> Result<i64> {
    Ok(a * b)
}

#[tool]
fn divide(a: i64, b: i64) -> Result<i64> {
    Ok(a / b)
}

#[derive(Serialize)]
pub struct PowResult {
    input: (i64, i64),
    result: i64,
}

/// tool macro also works with struct outputs!
#[tool]
fn power(a: i64, b: i64) -> Result<PowResult> {
    Ok(PowResult {
        input: (a, b),
        result: a.pow(b as u32),
    })
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let calculator_agent = providers::openai::Client::from_env()
        .agent(providers::openai::GPT_4O)
        .preamble("You are a calculator. Use the tools provided")
        .max_tokens(1024)
        .tool(ADD)
        .tool(SUBTRACT)
        .tool(MULTIPLY)
        .tool(DIVIDE)
        .tool(POWER)
        .build();

    for prompt in [
        "What tools do you have?",
        "Calculate 5 - 2",
        "Calculate 5 + 2",
        "Calculate 5 * 2",
        "Calculate 5 / 2",
        "Calculate 5 ^ 2",
    ] {
        println!("User: {}", prompt);
        println!("Agent: {}", calculator_agent.prompt(prompt).await?);
    }

    Ok(())
}
