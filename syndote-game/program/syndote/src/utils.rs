use crate::*;

impl Game {
    pub fn check_status(&self, game_status: GameStatus) {
        assert_eq!(self.game_status, game_status, "Wrong game status");
    }

    pub fn only_admin(&self) {
        assert_eq!(msg::source(), self.admin, "Only admin can start the game");
    }
    pub fn only_player(&self) {
        assert!(
            self.players.contains_key(&msg::source()),
            "You are not in the game"
        );
    }
}

pub fn get_player_info<'a>(
    player: &'a ActorId,
    players: &'a mut BTreeMap<ActorId, PlayerInfo>,
    current_round: u128,
) -> Result<&'a mut PlayerInfo, GameError> {
    if &msg::source() != player {
        //        debug!("PENALTY: WRONG MSG::SOURCE()");
        players.entry(msg::source()).and_modify(|player_info| {
            player_info.penalty += 1;
        });
        return Err(GameError::StrategicError);
    }
    let player_info = players.get_mut(player).expect("Cant be None: Get Player");
    if player_info.round >= current_round {
        //   debug!("PENALTY: MOVE ALREADY MADE");
        player_info.penalty += 1;
        return Err(GameError::StrategicError);
    }
    Ok(player_info)
}

pub fn sell_property(
    admin: &ActorId,
    ownership: &mut [ActorId],
    properties_for_sale: &Vec<u8>,
    properties_in_bank: &mut BTreeSet<u8>,
    properties: &[Option<(ActorId, Gears, u32, u32, CellType)>],
    player_info: &mut PlayerInfo,
) -> Result<(), GameError> {
    for property in properties_for_sale {
        if ownership[*property as usize] != msg::source() {
            //       debug!("PENALTY: TRY TO SELL NOT OWN PROPERTY");
            player_info.penalty += 1;
            return Err(GameError::StrategicError);
        }
    }

    for property in properties_for_sale {
        if let Some((_, _, price, _, _)) = properties[*property as usize] {
            player_info.cells.remove(property);
            player_info.balance += price / 2;
            ownership[*property as usize] = *admin;
            properties_in_bank.insert(*property);
        }
    }
    Ok(())
}
static mut SEED: u64 = 0;
pub fn get_rolls() -> (u8, u8) {
    let seed = unsafe {
        SEED = SEED.wrapping_add(1);
        SEED
    };
    let random = exec::random(&(exec::block_timestamp() + seed).to_be_bytes()).expect("");
    let r1: u8 = random.0[0] % 6 + 1;
    let r2: u8 = random.0[1] % 6 + 1;
    (r1, r2)
}

pub fn bankrupt_and_penalty(
    admin: &ActorId,
    players: &mut BTreeMap<ActorId, PlayerInfo>,
    players_queue: &mut Vec<ActorId>,
    properties: &mut [Option<(ActorId, Gears, Price, Rent, CellType)>],
    properties_in_bank: &mut BTreeSet<u8>,
    ownership: &mut [ActorId],
) {
    for (player, mut player_info) in players.clone() {
        if player_info.debt > 0 {
            for cell in &player_info.cells.clone() {
                if player_info.balance >= player_info.debt {
                    debug!("| Player {:?} | Penalty: Debt decreased from balance automatically" , player.as_ref()[0]);
                    player_info.balance -= player_info.debt;
                    player_info.debt = 0;
                    player_info.penalty += 1;
                    players.insert(player, player_info);
                    break;
                }
                if let Some((_, _, price, _, _)) = &properties[*cell as usize] {
                    debug!("| Player {:?} | Debt: Sold property to admin half-price" , player.as_ref()[0]);
                    player_info.balance += price / 2;
                    player_info.cells.remove(cell);
                    ownership[*cell as usize] = *admin;
                    properties_in_bank.insert(*cell);
                }
            }
        }
    }

    for (player, mut player_info) in players.clone() {
        if (player_info.penalty >= PENALTY || player_info.debt > 0) && !player_info.lost { // edited fixed: Kicked players still iterate over the list added "&& !player_info.lost"
            debug!("| Player {:?} | Kicked out of game" , player.as_ref()[0]);
            player_info.lost = true;
            players_queue.retain(|&p| p != player);
            for cell in &player_info.cells.clone() {
                ownership[*cell as usize] = *admin;
                properties_in_bank.insert(*cell);
            }
            players.insert(player, player_info);
        }
    }
}

pub fn init_properties(
    properties: &mut Vec<Option<(ActorId, Gears, Price, Rent, CellType)>>,
    ownership: &mut Vec<ActorId>,
) {
    //60 -> Genesis cell
    //61 -> Jail cell
    //62 -> GoToJail cell
    //63 -> Punishment cell
    //64 -> Jackpot cell
    //65 -> Teleport cell
    //66 -> Mystery cell
    
    //edited
    // 0
    properties.push(Some((ActorId::from(60), Vec::new(), NEW_CIRCLE, 0, CellType::Genesis))); //genesis

    // 1
    properties.push(Some((ActorId::zero(), Vec::new(), 1_000, 100, CellType::Normal)));

    // 2
    properties.push(Some((ActorId::from(63), Vec::new(), PUNISHMENT_FEE, 0, CellType::Punishment))); //punishment

    // 3
    properties.push(Some((ActorId::zero(), Vec::new(), 1_050, 105, CellType::Normal)));

    // 4
    properties.push(Some((ActorId::from(66), Vec::new(), MYSTERY_VALUE, 0, CellType::Mystery))); //mystery

    // 5
    properties.push(Some((ActorId::zero(), Vec::new(), 1_100, 110, CellType::Normal)));
    // 6
    properties.push(Some((ActorId::zero(), Vec::new(), 1_500, 150, CellType::Normal)));

    // 7
    properties.push(Some((ActorId::from(65), Vec::new(), TELEPORT_FEE, 0, CellType::Teleport))); //teleport

    // 8
    properties.push(Some((ActorId::zero(), Vec::new(), 1_550, 155, CellType::Normal)));
    // 9
    properties.push(Some((ActorId::zero(), Vec::new(), 1_700, 170, CellType::Normal)));

    // 10
    properties.push(Some((ActorId::from(61), Vec::new(), 0, 0, CellType::Jail))); //jail
    
    // 11
    properties.push(Some((ActorId::zero(), Vec::new(), 2_000, 200, CellType::Normal)));
    // 12
    properties.push(Some((ActorId::zero(), Vec::new(), 2_050, 205, CellType::Normal)));
    // 13
    properties.push(Some((ActorId::zero(), Vec::new(), 2_100, 210, CellType::Normal)));
    // 14
    properties.push(Some((ActorId::zero(), Vec::new(), 2_200, 220, CellType::Normal)));
    // 15
    properties.push(Some((ActorId::zero(), Vec::new(), 2_300, 230, CellType::Normal)));

    // 16
    properties.push(Some((ActorId::from(63), Vec::new(), PUNISHMENT_FEE, 0, CellType::Punishment))); //punishment

    // 17
    properties.push(Some((ActorId::zero(), Vec::new(), 2_400, 240, CellType::Normal)));
    // 18
    properties.push(Some((ActorId::zero(), Vec::new(), 2_450, 245, CellType::Normal)));
    // 19
    properties.push(Some((ActorId::zero(), Vec::new(), 2_500, 250, CellType::Normal)));

    // 20
    properties.push(Some((ActorId::from(64), Vec::new(), JACKPOT_EARN, 0, CellType::Jackpot))); //jackpot

    // 21
    properties.push(Some((ActorId::zero(), Vec::new(), 3_000, 300, CellType::Normal)));
    // 22
    properties.push(Some((ActorId::zero(), Vec::new(), 3_000, 300, CellType::Normal)));
    // 23
    properties.push(Some((ActorId::zero(), Vec::new(), 3_100, 310, CellType::Normal)));
    // 24
    properties.push(Some((ActorId::zero(), Vec::new(), 3_150, 315, CellType::Normal)));
    // 25
    properties.push(Some((ActorId::zero(), Vec::new(), 3_200, 320, CellType::Normal)));
    // 26
    properties.push(Some((ActorId::zero(), Vec::new(), 3_250, 325, CellType::Normal)));
    // 27
    properties.push(Some((ActorId::zero(), Vec::new(), 3_300, 330, CellType::Normal)));
    // 28
    properties.push(Some((ActorId::zero(), Vec::new(), 3_350, 334, CellType::Normal)));
    // 29
    properties.push(Some((ActorId::zero(), Vec::new(), 3_400, 340, CellType::Normal)));

    // 30
    properties.push(Some((ActorId::from(62), Vec::new(), 0, 0, CellType::GotoJail))); //Go to jail

    // 31
    properties.push(Some((ActorId::zero(), Vec::new(), 4_000, 400, CellType::Normal)));
    // 32
    properties.push(Some((ActorId::zero(), Vec::new(), 4_050, 405, CellType::Normal)));

    // 33
    properties.push(Some((ActorId::from(65), Vec::new(), TELEPORT_FEE, 0, CellType::Teleport))); //teleport

    // 34
    properties.push(Some((ActorId::zero(), Vec::new(), 4_100, 410, CellType::Normal)));
    // 35
    properties.push(Some((ActorId::zero(), Vec::new(), 4_150, 415, CellType::Normal)));

    // 36
    properties.push(Some((ActorId::from(66), Vec::new(), MYSTERY_VALUE, 0, CellType::Mystery))); //mystery

    // 37
    properties.push(Some((ActorId::zero(), Vec::new(), 4_200, 420, CellType::Normal)));

    // 38
    properties.push(Some((ActorId::from(63), Vec::new(), PUNISHMENT_FEE, 0, CellType::Punishment))); //Punishment

    // 39
    properties.push(Some((ActorId::zero(), Vec::new(), 4_500, 450, CellType::Normal)));

    for _i in 0..40 {
        ownership.push(ActorId::zero());
    }
}

pub enum GameError {
    StrategicError,
}
