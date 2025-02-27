use blocks_impl::Transaction;

fn main() {
    // ..... create firs transactions .....//

    let tx1 = Transaction::new(
        "Alice".to_string(),
        "Bob".to_string(),
        10,
        20,
        2,
        "data1".to_string(),
        5,
        blocks_impl::Network::Base,
    );

    // ..... create second transactions .....//

    let tx2 = Transaction::new(
        "S.man".to_string(),
        "Precious".to_string(),
        15,
        9,
        8,
        "data2".to_string(),
        7,
        blocks_impl::Network::Lisk,
    );
    // ..... print transactions .....//
    println!(
        "First transaction.... {:?} \n \n Second transaction.... {:?}",
        tx1, tx2
    );
}
