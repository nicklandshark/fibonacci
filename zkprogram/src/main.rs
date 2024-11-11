use risc0_zkvm::{
    guest::{env, sha::Impl},
    sha::Sha256
};

fn main() {
    // Read the input and commit the hash of the input to the journal
    // Bonsol requires us to commit the hash of the input to perform verification
    let n: u32 = env::read();
    let digest = Impl::hash_bytes(&n.to_le_bytes()[..]);
    env::commit_slice(digest.as_bytes());

    // Taken from the SP1 example
    // Compute the n'th fibonacci number, using normal Rust code.
    let mut a = 0u32;
    let mut b = 1u32;
    for _ in 0..n {
        let mut c = a + b;
        c %= 7919; // Modulus to prevent overflow.
        a = b;
        b = c;
    }

    println!("For input n: {} the nth fibonacci number is: {}", n, b);
    env::commit(&b);
}
