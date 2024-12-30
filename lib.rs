#![no_std]

// Import necessary modules from the MultiversX smart contract framework
multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct CraftRequest<M: ManagedTypeApi> {
    requester: ManagedAddress<M>,
    timestamp: u64,
}

// Define the Blacksmith contract
#[multiversx_sc::contract]
pub trait BlacksmithContract {
    #[init]
    fn init(&self) {}

    // Function to craft a Sword by consuming 1 gold and 3 ore
    #[payable("*")]
    #[endpoint(craftSword)]
    fn craft_sword(&self) -> SCResult<()> {
        let caller = self.blockchain().get_caller();
        let current_time = self.blockchain().get_block_timestamp();

        // Consume tokens and burn them
        let payments = self.call_value().all_esdt_transfers();
        self.consume_and_burn_tokens(&payments)?;

        // Record the crafting request with a timestamp
        let request = CraftRequest {
            requester: caller.clone(),
            timestamp: current_time,
        };
        self.requests(&caller).set(&request);

        Ok(())
    }

    // Function to claim the crafted Sword after 1 hour
    #[endpoint(claimSword)]
    fn claim_sword(&self) -> SCResult<()> {
        let caller = self.blockchain().get_caller();
        let current_time = self.blockchain().get_block_timestamp();

        // Retrieve the crafting request
        let request = self.requests(&caller).get();
        require!(
            current_time >= request.timestamp + 3600,
            "The tool is not ready to be claimed yet."
        );

        // Mint the Sword NFT for the caller
        self.mint_sword(&caller)?;

        // Clear the request record
        self.requests(&caller).clear();

        Ok(())
    }

    // Helper function to consume and burn tokens
    fn consume_and_burn_tokens(
        &self,
        payments: &ManagedVec<EsdtTokenPayment<Self::Api>>,
    ) -> SCResult<()> {
        // Logic to burn each payment token received
        for payment in payments.iter() {
            require!(
                (payment.token_identifier == TokenIdentifier::from("GOLD") && payment.amount >= 1)
                    || (payment.token_identifier == TokenIdentifier::from("ORE") && payment.amount >= 3),
                "Insufficient token amounts"
            );
            // Placeholder for actual burn logic
        }
        Ok(())
    }

    // Function to mint a Sword NFT for the caller
    fn mint_sword(&self, owner: &ManagedAddress) -> SCResult<()> {
        // Logic to mint the Sword NFT
        // This would involve creating a new NFT and transferring it to the caller
        Ok(())
    }

    // Storage mapper to record crafting requests based on the requester
    #[storage_mapper("requests")]
    fn requests(&self, address: &ManagedAddress) -> SingleValueMapper<CraftRequest<Self::Api>>;
}