use anyhow::Result;

mod examples;

const ENDPOINT_VAR: &str = "STREAMS_ENDPOINT";

#[tokio::main]
async fn main() -> Result<()> {
    let url = &std::env::var(ENDPOINT_VAR).unwrap_or_else(|_| {
        let url = "https://chrysalis-nodes.iota.org".to_string();
        println!(
            "\n!! Environment variable '{}' not found or invalid. \
            Using default {}",
            ENDPOINT_VAR, url
        );
        url
    });

    println!("\nStarting Examples");
    println!("---------------------------------------");
    println!("Single Publisher Examples");

    println!("\n---------------------------------------");
    println!("\nPublic - Single Branch - Single Publisher\n");
    examples::single_branch_public::example(url)?;

    println!("\n---------------------------------------");
    println!("\nPrivate - Single Branch - Single Publisher\n");
    examples::single_branch_private::example(url)?;

    println!("\n---------------------------------------");
    println!("\nMixed - Multi Branch - Single Publisher\n");
    examples::multi_branch_mixed_privacy::example(url)?;

    println!("\n---------------------------------------");
    println!("Multiple Publisher Examples");

    println!("\n---------------------------------------");
    println!("\nPrivate - Multi Branch - Single Publisher per Branch\n");
    examples::single_pub_per_branch::example(url)?;

    println!("\n---------------------------------------");
    println!("\nPrivate - Multi Branch - Multiple Publishers per Branch\n");
    examples::multi_pub_per_branch::example(url)?;

    println!("\n---------------------------------------");
    println!("Utility Examples");

    println!("\n---------------------------------------");
    println!("\nPrevious Message Retrieval\n");
    examples::fetch_prev::example(url)?;

    println!("\n---------------------------------------");
    println!("\nGranting and Revoking Access\n");
    examples::grant_and_revoke_access::example(url)?;

    println!("\n---------------------------------------");
    println!("\nUsing Public Keys for Keyload Generation\n");
    examples::pk_keyloads::example(url)?;

    println!("\n---------------------------------------");
    println!("\nUsing Pre Shared Keys for Keyload Generation\n");
    examples::psk_keyloads::example(url)?;

    println!("\n---------------------------------------");
    println!("\nState Recovery\n");
    examples::state_recovery::example(url)?;

    println!("\n---------------------------------------");
    println!("\nStateless Recovery\n");
    examples::stateless_recovery::example(url)?;

    println!("\n---------------------------------------");
    println!("Examples Complete");

    Ok(())
}
