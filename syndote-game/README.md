# Syndote game

Syndote is a Monopoly-like decentralized game that works completely on-chain. Players compete with each other by implementing various playing strategies uploaded as smart-contracts into the network.

Syndote consists of Master contract and Player contracts. Master contract is the main contract that starts and controls the game. Player contracts implement the game strategies that can be unique for each participant of the game. All moves in the game take place automatically, but it is possible to jump to each one individually to analyze the player's strategy.

To launch the game, you need to:
1. ⚒️ Build Master and Player contracts
2. 🏗️ Upload the contracts on chain
3. 🖥️ Build and run user interface

## ⚒️ Build Master and Player contracts

1. Get the source code of [Master contract](https://github.com/gear-tech/syndote-game/tree/master/program/syndote) and [Player contract](https://github.com/gear-tech/syndote-game/tree/master/program/player).
2. Modify Player's contract as you wish to achieve optimal game strategy. 
3. Build contracts as described in [program/README.md](https://github.com/gear-tech/syndote-game/blob/master/program/README.md#building-contracts).

## 🏗️ Upload contracts on chain

###  Run gear node locally

This is recommended while you are testing and debugging the program.

Here (https://get.gear.rs/) you can find prepared binaries.

```bash
./gear --dev --tmp --unsafe-ws-external --rpc-methods Unsafe --rpc-cors all
```

Upload and run Master and Player contracts and register the player as described in [program/README.md](https://github.com/gear-tech/syndote-game/blob/master/program/README.md#running-the-game).

### Run program in Gear Network

You can deploy contracts using [idea.gear-tech.io](https://idea.gear-tech.io). 

1. In the network selector choose wss://node-workshop.gear.rs network.
2. Upload and run Master and Player contracts and register the player as described in [program/README.md](https://github.com/gear-tech/syndote-game/blob/master/program/README.md#running-the-game).

## 🖥️ Build and run user interface

Download this repository locally and run game application using instruction from [frontend/README.md](https://github.com/gear-tech/syndote-game/tree/master/frontend#readme).

Make necessary steps described at [program/README.md](https://github.com/gear-tech/syndote-game/blob/master/program/README.md#running-the-game).
