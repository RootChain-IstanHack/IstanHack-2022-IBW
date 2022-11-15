#![no_std]
use gstd::{exec, msg, prelude::*, ActorId};
use syndote_io::*;

pub const COST_FOR_UPGRADE: u32 = 500;
pub const FINE: u32 = 1_000;
pub const MAX_GEAR_PRICE: u32 = 2500;

#[gstd::async_main]
async fn main() {
    let monopoly_id = msg::source();
    let mut message: YourTurn = msg::load().expect("Unable to decode struct`YourTurn`");
    let my_player = message
        .players
        .get_mut(&exec::program_id())
        .expect("Players: Cant be `None`");

    if my_player.in_jail {
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
    }

    let position = my_player.position;

    let (my_cell, free_cell, gears, price) =
        if let Some((account, gears, price, _)) = &message.properties[position as usize] {
            let my_cell = account == &exec::program_id();
            let free_cell = account == &ActorId::zero();
            (my_cell, free_cell, gears, price)
        } else {
            msg::reply("", 0).expect("Error in sending a reply to monopoly contract");
            return;
        };

    if my_cell && gears.len() < 3 {
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
    }
    if free_cell && price <= &MAX_GEAR_PRICE {
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

#[no_mangle]
unsafe extern "C" fn init() {
    //   MONOPOLY = msg::load::<ActorId>().expect("Unable to decode ActorId");
}

gstd::metadata! {
title: "Player",
 //   init:
   //     input: ActorId,
}
