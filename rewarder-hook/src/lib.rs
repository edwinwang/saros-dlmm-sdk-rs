#[cfg(not(feature = "no-entrypoint"))]
anchor_gen::generate_cpi_crate!("idls/rewarder_hook.json");

#[cfg(feature = "no-entrypoint")]
pub mod prelude {
    use solana_sdk::pubkey;
    use solana_sdk::pubkey::Pubkey;

    pub const ID: Pubkey = pubkey!("mdmavMvJpF4ZcLJNg6VSjuKVMiBo5uKwERTg1ZB9yUH");
}

#[cfg(feature = "no-entrypoint")]
pub use prelude::*;
