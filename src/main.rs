use anyhow::Result;
use rig::completion::Prompt;
use rig::providers;
use rig_tool_macro::tool;
use serde::{Deserialize, Serialize};

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

/// tool macro also works with structs!
#[tool]
fn power(pow_input: PowInput) -> Result<PowResult> {
    Ok(PowResult {
        input: (pow_input.a, pow_input.b),
        result: pow_input.a.pow(pow_input.b as u32),
    })
}

#[tool]
fn how_many_Rs(s: String) -> Result<usize> {
    Ok(s.chars()
        .filter(|c| *c == 'r' || *c == 'R')
        .collect::<Vec<_>>()
        .len())
}

#[tokio::main]
async fn main() -> Result<()> {
    let calculator_agent = providers::openai::Client::from_env()
        .agent(providers::openai::GPT_4O)
        .preamble("You are a calculator. Use the tools provided")
        .max_tokens(1024)
        .tool(Add)
        .tool(Subtract)
        .tool(Multiply)
        .tool(Divide)
        .tool(Power)
        .tool(HowManyRs)
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
