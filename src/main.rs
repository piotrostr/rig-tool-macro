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

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Create OpenAI client
    let openai_client = providers::openai::Client::from_env();

    // Create agent with a single context prompt and two tools
    let calculator_agent = openai_client
        .agent(providers::openai::GPT_4O)
        .preamble("You are a calculator here to help the user perform arithmetic operations. Use the tools provided to answer the user's question.")
        .max_tokens(1024)
        .tool(ADD)     // Using the generated constant from our macro
        .tool(SUBTRACT) // Using the generated constant from our macro
        .build();

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

    Ok(())
}
