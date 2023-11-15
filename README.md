## Project overview :

The project is a decentralized and idle-stacking game, utilizing NFTs and smart contracts to create a rewarding gaming experience for its players. It is based on the original “Wolf Game”
Wolf Game website : https://wolf.game/
We will issue different generations of the game, starting with gen1. This technical
document covers gen1 and gen2 features as this is our priority.

## Game concept :

In order to play the game, the player must mint an NFT.
There are 2 types of playable characters :

- Chickens (80% chance to mint)
- Foxes (20% chance to mint)

## On gen1, the supply is 15,000 units :

- 13,500 chickens
- 1,500 foxes
- The in-game currency is the $EGGS token.
- The game dynamics are based on the interactions between the foxes and the chickens.
- Chickens NFTs can be staked to generate $EGGS each day, but when they try to sell their $EGGS, there is a 50% chance that the accumulated $EGGS gets stolen by a Wolf.
- Additionally, a 20% tax is charged on sold $EGGS and distributed to the foxes NFTs holders.
  Foxes automatically earn 20% of all the $EGGS sold by the chickens.

## Contracts

The following contracts exist within this project:

- Eggs_Contract
- Chickens Contract
- Foxes Contract
- Staking_Contract
- Factory Contract

## Code standar

ink! is an EDSL based on Rust, therefore, we use clippy and rustfmt to make sure code is in compliance with Rust idioms.

```
rustup component add rustfmt --toolchain nightly
cargo +nightly fmt
cargo clippy
```

## Contract Build and Deploy Instructions

Before building smart contract, you will first need to install some development tools. The comprehensive guide can be found at: https://docs.alephzero.org/aleph-zero/build/installing-required-tools

Go to the contract folder you want to build within the ./contracts folder and run:

```
cargo contract build --release
```

for ./factory and ./staking

Then run:

```
cargo contract build --release --features "contract"
```

for ./eggs, ./chickens, and ./foxes (PSP22 and PSP34s)


After the contract is built successfully, you will see under ./contracts/<contract_name>/target/ink 3 files:

- contract_name.wasm
- contract_name.contract
- contract_name.json

Follow this instruction to deploy the contract:
https://docs.alephzero.org/aleph-zero/build/deploying-your-contract-to-aleph-zero-testnet

## Contract Deployment Steps

1. Deploy the Factory contract first.

2. Deploy Chickens contract, while providing the deployed factory contract address as one of the required call arguments. (13,500 supply)

3. Deploy Foxes contract, while providing the deployed factory contract address as one of the required call arguments. (1,500 supply)

Careful of your input into the u128 fields for max supply for both contracts, knowing it is going to multiply by 10 to the power of 12.

4. Set the address of the chickens and foxes contracts within the Factory using the set_chickens_nft_address() and set_foxes_nft_address() methods respectively.

5. Deploy the Staking contract, providing useful fields.

6. Deploy the Eggs contract, while providing deployed staking contract address as one of the required call arguments. (Name: Eggs, Symbol: EGGS, decimals: 6).

7. Set the address of $EGGS contract within the staking contract using the set_eggs_address() method.

8. Voila! Deployment done! Make sure to provide all other arguments appropriately.