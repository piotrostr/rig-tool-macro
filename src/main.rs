use rig_tool_macro::tool;

#[tool]
fn add(a: u64, b: u64) -> Result<u64, std::io::Error> {
    Ok(a + b)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Test implementation
    Ok(())
}
