use sqisign_rust::*;
use std::thread;
fn main() -> Result<()> {
    thread::spawn(|| {
        let seed = [47; 48];
        let key_pair = KeyPair::<Lvl5>::new(seed).unwrap();
        println!("sk: {:?}", key_pair.private_key().bytes());

        println!("pk: {:?}", key_pair.public_key().bytes());
        let kp2 = KeyPair::<Lvl5>::new(seed).unwrap();
        assert_eq!(kp2, key_pair);
    })
    .join()
    .unwrap();
    Ok(())
}
