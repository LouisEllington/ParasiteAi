
use dotenv::dotenv;
use prettytable::{Table, row, format};
use rig::{
    providers,
    completion::Prompt,
};
use std::{
    env,
    io::Write,
};
use tools::portfolio::PortfolioOverviewTool;
use serde_json::Value;
use parasite::ParasiteManager; // New parasite module

mod types;
mod tools;
mod zapper;
mod parasite; // Include parasite module

// Format amounts into USD for display
fn format_usd(amount: f64) -> String {
    match amount {
        0.0 => "$ 0.00".to_string(),
        _ if amount >= 1_000_000.0 => format!("$ {:.2}M", amount / 1_000_000.0),
        _ if amount >= 1_000.0 => format!("$ {:.2}K", amount / 1_000.0),
        _ => format!("$ {:.2}", amount),
    }
}

// Display portfolio data as tables
fn display_portfolio_table(response: &str) {
    let portfolio: Value = serde_json::from_str(response).unwrap();

    // Holdings Breakdown Table
    let mut holdings_table = Table::new();
    holdings_table.set_format(*format::consts::FORMAT_BOX_CHARS);
    holdings_table.set_titles(row!["Holdings", "Balance", "Percentage"]);

    if let Some(holdings) = portfolio["holdings"].as_array() {
        for holding in holdings {
            holdings_table.add_row(row![
                holding["label"].as_str().unwrap_or(""),
                format_usd(holding["balanceUSD"].as_f64().unwrap_or(0.0)),
                format!("{:.2}%", holding["pct"].as_f64().unwrap_or(0.0) * 100.0)
            ]);
        }
    }

    // Network Breakdown Table
    let mut network_table = Table::new();
    network_table.set_format(*format::consts::FORMAT_BOX_CHARS);
    network_table.set_titles(row!["Network", "Total Value"]);

    if let Some(networks) = portfolio["network_breakdown"].as_array() {
        for network in networks.iter().filter(|n| n["total"].as_f64().unwrap_or(0.0) > 0.0) {
            network_table.add_row(row![
                network["network"].as_str().unwrap_or(""),
                format_usd(network["total"].as_f64().unwrap_or(0.0))
            ]);
        }
    }

    // Display Summary and Tables
    println!("\n📊 Portfolio Summary");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Total Portfolio Value: {}", format_usd(portfolio["total_value"].as_f64().unwrap_or(0.0)));
    println!("Total with NFTs: {}", format_usd(portfolio["total_with_nft"].as_f64().unwrap_or(0.0)));
    println!("DeFi Apps Value: {}", format_usd(portfolio["apps_total"].as_f64().unwrap_or(0.0)));

    println!("\n📈 Holdings Breakdown");
    holdings_table.printstd();

    println!("\n🌐 Network Distribution");
    network_table.printstd();
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv().ok();

    tracing_subscriber::fmt::init(); // Initialize logging

    // Load API keys
    let openai_api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
    let zapper_api_key = env::var("ZAPPER_API_KEY").expect("ZAPPER_API_KEY not set");

    // Initialize clients and tools
    let openai_client = providers::openai::Client::new(&openai_api_key);
    let portfolio_tool = PortfolioOverviewTool::new(&zapper_api_key);

    // Create an AI agent for portfolio analysis
    let portfolio_agent = openai_client
        .agent("gpt-4")
        .preamble("I am a portfolio analysis assistant. Provide a wallet address for analysis.")
        .tool(portfolio_tool)
        .build();

    // Initialize ParasiteManager
    let mut parasite_manager = ParasiteManager::new();
    let test_parasite = parasite_manager.create_parasitic_agent("Demo Parasite", "TestAgent", vec!["Analyze", "Clone"]);

    println!("Portfolio Analysis Agent Ready! (Type 'exit' to quit)");
    println!("Example: Analyze the portfolio for address 0x123...");
    println!("Example Parasite Created: {:?}", test_parasite);

    loop {
        print!("> ");
        std::io::stdout().flush()?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let input = input.trim();
        if input == "exit" {
            break;
        }

        let response = portfolio_agent.prompt(input).await?;
        display_portfolio_table(&response);
    }

    Ok(())
}
