## Project overview :

The project is a decentralized and idle-stacking game, utilizing NFTs and smart contracts to create a rewarding gaming experience for its players. It is based on the original “Wolf Game”
Wolf Game website : https://wolf.game/
We will issue different generations of the game, starting with gen1. This technical
document covers gen1 and gen2 features as this is our priority.

## Game concept :

In order to play the game, the player must mint an NFT.
There are 2 types of playable characters :

- Chickens (90% chance to mint)
- Foxes (10% chance to mint)

## On gen1, the supply is 15,000 units :

- 13,500 chickens
- 1,500 foxes
- The in-game currency is the $EGGS token.
- The game dynamics are based on the interactions between the foxes and the chickens.
- Chickens NFTs can be staked to generate $EGGS each day, but when they try to sell their $EGGS, there is a 50% chance that the accumulated $EGGS gets stolen by a Wolf.
- Additionally, a 20% tax is charged on sold $EGGS and distributed to the foxes NFTs holders.
  Foxes automatically earn 20% of all the $EGGS sold by the chickens and also have a 10% chance of stealing new mints. The rarity of the fox increases the holder's chance of stealing a new mint.

## Contracts

The main implementation of Foxes smart contract are in /contracts folder and the traits and implementation of traits locate in impls and traits folders and contains following contracts:

- Eggs_Contract
- Nft_Contract
- Staking_Contract

## Code standar

ink! is an EDSL based on Rust, therefore, we use clippy and rustfmt to make sure code is in compliance with Rust idioms.

```
rustup component add rustfmt --toolchain nightly
cargo +nightly fmt
cargo clippy
```

## Contract Build and Deploy Instructions

Before building smart contract, you will first need to install some development tools. The comprehensive guide can be found at: https://docs.alephzero.org/aleph-zero/build/installing-required-tools

Go to the contract folder you want to build under **contracts** and run

```
cargo contract build --release
```

After the contract is built successfully, you will see under contracts/<contract_name>/target/ink 3 files:
. contract_name.wasam
. contract_name.contract
. contract_name.json

Follow this instruction to deploy the contract:
https://docs.alephzero.org/aleph-zero/build/deploying-your-contract-to-aleph-zero-testnet

## Contract Deployment Steps

1. Deploy Eggs Contract
2. Deploy NFT Contract
3. Deploy Staking Contract make sure select **eggs_contract** and **nft_contract** while deploying **staking_contract**
