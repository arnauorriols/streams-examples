use iota_streams::{
    app::transport::tangle::client::Client,
    app_channels::api::tangle::{Address, Author, Bytes, Subscriber},
    core::{println, Result},
};

use crate::examples::{generate_seed, verify_messages};

pub fn example(node_url: &str) -> Result<()> {
    // Create the Transport Client
    let client = Client::new_from_url(node_url);

    // Generate a unique seed for the author
    let seed = &generate_seed();

    // Generate an Author
    let mut author = Author::new(seed, "utf-8", 1024, false, client.clone());

    // Create the channel with an announcement message. Make sure to save the resulting link somewhere,
    let announcement_link = author.send_announce()?;
    // This link acts as a root for the channel itself
    let ann_link_string = announcement_link.to_string();
    println!(
        "Announcement Link: {}\nTangle Index: {}\n",
        ann_link_string, announcement_link
    );

    // Author will now send signed encrypted messages in a chain
    let msg_inputs = [
        "These", "Messages", "Will", "Be", "Masked", "And", "Sent", "In", "A", "Chain",
    ];

    let mut prev_msg_link = announcement_link;
    for input in msg_inputs {
        let (msg_link, _seq_link) = author.send_signed_packet(
            &prev_msg_link,
            &Bytes::default(),
            &Bytes(input.as_bytes().to_vec()),
        )?;
        println!("Sent msg: {}", msg_link);
        prev_msg_link = msg_link;
    }

    // ------------------------------------------------------------------
    // In their own separate instances generate the subscriber(s) that will be attaching to the channel
    let mut subscriber = Subscriber::new("SubscriberA", "utf-8", 1024, client);

    // Generate an Address object from the provided announcement link string from the Author
    let ann_link_split = ann_link_string.split(':').collect::<Vec<&str>>();
    let ann_address = Address::from_str(ann_link_split[0], ann_link_split[1])?;

    // Receive the announcement message to start listening to the channel
    subscriber.receive_announcement(&ann_address)?;

    let retrieved = subscriber.fetch_all_next_msgs();
    verify_messages(&msg_inputs, retrieved)?;

    Ok(())
}
