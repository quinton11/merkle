# **Merkle**

A demo of the Binary Merkle Tree data structure used in block chains to verify the integrity and consistency of blocks/ transactions. Keccak256 hashing was used for generating unique hashes for each word.

**Get Started**

- Clone into the repo and `cd` into the folder
- then run `cargo run` (![Must have rustc installed](https://www.rust-lang.org/tools/install))

#### **Preview**

- 8-word Binary merkle tree with sparse proofs (showing the minimum hashes needed to verify the integrity of the selected word)

- 8 and 4-word BMT showing a slight change in word generates a hash which fails the proof
