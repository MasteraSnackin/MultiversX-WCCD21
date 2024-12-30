Explanation
CraftRequest Struct: Stores the requester and the timestamp when the crafting request was made.

craft_sword Endpoint: Allows users to send tokens to craft a Sword. The tokens are consumed and marked for burning. A timestamped request is recorded.

claim_sword Endpoint: Allows users to claim the Sword NFT after one hour has passed since the crafting request.

Helper Functions:

consume_and_burn_tokens: Validates and burns the tokens sent to the contract.
mint_sword: Mints the Sword NFT for the requester.
Documentation
Smart Contract Design: The contract is designed to handle crafting requests by consuming and burning specified tokens. It delays the issuance of the crafted tool by one hour to simulate crafting time.

Verification: The contract is designed to be easily verifiable by checking the code and ABI. The logic for token consumption, burning, and delayed issuance is explicitly coded for transparency.

Open Source: The contract is intended to be open source, allowing others to review, use, and contribute to its development.

Deployment and Usage
Build the Contract: Compile the smart contract to WebAssembly.

cargo build --release --target=wasm32-unknown-unknown
Deploy the Contract: Use MultiversX tools to deploy the compiled Wasm file to the MultiversX Devnet or Mainnet.

Interact with the Contract: Use the MultiversX Devnet Explorer or CLI to call endpoints such as craftSword and claimSword.