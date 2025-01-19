use anyhow::Result;
use rig::completion::Prompt;
use rig::providers;
use rig_tool_macro::tool;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct Exchange {
    liquidity: u64,
}

// Input structs for the methods
#[derive(Debug, Serialize, Deserialize)]
pub struct AddLiquidityInput {
    amount: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoveLiquidityInput {
    amount: u64,
}

impl Exchange {
    pub fn new(initial_liquidity: u64) -> Self {
        Self {
            liquidity: initial_liquidity,
        }
    }

    #[tool]
    pub fn add_liquidity(&mut self, input: AddLiquidityInput) -> Result<u64> {
        println!("[TOOL] Adding liquidity: {}", input.amount);
        self.liquidity += input.amount;
        Ok(self.liquidity)
    }

    #[tool]
    pub fn remove_liquidity(&mut self, input: RemoveLiquidityInput) -> Result<u64> {
        println!("[TOOL] Removing liquidity: {}", input.amount);
        if input.amount <= self.liquidity {
            self.liquidity -= input.amount;
            Ok(self.liquidity)
        } else {
            Err(anyhow::anyhow!("Insufficient liquidity"))
        }
    }

    #[tool]
    pub fn get_liquidity(&self) -> Result<u64> {
        println!("[TOOL] Getting current liquidity");
        Ok(self.liquidity)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let exchange = Exchange::new(1000);

    let agent = providers::openai::Client::from_env()
        .agent(providers::openai::GPT_4O)
        .preamble("You are an exchange manager with tools access")
        .max_tokens(1024)
        .tool(AddLiquidity)
        .tool(RemoveLiquidity)
        .tool(GetLiquidity)
        .build();

    for prompt in [
        "What tools do you have?",
        "What's the current liquidity?",
        "Add 500 units of liquidity",
        "Remove 200 units of liquidity",
    ] {
        println!("User: {}", prompt);
        println!("Agent: {}", agent.prompt(prompt).await?);
    }

    Ok(())
}
