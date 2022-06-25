use std::str;
use rand::prelude::*;
use openssl::symm::Mode;
use cryptopals_rs::utils::Profile;
use cryptopals_rs::byte_buffer::ByteBuffer;
use cryptopals_rs::cypher;

fn profile_for(email: &str) -> String {
        let mut rng = thread_rng();
        let uid: u32 = rng.gen();

        let profile = Profile {
                email: email.to_owned(),
                uid: uid,
                role: "user".to_owned(),
        };

        return profile.encode();
}

fn make_profile_oracle<'a>() -> (Box<dyn Fn(&'a str) -> Vec<u8>>, Box<dyn Fn(&Vec<u8>) -> Result<Profile, String>>) {
        let rand_key = Vec::<u8>::from_rand_bytes(16);
        let rand_key_clone = rand_key.clone();

        let encrypt = Box::new(move |email| {
                let encoded_profile = profile_for(email).as_bytes().to_vec();
                cypher::aes_ecb(&encoded_profile, &rand_key, Mode::Encrypt)
        });

        let decrypt = Box::new(move |encrypted: &Vec<u8>| {
                let mut decrypted = cypher::aes_ecb(encrypted, &rand_key_clone, Mode::Decrypt);
                if let Some(padded_by) = decrypted.is_padded_for_blocksize(16) {
                        decrypted.truncate(decrypted.len() - padded_by);
                }

                match str::from_utf8(&decrypted) {
                        Err(_) => Err(String::from("Invalid utf8")),
                        Ok(decrypted_str) => {
                                Profile::decode(decrypted_str)
                        }
                }
        });

        (encrypt, decrypt)
}

fn main() {
        let (encrypt_prof, decrypt_prof) = make_profile_oracle();

        let cutting_block = encrypt_prof("FOO@BAR.AA");
        let block_one = cutting_block[..16].to_vec(); // email=FOO@BAR.AA
        let cutting_block = encrypt_prof("AAAAAAAAAAAAAAAAAAAA");
        let block_two = cutting_block[16..32].to_vec(); // AAAAAAAAAA&role=
        let cutting_block = encrypt_prof("AAAAAAAAAAadmin");
        let rest = cutting_block[16..].to_vec(); // admin&role=user&uid=xxx + padding

        let pasted = [block_one, block_two, rest].concat();

        let cracked_prof = decrypt_prof(&pasted).unwrap();
        assert_eq!(cracked_prof.role, String::from("admin"));
}
