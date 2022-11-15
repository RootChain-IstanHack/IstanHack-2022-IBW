
## Syndote game smart contracts

Syndote consists of Master contract Player contracts. Master contract is the main contract that starts and controls the game. Player contracts implement the game strategy of each participant of the game. All moves in the game take place automatically, but it is possible to jump to each one individually to analyze the player's strategy.

## Building contracts

### ⚙️ Install Rust

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### ⚒️ Add specific toolchains

```shell
rustup toolchain add nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
```

... or ...

```shell
make init
```

### 🏗️ Build

```shell
cargo build --release
```

... or ...

```shell
make build
```

If everything goes well, your working directory should now have a `target` directory that looks like this:

```
target
    ├── CACHEDIR.TAG
    ├── release
    │   └── ...
    └── wasm32-unknown-unknown
        └── release
            ├── ...
            ├── syndote.wasm      <---- this is built .wasm file
            ├── syndote.opt.wasm  <---- this is optimized .wasm file
            ├── syndote.meta.wasm <---- this is meta .wasm file
            ├── player.wasm       <---- this is built .wasm file
            ├── player.opt.wasm   <---- this is optimized .wasm file
            └── player.meta.wasm  <---- this is meta .wasm file
```

## Running the game

To run the game you have to deploy the master contract and the players contracts to the network. During initialization the master contract is filled with monopoly card information (cell cost, special cells: jails, lottery, etc).
 
You have to give enough gas reservation for automatic play. Before each round the master contract checks the amount of gas and if it is not enough it will send a message to the game admin to request for another gas reservation. To make a reservation you have to send to the master contract the following message: 

```rust
GameAction::ReserveGas
```
Currently the single gas reservation amount can be up to 245 000 000 000 since it is not yet possible to make a reservation more than the block gas limit (250 000 000 000). To run the full game you have to make at least 5 reservations.

Then you need to register the contracts of your players. For testing purposes you can upload the same player contract several times. Up to four players or less can be added in the Syndote Master contract.

To register the player you have to send the following message to the Syndote contract:

```rust
GameAction::Register {
    player: ActorId
}
```

After registering players, just start the game via sending the message:

```rust
GameAction::Play
```

If the game is not over, make more reservations and send a message `GameAction::Play` again. 
After the game is over, it's state becomes `Finished` and the admin can restart the game by starting a new player registration:

```rust
GameAction::StartRegistration
```
