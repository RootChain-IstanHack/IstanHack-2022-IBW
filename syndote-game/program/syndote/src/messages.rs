use crate::*;
use gstd::errors::ContractError;

pub async fn take_your_turn(player: &ActorId, game: &Game) -> Result<Vec<u8>, ContractError> {
    msg::send_for_reply(
        *player,
        YourTurn {
            players: game.players.clone(),
            properties: { //I got decode error so did a dummy workaround like this
                let mut copy_properties: Vec<Option<(ActorId, Gears, u32, u32)>> = vec![None];
                for property in game.properties.clone() {
                    let (actor_id, gear, price, rent, _) = property.unwrap();
                    copy_properties.push(Some((actor_id, gear, price, rent)));
                }
                copy_properties.clone()
                
            },
        },
        0,
    )
    .expect("Error on sending `YourTurn` message")
    .up_to(Some(WAIT_DURATION))
    .expect("Invalid wait duration.")
    .await
}
