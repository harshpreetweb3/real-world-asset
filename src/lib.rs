use scrypto::prelude::*;


#[derive(ScryptoSbor, NonFungibleData)]
pub struct AssetData {
    asset_type: String,
    unique_identifier: i32,
    #[mutable]
    current_value: Decimal
}

#[blueprint]
mod asset_nft_system {
    struct AssetCollection {
        nfts: ResourceManager,
        next_uid: i32, // Counter for generating unique identifiers
    }

    impl AssetCollection {
        pub fn instantiate_asset_collection() -> Global<AssetCollection> {
            let nft_resource_address = ResourceBuilder::new_integer_non_fungible::<AssetData>(OwnerRole::None)
                .metadata(metadata! {
                    init {
                        "name" => "Tokenized Asset NFT", locked;
                    }
                })
                .mint_roles(mint_roles! {
                    minter => rule!(allow_all);
                    minter_updater => rule!(deny_all);
                })
                .non_fungible_data_update_roles(non_fungible_data_update_roles! {
                    non_fungible_data_updater => rule!(allow_all);
                    non_fungible_data_updater_updater => rule!(deny_all);
                })
                .create_with_no_initial_supply();

            Self {
                nfts: nft_resource_address,
                next_uid: 1, // Start UID counter from 1
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::None)
            .globalize()
        }

        pub fn mint_asset(&mut self, asset_type: String, initial_value: Decimal) -> Bucket {
            let unique_identifier = self.next_uid;
            self.next_uid += 1; // Increment counter for next UID

            let asset_data = AssetData {
                asset_type,
                unique_identifier,
                current_value: initial_value,
            };

            let resource_manager = self.nfts;
            resource_manager.mint_non_fungible(&NonFungibleLocalId::integer(unique_identifier as u64), asset_data)
        }

        pub fn update_asset_value(&self, nft: NonFungibleBucket, new_value: Decimal) -> NonFungibleBucket {
            let id: NonFungibleLocalId = nft.as_non_fungible().non_fungible_local_id();
            let resource_manager = self.nfts;
            resource_manager.update_non_fungible_data(&id, "current_value", new_value);
            nft
        }

        pub fn get_asset_data(&self, nft_id: NonFungibleLocalId) -> AssetData {
            let resource_manager = self.nfts;
            resource_manager.get_non_fungible_data(&nft_id)
        }

        pub fn transfer_token(&self, mut to_account: Global<Account>, resource: Bucket) {
            to_account.try_deposit_or_abort(resource, None);
        }

        // pub fn transfer_asset(&self, nft_bucket: Bucket, to: ComponentAddress) {
        //     assert!(nft_bucket.resource_address() == self.nfts, "Invalid NFT resource address");
        //     assert!(nft_bucket.amount() == Decimal::one(), "Only one NFT can be transferred at a time");
        //     borrow_component!(to).call::<()>("deposit", scrypto_args!(nft_bucket));
        // }

        // Not in use and can be skipped in current use case.
        // fn get_next_local_id(&self) -> u64 {
        //     let resource_manager = self.nfts;
        //     match resource_manager.total_supply() {
        //         Some(supply) => {
        //             // Convert Decimal to u64
        //             supply.0.try_into().unwrap_or(0) // Provide a default value of 0 if conversion fails
        //         },
        //         None => 0, // Default value if total supply is None
        //     }
        // }
    }
}


// resim call-function package_sim1pk3cmat8st4ja2ms8mjqy2e9ptk8y6cx40v4qnfrkgnxcp2krkpr92 AssetCollection instantiate_asset_collection
// resim call-method component_sim1cz8et5yv5srl909chc9x4dgav32f50rmw4mujuq2trws9xc4y73u6l mint_asset parna 160
// resim call-method component_sim1cz8et5yv5srl909chc9x4dgav32f50rmw4mujuq2trws9xc4y73u6l update_asset_value #1# 200.0

// resim call-method component_sim1cz8et5yv5srl909chc9x4dgav32f50rmw4mujuq2trws9xc4y73u6l transfer_token account_sim1cyf0w04zyj05k7skd4tscpk2vedtv8vnyejfkgc3fm798lyxjjrzvg resource_sim1nfh3kupr5l95w6ufpuysl0afun0gfzzw7ltmk7y68ks5ekqhlkjuyc:1