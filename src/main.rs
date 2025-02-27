use blocks_impl::Transaction;

fn main() {
    let tx1 = Transaction::new(
        "Alice".to_string(),
            "Bob".to_string(),
        10,
        20,
        2,
        "".to_string(),
        5,
        blocks_impl::Network::Base
    );

    println!("{:?}", tx1); // Print the transaction
}
