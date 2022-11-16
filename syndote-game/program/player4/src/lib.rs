#![no_std]
use gstd::{exec, msg, prelude::*, ActorId, debug};
use syndote_io::*;
//static mut MONOPOLY: ActorId = ActorId::zero();
pub const COST_FOR_UPGRADE: u32 = 500;
pub const FINE: u32 = 1_000;

#[gstd::async_main]
async fn main() {
    //let monopoly_id = unsafe { MONOPOLY };
    let monopoly_id = msg::source();
    // assert_eq!(
    //     msg::source(),
    //     monopoly_id,
    //     "Only monopoly contract can call strategic contract"
    // );
    let mut message: YourTurn = msg::load().expect("Unable to decode struct`YourTurn`");
    let my_player = message
        .players
        .get_mut(&exec::program_id())
        .expect("Players: Cant be `None`");
    if my_player.in_jail {
        if my_player.balance <= FINE {
            let reply: GameEvent = msg::send_for_reply_as(
                monopoly_id,
                GameAction::ThrowRoll {
                    pay_fine: false,
                    properties_for_sale: None,
                },
                0,
            )
            .expect("Error in sending a message `GameAction::ThrowRoll`")
            .await
            .expect("Unable to decode `GameEvent");

            if let GameEvent::Jail { in_jail, position } = reply {
                if !in_jail {
                    my_player.position = position;
                } else {
                    msg::reply("", 0).expect("Error in sending a reply to monopoly contract");
                    return;
                }
            }
        } else {
            msg::send_for_reply_as::<_, GameEvent>(
                monopoly_id,
                GameAction::ThrowRoll {
                    pay_fine: true,
                    properties_for_sale: None,
                },
                0,
            )
            .expect("Error in sending a message `GameAction::ThrowRoll`")
            .await
            .expect("Unable to decode `GameEvent");

            msg::reply("", 0).expect("Error in sending a reply to monopoly contract");
            return;
        }
    }

    let position = my_player.position;

    let (account, my_cell, free_cell, gears, special_cell) =
        if let Some((account, gears, _, rent)) = &message.properties[position as usize] {
            let my_cell = account == &exec::program_id();
            let free_cell = account == &ActorId::zero();
            if rent == &0 { (account, my_cell, free_cell, gears, true)}
            else { (account, my_cell, free_cell, gears, false)}
        } else {
            msg::reply("", 0).expect("Error in sending a reply to monopoly contract");
            return;
        };

        if special_cell {
            if account == &ActorId::from(65) { //teleport cell
                msg::send_for_reply_as::<_, GameEvent>(
                    monopoly_id,
                    GameAction::Teleport,
                    0,
                )
                .expect("Error in sending a message `GameAction::AddGear`")
                .await
                .expect("Unable to decode `GameEvent");
            }
            else if account == &ActorId::from(66) { //Mystery cell
                msg::send_for_reply_as::<_, GameEvent>(
                    monopoly_id,
                    GameAction::Mystery,
                    0,
                )
                .expect("Error in sending a message `GameAction::AddGear`")
                .await
                .expect("Unable to decode `GameEvent");
    
            }
            msg::reply("", 0).expect("Error in sending a reply to monopoly contract");
            return;
        }

    if my_cell {
        //debug!("ADD GEAR");
        if gears.len() < 3 && calculate_chance(70, my_player.balance.into()){
            msg::send_for_reply_as::<_, GameEvent>(
                monopoly_id,
                GameAction::AddGear {
                    properties_for_sale: None,
                },
                0,
            )
            .expect("Error in sending a message `GameAction::AddGear`")
            .await
            .expect("Unable to decode `GameEvent");
            msg::reply("", 0).expect("Error in sending a reply to monopoly contract");
            return;
        } else {
            //debug!("UPGRADE");
            msg::send_for_reply_as::<_, GameEvent>(
                monopoly_id,
                GameAction::Upgrade {
                    properties_for_sale: None,
                },
                0,
            )
            .expect("Error in sending a message `GameAction::Upgrade`")
            .await
            .expect("Unable to decode `GameEvent");
            msg::reply("", 0).expect("Error in sending a reply to monopoly contract");
            return;
        }
    }
    if free_cell && calculate_chance(60, my_player.balance.into()) {
        //debug!("BUY CELL");
        msg::send_for_reply_as::<_, GameEvent>(
            monopoly_id,
            GameAction::BuyCell {
                properties_for_sale: None,
            },
            0,
        )
        .expect("Error in sending a message `GameAction::BuyCell`")
        .await
        .expect("Unable to decode `GameEvent");
    } else if !my_cell {
        //debug!("PAY RENT");
        msg::send_for_reply_as::<_, GameEvent>(
            monopoly_id,
            GameAction::PayRent {
                properties_for_sale: None,
            },
            0,
        )
        .expect("Error in sending a message `GameAction::PayRent`")
        .await
        .expect("Unable to decode `GameEvent");
    }
    msg::reply("", 0).expect("Error in sending a reply to monopoly contract");
}

pub fn calculate_chance(percentance: u8, seed: u64) -> (bool) {
    let random = exec::random(&(exec::block_timestamp() + seed).to_be_bytes()).expect("");
    let chance: u8 = random.0[0] %  100 + 1;
    if chance < percentance { return true; }
    else { return false; }
}

#[no_mangle]
unsafe extern "C" fn init() {
    //   MONOPOLY = msg::load::<ActorId>().expect("Unable to decode ActorId");
}

gstd::metadata! {
title: "Player",
 //   init:
   //     input: ActorId,
}