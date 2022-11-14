
## Syndote game
Syndote contract(master contract) is the main contract that starts the game. The participants of the game are strategic contracts (player program). 
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
To run the game you have to deploy the master contract and the contracts of players. 
During initialization the master contract is filled with monopoly card information (cell cost, special cells: jails, lottery, etc). 
You have to give enough gas reservation for automatic play. Before each round  the master contract checks the amount of gas and if it is not enough it will send a message to the game admin to request for another gas reservation. To make a reservation you have to send to the game contract the following message: 
```rust
GameAction::ReserveGas
```
Now one reservation is 245 000 000 000 sinces it is not yet possible to make a reservation more than the block gas limit (250 000 000 000). To run the full game make at least 5 reservations.
Then you have to register the contracts of your players (For testing purposes you can upload one player contract four time and also reduce the number of the players in the syndote contract). 
To register the player you have to send the following message to the syndote contract:
```rust
GameAction::Register {
    player: ActorId
}
```
After registering the players just start the game, sending the message:
```rust
GameAction::Play
```
If the game is not over, make more reservations and send a message `GameAction::Play` again. 
After the game is over, it's state become `Finished` and the admin can restart the game by starting a new player registration:
```rust
GameAction::StartRegistration
```