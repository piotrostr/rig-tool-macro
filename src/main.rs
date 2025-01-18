use rig::completion::Prompt;
use rig::providers;
use rig_tool_macro::tool;

#[tool]
fn add(a: u64, b: u64) -> Result<u64, anyhow::Error> {
    Ok(a + b)
}

#[tool]
fn subtract(a: i64, b: i64) -> Result<i64, anyhow::Error> {
    Ok(a - b)
}

#[tool]
fn multiply(a: i64, b: i64) -> Result<i64, anyhow::Error> {
    Ok(a * b)
}

#[tool]
fn divide(a: i64, b: i64) -> Result<i64, anyhow::Error> {
    Ok(a / b)
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let calculator_agent = providers::openai::Client::from_env()
        .agent(providers::openai::GPT_4O)
        .preamble("You are a calculator. Use the tools provided")
        .max_tokens(1024)
        .tool(ADD)
        .tool(SUBTRACT)
        .tool(MULTIPLY)
        .tool(DIVIDE)
        .build();

    println!(
        "Calculator Agent: {:?}",
        calculator_agent.prompt("what tools do you have?").await?
    );

    // Prompt the agent and print the response
    println!("Calculate 5 - 2");
    println!(
        "Calculator Agent: {}",
        calculator_agent.prompt("Calculate 5 - 2").await?
    );

    println!("Calculate 5 + 2");
    println!(
        "Calculator Agent: {}",
        calculator_agent.prompt("Calculate 5 + 2").await?
    );

    println!("Calculate 5 * 2");
    println!(
        "Calculator Agent: {}",
        calculator_agent.prompt("Calculate 5 * 2").await?
    );

    println!("Calculate 5 / 2");
    println!(
        "Calculator Agent: {}",
        calculator_agent.prompt("Calculate 5 / 2").await?
    );

    Ok(())
}
