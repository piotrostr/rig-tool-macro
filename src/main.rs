#![allow(non_upper_case_globals)]

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

/// tool macro also works with structs (but not as good as with loose params)
#[tool]
fn power(pow_input: PowInput) -> Result<PowResult> {
    Ok(PowResult {
        input: (pow_input.a, pow_input.b),
        result: pow_input.a.pow(pow_input.b as u32),
    })
}

#[tool]
fn how_many_rs(s: String) -> Result<usize> {
    println!("Counting Rs in '{}'", s);
    Ok(s.chars()
        .filter(|c| *c == 'r' || *c == 'R')
        .collect::<Vec<_>>()
        .len())
}

#[tool]
fn answer_secret_question() -> Result<(bool, bool, bool, bool, bool)> {
    println!("Answering secret question");
    Ok((false, false, true, false, false))
}

#[tool]
fn sum_numbers(numbers: Vec<i64>) -> Result<i64> {
    Ok(numbers.iter().sum())
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
        .tool(AnswerSecretQuestion)
        .tool(SumNumbers)
        .build();

    for prompt in [
        "What tools do you have?",
        "Calculate 5 - 2",
        "Calculate 5 + 2",
        "Calculate 5 * 2",
        "Calculate 5 / 2",
        "Calculate 2 ^ 10",
        "Sum for me 1, 2, 3, 4, 5",
        "how many Rs are in the word strawberry?",
        "answer the secret question",
    ] {
        println!("User: {}", prompt);
        println!("Agent: {}", calculator_agent.prompt(prompt).await?);
    }

    Ok(())
}
