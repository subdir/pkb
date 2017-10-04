use std::fmt::Debug;
use std::cmp::{min, max};
use std::vec::Vec;
use std::collections::VecDeque;
use std::result::Result::{Ok, Err};

use chips::Chips;

// Invariant: stake is always greater than zero
#[derive(Debug)]
struct ActivePlayer<Player> {
    player: Player,
    stake: Chips,
}

impl<Player> ActivePlayer<Player> {
    fn new(player: Player, stake: Chips) -> Self {
        assert!(stake > Chips::new(0));
        Self { player: player, stake: stake }
    }
}


#[derive(Debug)]
enum SimpleBetError {
    NotEnoughChips,
    LessThanPreviousBet,
}


#[derive(Debug)]
struct Bettor<Player> {
    player: Player,
    stake: Chips,
    bet: Chips,
}

impl<Player> Bettor<Player> {
    fn new(player: ActivePlayer<Player>) -> Self {
        Self { player: player.player, stake: player.stake, bet: Chips::new(0) }
    }

    fn player(&self) -> &Player { &self.player }
    fn stake(&self) -> Chips { self.stake }
    fn bet(&self) -> Chips { self.bet }
    fn is_all_in(&self) -> bool { self.bet == self.stake }

    fn make_bet(&mut self, new_bet: Chips) -> Result<(), SimpleBetError> {
        if new_bet < self.bet {
            Err(SimpleBetError::LessThanPreviousBet)
        } else if new_bet > self.stake {
            Err(SimpleBetError::NotEnoughChips)
        } else {
            self.bet = new_bet;
            Ok(())
        }
    }
}


enum BetError<Player> {
    NotEnoughChips { bettor: ActiveBettor<Player>, bet: Chips },
    LessThanPreviousBet { bettor: ActiveBettor<Player>, bet: Chips },
    TooSmallBet { bettor: ActiveBettor<Player>, bet: Chips, min_bet: Chips },
}

impl<Player> BetError<Player> {
    fn into_bettor(self) -> ActiveBettor<Player> {
        use self::BetError::*;

        match self {
            NotEnoughChips{ bettor, bet } => bettor,
            LessThanPreviousBet{ bettor, bet } => bettor,
            TooSmallBet { bettor, bet, min_bet } => bettor,
        }
    }
}


enum InvolvedBettor<Player> {
    Active(ActiveBettor<Player>),
    AllIn(AllInBettor<Player>),
}

impl<Player> InvolvedBettor<Player> {
    fn new(bettor: Bettor<Player>) -> Self {
        use self::InvolvedBettor::*;

        if bettor.is_all_in() {
            AllIn(AllInBettor { bettor })
        } else {
            Active(ActiveBettor { bettor: bettor, acted: false })
        }
    }

    fn new_forced(bettor: Bettor<Player>) -> Self {
        use self::InvolvedBettor::*;

        if bettor.is_all_in() {
            AllIn(AllInBettor { bettor })
        } else {
            Active(ActiveBettor { bettor: bettor, acted: true })
        }
    }

    fn bet(&self) -> Chips {
        use self::InvolvedBettor::*;

        match self {
            &Active(ref bettor) => bettor.bet(),
            &AllIn(ref bettor) => bettor.bet(),
        }
    }
}


// Invariant: bet is always less than stake
struct FoldedBettor<Player> {
    bettor: Bettor<Player>
}

impl<Player> FoldedBettor<Player> {
    fn new(bettor: Bettor<Player>) -> Self {
        assert!(bettor.bet < bettor.stake());
        Self { bettor: bettor }
    }
}


// Invariant: bet is always equal to stake
struct AllInBettor<Player> {
    bettor: Bettor<Player>
}

impl<Player> AllInBettor<Player> {
    fn new(bettor: Bettor<Player>) -> Self {
        assert!(bettor.bet == bettor.stake());
        Self { bettor: bettor }
    }

    fn bet(&self) -> Chips { self.bettor.bet() }
}


// Invariant: bet is always zero
#[derive(Debug)]
struct ForcedBettor<Player> {
    bettor: Bettor<Player>
}

impl<Player> ForcedBettor<Player> {
    fn new(player: ActivePlayer<Player>) -> Self {
        Self { bettor: Bettor::new(player) }
    }
}

impl<Player: Eq> ForcedBettor<Player> {
    fn is(self, player: Player) -> Self {
        assert!(
            player == *self.bettor.player(),
            "Player mismatch: {:?} != {:?}"//,
//            player,
//            self.bettor.player()
        );
        self
    }
}

impl<Player: Debug> ForcedBettor<Player> {
    fn make_forced_bet(mut self, new_bet: Chips) -> InvolvedBettor<Player> {
        let stake = self.bettor.stake();
        match self.bettor.make_bet(min(new_bet, stake)) {
            Err(error) => panic!("Bad forced bet {:?} on {:?}, error: {:?}", new_bet, self, error),
            Ok(()) => InvolvedBettor::new_forced(self.bettor),
        }
    }
}


struct ActiveBettor<Player> {
    bettor: Bettor<Player>,
    acted: bool,
}

impl<Player: Eq> ActiveBettor<Player> {
    fn is(self, player: Player) -> Self {
        assert!(player == *self.bettor.player());
        self
    }
}

impl<Player> ActiveBettor<Player> {
    fn bet(&self) -> Chips { self.bettor.bet() }
    fn stake(&self) -> Chips { self.bettor.stake() }
    fn acted(&self) -> bool { self.acted }

    fn fold(self) -> FoldedBettor<Player> {
        FoldedBettor::new(self.bettor)
    }

    fn make_bet(mut self, new_bet: Chips) -> Result<InvolvedBettor<Player>, BetError<Player>> {
        use self::SimpleBetError::*;

        match self.bettor.make_bet(new_bet) {
            Ok(()) => Ok(InvolvedBettor::new(self.bettor)),
            Err(error) => match error {
                NotEnoughChips => Err(BetError::NotEnoughChips { bettor: self, bet: new_bet }),
                LessThanPreviousBet => Err(BetError::LessThanPreviousBet { bettor: self, bet: new_bet }),
            },
        }
    }
}


struct Bettors<Player> {
    betting_queue: VecDeque<ActiveBettor<Player>>,
    all_in: Vec<AllInBettor<Player>>,
    folded: Vec<FoldedBettor<Player>>,
    max_bet: Chips,
}

impl<Player> Bettors<Player> {
    fn new() -> Self {
        Self {
            betting_queue: VecDeque::new(),
            all_in: Vec::new(),
            folded: Vec::new(),
            max_bet: Chips::new(0),
        }
    }

    fn max_bet(&self) -> Chips { self.max_bet }
    fn total_count(&self) -> usize { self.betting_queue.len() + self.all_in.len() + self.folded.len() }
    fn involved_count(&self) -> usize { self.betting_queue.len() + self.all_in.len() }
    fn active_count(&self) -> usize { self.betting_queue.len() }
    fn folded_count(&self) -> usize { self.folded.len() }

    fn all_acted_and_agreed(&self) -> bool {
        debug_assert!(self.active_count() > 0);
        self.betting_queue.iter().all(|b| b.acted() && b.bet() == self.max_bet)
    }

    fn may_agree_by_checking(&self) -> bool {
        debug_assert!(self.active_count() > 0);
        self.betting_queue.iter().all(|b| b.bet() == self.max_bet)
    }

    fn next_bettor(&mut self) -> ActiveBettor<Player> {
        self.betting_queue.pop_front().unwrap()
    }

    fn insert_next_bettor(&mut self, bettor: ActiveBettor<Player>) {
        self.betting_queue.push_front(bettor);
    }

    fn add_folded(&mut self, folded: FoldedBettor<Player>) {
        self.folded.push(folded);
    }

    fn add_involved(&mut self, bettor: InvolvedBettor<Player>) {
        use self::InvolvedBettor::*;

        self.max_bet = max(self.max_bet, bettor.bet());
        match bettor {
            Active(bettor) => {
                self.betting_queue.push_back(bettor);
            },
            AllIn(bettor) => {
                self.all_in.push(bettor)
            },
        }
    }
}


struct ForcedBetting<Player, ForcedBettorIter> {
    forced: ForcedBettorIter,
    bettors: Bettors<Player>,
}

impl<Player, ForcedBettorIter> ForcedBetting<Player, ForcedBettorIter>
where
    ForcedBettorIter: Iterator<Item=ForcedBettor<Player>>,
{
    pub fn new(forced_bettors: ForcedBettorIter) -> Self {
        Self {
            forced: forced_bettors,
            bettors: Bettors::new(),
        }
    }

    fn finish(mut self) -> RoundState<Player> {
        assert!(self.forced.next().is_none());
        assert!(self.bettors.involved_count() >= 2);

        if self.bettors.active_count() == 0 {
            return RoundState::Showdown(Showdown{ bettors: self.bettors })
        }

        if self.bettors.active_count() == 1 && self.bettors.may_agree_by_checking() {
            return RoundState::Showdown(Showdown{ bettors: self.bettors })
        }

        return RoundState::Betting(Betting{ bettors: self.bettors })
    }
}

impl<Player, ForcedBettorIter> ForcedBetting<Player, ForcedBettorIter>
where
    ForcedBettorIter: Iterator<Item=ForcedBettor<Player>>,
    Player: Eq + Debug,
{
    fn make_forced_bet(&mut self, player: Player, bet: Chips) {
        self.bettors.add_involved(
            self.forced.next().unwrap().is(player).make_forced_bet(bet)
        );
    }
}


enum RoundState<Player> {
    Betting(Betting<Player>),
    InvalidBet(InvalidBet<Player>),
    Winner(Winner<Player>),
    Showdown(Showdown<Player>),
    Agreement(Agreement<Player>),
}


struct Betting<Player> {
    bettors: Bettors<Player>
}

impl<Player: Eq> Betting<Player> {
    fn fold(mut self, player: Player) -> RoundState<Player> {
        let folded = self.bettors.next_bettor().is(player).fold();
        self.bettors.add_folded(folded);

        debug_assert!(self.bettors.involved_count() >= 1);

        if self.bettors.involved_count() == 1 {
            RoundState::Winner(Winner{ bettors: self.bettors })
        } else if self.bettors.active_count() == 0 {
            RoundState::Showdown(Showdown{ bettors: self.bettors })
        } else if self.bettors.all_acted_and_agreed() {
            RoundState::Agreement(Agreement{ bettors: self.bettors })
        } else {
            RoundState::Betting(self)
        }
    }

    fn make_bet(mut self, player: Player, new_bet: Chips) -> RoundState<Player> {
        debug_assert!(self.bettors.active_count() >= 1);
        debug_assert!(self.bettors.involved_count() >= 2);

        let bettor = self.bettors.next_bettor().is(player);

        if new_bet == bettor.stake() || new_bet >= self.bettors.max_bet() {
            match bettor.make_bet(new_bet) {
                Err(error) => RoundState::InvalidBet(InvalidBet{ bettors: self.bettors, error: error }),
                Ok(bettor) => {
                    self.bettors.add_involved(bettor);

                    if self.bettors.active_count() == 0 {
                        RoundState::Showdown(Showdown{ bettors: self.bettors })
                    } else if self.bettors.all_acted_and_agreed() {
                        RoundState::Agreement(Agreement{ bettors: self.bettors })
                    } else {
                        RoundState::Betting(self)
                    }
                }
            }
        } else {
            RoundState::InvalidBet(InvalidBet{
                error: BetError::TooSmallBet {
                    bettor: bettor,
                    bet: new_bet,
                    min_bet: self.bettors.max_bet()
                },
                bettors: self.bettors,
            })
        }
    }
}


struct InvalidBet<Player> {
    bettors: Bettors<Player>,
    error: BetError<Player>,
}

impl<Player> InvalidBet<Player> {
    fn error(&self) -> &BetError<Player> { &self.error }
}

impl<Player: Eq> InvalidBet<Player> {
    fn fold_fix(mut self, player: Player) -> RoundState<Player> {
        self.bettors.insert_next_bettor(self.error.into_bettor());
        Betting{ bettors: self.bettors }.fold(player)
    }
}


struct Winner<Player> {
    bettors: Bettors<Player>
}

impl<Player> Winner<Player> {
}


struct Agreement<Player> {
    bettors: Bettors<Player>
}

impl<Player> Agreement<Player> {
}


struct Showdown<Player> {
    bettors: Bettors<Player>
}

impl<Player> Showdown<Player> {
}


#[cfg(test)]
mod test {
    use chips::Chips;

    use super::ActivePlayer;
    use super::ForcedBettor;
    use super::ForcedBetting;
    use super::RoundState::Betting;

    #[test]
    fn test() {
        let players = vec!(
            ForcedBettor::new(ActivePlayer::new(0, Chips::new(1000))),
            ForcedBettor::new(ActivePlayer::new(1, Chips::new(1000))),
            ForcedBettor::new(ActivePlayer::new(2, Chips::new(1000))),
        );
        let mut forced = ForcedBetting::new(players.into_iter());

        forced.make_forced_bet(0, Chips::new(100));
        forced.make_forced_bet(1, Chips::new(200));
        forced.make_forced_bet(2, Chips::new(0));

        let round = forced.finish();

        match round {
            Betting(betting) => {
                assert_eq!(betting.bettors.max_bet(), Chips::new(200));
                betting.make_bet(0, Chips::new(199));
            },
            _ => assert!(false),
        }
    }
}

