use std::fs;
use std::cmp::Ordering;
use std::collections::HashMap;

/*
Five of a kind, where all five cards have the same label: AAAAA
Four of a kind, where four cards have the same label and one card has a different label: AA8AA
Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
High card, where all cards' labels are distinct: 23456
*/

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum HandKind {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug)]
struct Hand {
    cards: String,
    score: usize,
    kind: HandKind,
    j_is_weak: bool,
}

impl From<String> for HandKind {
    fn from(item: String) -> Self {
        let mut cards : HashMap<char, usize> = HashMap::new();
        for c in item.chars() {
            *cards.entry(c).or_insert(0) += 1;
        }

        let mut has_three = false;
        let mut count_pairs = 0;

        // check if has 5 or 4
        for &v in cards.values() {
            if v == 5 {
                return Self::FiveOfAKind;
            }
            if v == 4 {
                return Self::FourOfAKind;
            }
            if v == 3 {
                has_three = true;
            }
            if v == 2 {
                count_pairs += 1;
            }
        }

        // check if full house
        if has_three && count_pairs > 0 {
            Self::FullHouse
        } else if has_three {
            Self::ThreeOfAKind
        } else if count_pairs == 2 {
            Self::TwoPair
        } else if count_pairs == 1 {
            Self::OnePair
        } else {
            Self::HighCard
        }
    }
}

impl From<&str> for Hand {
    fn from(item: &str) -> Self {
        let cards = item.split(' ').next().unwrap().to_string();
        let score = item.split(' ').nth(1).unwrap().parse::<usize>().unwrap();
        let kind = cards.clone().into();

        Hand { 
            cards,
            score,
            kind,
            j_is_weak: false,
        }
    }
}

// only to get part2 best possible hand
impl From<String> for Hand {
    fn from(item: String) -> Self {
        let cards = item.to_string();
        let kind = cards.clone().into();

        Hand { 
            cards,
            score: 0,
            kind,
            j_is_weak: false,
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Hand) -> bool {
        self.cards == other.cards
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let mut cards_order: Vec<char> = ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'].into();

        if self.j_is_weak {
            cards_order = ['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'].into();
        }

        if self.kind != other.kind {
            return Some(self.kind.cmp(&other.kind));
        }

        for (idx, c) in self.cards.chars().enumerate() {
            let o = other.cards.chars().nth(idx).unwrap();
            // A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2
            if c != o {
                for &high_card in &cards_order {
                    if o == high_card {
                        return Some(Ordering::Greater);
                    } else if c == high_card {
                        return Some(Ordering::Less);
                    }
                }
            }
        }

        Some(Ordering::Equal)
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn get_variants(s: String) -> Vec<String> {
    let mut result = Vec::new();
    let cards_order: Vec<char> = ['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2'].into();

    if s.is_empty() {
        return Vec::new();
    }

    if s.starts_with('J') {
        for c in cards_order {
            let subs = get_variants(s[1..].to_string());
            let s = String::from(c);

            if subs.is_empty() {
                result.push(s);
            } else {
                for sub in subs {
                    result.push(s.clone() + &sub.clone());
                }    
            }
        }
    } else {
        let subs = get_variants(s[1..].to_string());
        let s = String::from(&s[0..1]);

        if subs.is_empty() {
            result.push(s);
        } else {
            for sub in subs {
                result.push(s.clone() + &sub.clone());
            }    
        }
    }

    result
}

fn build_best_possible_hands(hand: &Hand) -> Hand {
    let variants = get_variants(hand.cards.clone());

    let mut hands: Vec<Hand> = Vec::new();

    for variant in variants {
        hands.push(variant.into());
    }

    hands.sort();

    let mut best_hand_kind: Option<HandKind> = None;

    for new_hand in hands {
        // println!("{:?} -> {:?}", new_hand, hand);
        if best_hand_kind.is_none() {
            best_hand_kind = Some(new_hand.kind);
        }
    }

    Hand {
        cards: hand.cards.clone(),
        score: hand.score,
        kind: best_hand_kind.unwrap(),
        j_is_weak: true,
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("file");
    let lines = contents.lines().collect::<Vec<&str>>();

    let mut hands: Vec<Hand> = Vec::new();

    for line in lines {
        hands.push(line.into());
    }

    hands.sort();
    hands.reverse();

    let mut p1 = 0;

    for (idx, hand) in hands.iter().enumerate() {
        p1 += hand.score * (idx + 1);
    }

    println!("#1 {}", p1);

    let mut hands2: Vec<Hand> = Vec::new();

    for hand in &hands {
        hands2.push(build_best_possible_hands(hand));
    }

    hands2.sort();
    hands2.reverse();

    let mut p2 = 0;

    for (idx, hand) in hands2.iter().enumerate() {
        p2 += hand.score * (idx + 1);
    }

    println!("#2 {}", p2);

}
