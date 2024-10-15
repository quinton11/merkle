# **Merkle**

A demo of the Binary Merkle Tree data structure used in block chains to verify the integrity and consistency of blocks/ transactions. Keccak256 hashing was used for generating unique hashes for each word.

**Get Started**

- Clone into the repo and `cd` into the folder
- then run `cargo run` ([Must have rustc installed](https://www.rust-lang.org/tools/install))

#### **Preview**

- 8-word Binary merkle tree with sparse proofs (showing the minimum hashes needed to verify the integrity of the selected word)


https://github.com/user-attachments/assets/fc370b2e-7a3b-4d0a-8287-16dd6731fadb



- 8 and 4-word BMT showing a slight change in word generates a hash which fails the proof


https://github.com/user-attachments/assets/20fe2cf6-feee-4065-9955-312ac2c5b276

