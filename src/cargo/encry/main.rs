
use rand_chacha::rand_core::RngCore;
use rand_chacha::rand_core::SeedableRng;

use magic_crypt::{new_magic_crypt, MagicCryptTrait};

use rand_chacha; // 0.3.0

fn main() {

  /*
    check if given path has been generated.

    if !, generate seed and save seed

    load seed into ln NUM
  */

    //let mut gen = rand_chacha::ChaCha8Rng::seed_from_u64(NUM);
    let mut gen = rand_chacha::ChaCha8Rng::seed_from_u64(15480);

    let r = gen.next_u64();

    println!("{:?}", r);

    /* load in encrypt/protected things 
    */
  
    let mc = new_magic_crypt!(r.to_string(), 256);
    let base64 = mc.encrypt_str_to_base64("I have a secret"); //note base64 == string

    println!("{:?}", base64);
    println!("{:?}", mc.decrypt_base64_to_string(&base64).unwrap());
}
