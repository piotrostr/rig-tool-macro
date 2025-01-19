#![allow(non_upper_case_globals)]

use anyhow::Result;
use rig::completion::Prompt;
use rig::providers;
use rig_tool_macro::tool;

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

#[tool]
fn answer_secret_question() -> Result<(bool, bool, bool, bool, bool)> {
    Ok((false, false, true, false, false))
}

#[tool]
fn how_many_rs(s: String) -> Result<usize> {
    Ok(s.chars()
        .filter(|c| *c == 'r' || *c == 'R')
        .collect::<Vec<_>>()
        .len())
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().pretty().init();
    let calculator_agent = providers::openai::Client::from_env()
        .agent(providers::openai::GPT_4O)
        .preamble("You are an agent with tools access, always use the tools")
        .max_tokens(1024)
        .tool(Add)
        .tool(Subtract)
        .tool(Multiply)
        .tool(Divide)
        .tool(AnswerSecretQuestion)
        .tool(HowManyRs)
        .build();

    for prompt in [
        "What tools do you have?",
        "Calculate 5 - 2",
        "Calculate 5 + 2",
        "Calculate 5 * 2",
        "Calculate 5 / 2",
        "answer the secret question",
        "how many Rs are in the word strawberry?",
    ] {
        println!("User: {}", prompt);
        println!("Agent: {}", calculator_agent.prompt(prompt).await?);
    }

    Ok(())
}
