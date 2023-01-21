use frame_support::{decl_module, dispatch::DispatchResult};
use frame_system::{self as system, ensure_signed};

pub trait Trait: system::Trait {}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        #[weight = 0]
        pub fn add_funds(origin, amount: u32) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            // Add the funds to the sender's account balance
            <system::Module<T>>::deposit_creating(&sender, amount.into());
            Ok(())
        }
    }
}
