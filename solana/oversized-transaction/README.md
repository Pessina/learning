# Questions to solve before move AA from Near to Solana

- How to handle oversized transaction?
    - Option 1: Use https://mina86.com/2025/solana-tx-size-limits/, the main con it's that it needs a Anchor fork. But it would allow chunking on the instruction level (Program ID, Accounts, Data)
    - Option 2: Implemented chunking on input level, it's les flexible than Option 1 but because we are dealing exclusively with Multi-Chain transaction we won't need to access many Solana's accounts besides the User PDAs to check permissions and authorized identities.
- Do we have enough CU to: validate signature -> Inspect transaction -> Execute transaction?
- Do the Rust libraries for Signature Verification works smooth in Solana as Near? 