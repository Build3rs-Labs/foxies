# Project overview :

The project is a decentralized and idle-stacking game, utilizing NFTs and smart contracts to create a rewarding gaming experience for its players. It is based on the original “Wolf Game”
Wolf Game website : https://wolf.game/
We will issue different generations of the game, starting with gen1. This technical
document covers gen1 and gen2 features as this is our priority.

# Game concept :

In order to play the game, the player must mint an NFT.
There are 2 types of playable characters :

- Chickens (90% chance to mint)
- Foxes (10% chance to mint)
  On gen1, the supply is 15,000 units :
- 13,500 chickens
- 1,500 foxes
  The in-game currency is the $EGGS token.
  The game dynamics are based on the interactions between the foxes and the chickens.
  Chickens NFTs can be staked to generate $EGGS each day, but when they try to sell their $EGGS, there is a 50% chance that the accumulated $EGGS gets stolen by a Wolf.
  Additionally, a 20% tax is charged on sold $EGGS and distributed to the foxes NFTs holders.
  Foxes automatically earn 20% of all the $EGGS sold by the chickens and also have a 10% chance of stealing new mints. The rarity of the fox increases the holder's chance of stealing a new mint.

# Designing $EGGS smart contract :

Here are some of the core functions we will be using :

- Mapping to store the balance of EGGS for each address
- Mapping to store the allowance of EGGS for each address pair
- Store the total supply of EGGS
- Variables to store game state
- event LaidEggs
- event StolenEggs
- isFox and chickenBalance mappings: These store the game state.
- isFox tracks which addresses are foxes, and chickenBalance tracks the
  number of chickens owned by each address.
- LaidEggs and StolenEggs events: These are emitted when chickens lay eggs
  or foxes steal eggs. They allow game events to be tracked on the blockchain.
- layEggs function: This is called to mint new $EGGS tokens when a chicken
  lays eggs. It checks that the caller owns at least one chicken, mints the
  specified amount of $EGGS, and emits a LaidEggs event.
- stealEggs function: This is called when a fox tries to steal eggs. It checks that the caller is a fox and that the target chicken owner has some $EGGS. It then
  transfers half the chicken owner's $EGGS to the fox and emits a StolenEggs event.
  Designing NFT collection smart contract :
  Minting process : We can use the `mint` function from the ink! standard. We will use a random number generator to determine whether a Chicken or a Fox is minted. The minting price will also be defined.
  Staking mechanism : We define how the players can stake their NFTs to earn $EGGS. We will create functions that allow users to stake and unstake their NFTs.
  Earning mechanism : We define the earning mechanism for $EGGS. This will depend on whether the user has staked a chicken or a fox.
  Stealing mechanism : We define the stealing mechanism. When a chicken tries to claim their $EGGS, there should be a chance that a fox can steal some or all of it.

This contract provides a basic structure for a game similar to the Wolf Game, with the following main components:
● FoxMinted: An event that is emitted whenever a new fox NFT is minted. We will have similar events for chickens and for when eggs are earned or stolen.
● mint: A function for minting new NFTs. In this function, we will implement the logic for minting new foxes and chickens, including the probability distributions.
● stake: A function for staking an NFT to earn eggs. This function will implement the logic for how foxes and chickens earn eggs, as well as the rules for when eggs can be stolen.
This is a very simplified example and doesn't include all the details we need for the full game. We will also need to implement the rules for selling and buying NFTs on the secondary market, the specifics of how eggs are earned and stolen, and more.
Before diving into the development, we might want to consider running a Substrate node locally for testing and debugging purposes.
