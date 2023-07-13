use openbrush::{
    contracts::psp34::PSP34Error,
    traits::{AccountId, Balance},
};

#[openbrush::wrapper]
pub type EggsRef = dyn Eggs;

#[openbrush::trait_definition]
pub trait Eggs {
    
}
