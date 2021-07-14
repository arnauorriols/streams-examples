use iota_streams::{
    app::transport::tangle::client::Client,
    app_channels::api::tangle::{Author, Bytes},
    core::{println, Result},
};

use crate::examples::{generate_seed, verify_messages};

pub fn example(node_url: &str) -> Result<()> {
    // Generate a unique seed for the author
    let seed = &generate_seed();

    // Create the Transport Client
    let client = Client::new_from_url(node_url);

    // Generate an Author
    let mut author = Author::new(seed, "utf-8", 1024, false, client.clone());

    // Create the channel with an announcement message. Make sure to save the resulting link somewhere,
    let announcement_link = author.send_announce()?;
    println!(
        "Announcement Link: {:?}\nTangle Index: {}\n",
        announcement_link, announcement_link
    );

    // Author will now send signed encrypted messages in a chain
    let msg_inputs = vec!["Send", "Some", "Messages"];

    let mut prev_msg_link = announcement_link;
    for input in &msg_inputs {
        let (msg_link, _seq_link) = author.send_signed_packet(
            &prev_msg_link,
            &Bytes::default(),
            &Bytes(input.as_bytes().to_vec()),
        )?;
        println!("Sent msg: {}", msg_link);
        prev_msg_link = msg_link;
    }

    // Export State of author
    let state = author.export("Password")?;
    // Write state to file
    std::fs::write("./author_state.bin", state)?;

    // Retrieve State from file
    let state = std::fs::read("./author_state.bin")?;

    // Import state
    let mut new_author = Author::import(&state, "Password", client)?;

    let (_last_msg_link, _seq) = new_author.send_signed_packet(
        &prev_msg_link,
        &Bytes::default(),
        &Bytes("One last message".as_bytes().to_vec()),
    )?;

    let msgs = author.fetch_next_msgs();
    if msgs.is_empty() {
        panic!("Old author could not fetch next message");
    }


    Ok(())
}
