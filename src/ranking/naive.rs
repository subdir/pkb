use Card;
use Value;
use Rank;
use rank::Straight;
use ordered_iter::{OrderedIterator, StrictDescending};
use sequential::Sequential;


fn find_best_straight<I>(mut iter: I) -> Option<(Rank, Vec<Card>)>
where
    I: OrderedIterator<Order=StrictDescending, Item=Card>
{
    match iter.next() {
        None => None,
        Some(first) => {
            let mut straight = Vec::<Card>::with_capacity(5);
            straight.push(first);

            for card in iter {
                if card.value() == straight.last().unwrap().value() {
                    continue;
                } if card.value().consequent().unwrap() != straight.last().unwrap().value() {
                    straight.clear();
                    straight.push(card);
                } else {
                    straight.push(card);
                    if straight.len() == 5 {
                        return Some((
                            Rank::straight(Straight::new(straight.first().unwrap().value())),
                            straight
                        ))
                    }
                }
            }

            None
        }
    }
}


struct ValueCounter {
    counters: [u8; Value::VARIANTS_NUM]
}


impl ValueCounter {
    fn count(&mut self, value: Value) {
        self.counters[value.to_serial()] += 1;
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use Value::*;
    use ordered_iter::OrderEnsurableIterator;

    #[test]
    fn test_find_best_straight_empty() {
        assert_eq!(
            find_best_straight(Vec::<Card>::new().into_iter().assert_order(StrictDescending)),
            None
        );
    }

    #[test]
    fn test_find_best_straight_short() {
        let cards = "AH KH QH JD".split(" ").map(|s| Card::from_str(s)).collect::<Vec<Card>>();
        assert_eq!(
            find_best_straight(cards.clone().into_iter().assert_order(StrictDescending)),
            None
        );

    }

    #[test]
    fn test_find_best_straight_doubled() {
        let cards = "AH AS KH QH JC JD JH JS TD".split(" ").map(|s| Card::from_str(s)).collect::<Vec<Card>>();
        assert_eq!(
            find_best_straight(cards.clone().into_iter().assert_order(StrictDescending)),
            Some((Rank::straight(Straight::new(Ace)), "AH KH QH JC TD".split(" ").map(|s| Card::from_str(s)).collect::<Vec<Card>>()))
        );

    }

    #[test]
    fn test_find_best_straight_skipped() {
        let cards = "AH AS QH JH TD 9C 8D".split(" ").map(|s| Card::from_str(s)).collect::<Vec<Card>>();
        assert_eq!(
            find_best_straight(cards.clone().into_iter().assert_order(StrictDescending)),
            Some((Rank::straight(Straight::new(Queen)), "QH JH TD 9C 8D".split(" ").map(|s| Card::from_str(s)).collect::<Vec<Card>>()))
        );
    }
}

