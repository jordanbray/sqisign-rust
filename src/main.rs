use sqisign_rust::*;
use std::thread;

// This is test code, which verifies that we can do signature
// stuff in a thread with 2MiB of stack space.
fn main() -> Result<()> {
    thread::spawn(|| {
        let seed = [47; 48];
        let key_pair = KeyPair::<Lvl5>::new(seed).unwrap();
        println!("sk: {:?}", key_pair.private_key().bytes());

        println!("pk: {:?}", key_pair.public_key().bytes());
        let kp2 = KeyPair::<Lvl5>::new(seed).unwrap();
        assert_eq!(kp2, key_pair);

        let msg_good = b"Hello";
        let msg_bad = b"Goodbye";

        let sig = key_pair
            .private_key()
            .sign(msg_good.as_ref())
            .expect("Could not sign message.");
        assert!(sig.verify(msg_good.as_ref(), key_pair.public_key()));
        assert!(!sig.verify(msg_bad.as_slice(), key_pair.public_key()));
    })
    .join()
    .unwrap();
    Ok(())
}
