use rand::Rng;  // For .choose()
use std::io;
use std::fmt;

#[derive(Debug)]
enum Guess {
    Higher,
    Lower,
} 
use Guess::*;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Suit {
    Spades,
    Clubs,
    Diamonds,
    Hearts
}
use Suit::*;

impl std::fmt::Debug for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Spades => write!(f, "spades"),
            Clubs => write!(f, "clubs"),
            Diamonds => write!(f, "diamonds"),
            Hearts => write!(f, "hearts"),
        }
    }
 }

const SUITS: [Suit;4] = [
    Spades,
    Clubs,
    Diamonds,
    Hearts
];

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
 }
use Rank::*;

impl std::fmt::Debug for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ace => write!(f, "Ace"),
            King => write!(f, "King"),
            Queen => write!(f, "Queen"), 
            Jack => write!(f, "Jack"),
            Ten => write!(f, "10"),
            Nine => write!(f, "9"),
            Eight => write!(f, "8"),
            Seven => write!(f, "7"),
            Six => write!(f, "6"),
            Five => write!(f, "5"),
            Four => write!(f, "4"),
            Three => write!(f, "3"),
            Two => write!(f, "2"),
        }
    }
}

const RANKS: [Rank;13] = [
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two
];

#[derive(PartialEq, Eq, PartialOrd, Clone, Copy, Debug)]
struct Card {
    rank: Rank,
    suit: Suit
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} of {:?}", self.rank, self.suit)
    }
}

fn correct(guess: &Guess, new: &Card, previous: &Card) -> bool {
    match guess {
        &Higher => new.rank>previous.rank,
        &Lower => new.rank<previous.rank
    }
}

fn input_guess() -> Guess {
    loop {
        match input_char() {
            Ok(input) => {
                let input = input.to_ascii_uppercase();
                match input {
                    'H' => {return Higher;},
                    'L' => {return Lower;},
                    _ => {
                        println!("That wasn't a valid input!\nPlease try again with either \"H\" or \"L\".");
                    }
                }
            }
            Err(_) => {
                println!("That wasn't a valid input!\nPlease try again with either \"H\" or \"L\".");
            }
        }
    }
}

fn input_char() -> Result<char,&'static str> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input = input.trim();
    if input.len() == 1 {
        if let Some(c) = input.chars().next() {
            Ok(c)
        } else {
            Err("Should be unreachable")
        }
    } else {
        Err("Invalid character input")
    }
}

fn main() {
    let mut cards : Vec<Card> = Vec::new();
    for rank in RANKS {
        for suit in SUITS {
            cards.push(Card {rank,suit});
        }
    }
    let index = rand::thread_rng().gen_range(0..cards.len());
    let mut random_cards: [Option<Card>;2] = [Some(cards.swap_remove(index)),None];
    println!("Welcome to Higher or Lower! After each card, please guess either \"H\" or \"L\".");
    println!("First card: {}", random_cards[0].unwrap());
    for _ in 0..5 {
        let guess = input_guess();
        let index = rand::thread_rng().gen_range(0..cards.len());
        random_cards[1] = Some(cards.swap_remove(index));
        if let [Some(prev_card),Some(new_card)] = random_cards {
            println!("Next card: {new_card}");
            if correct(&guess,&new_card,&prev_card) {
                random_cards[0] = random_cards[1];
            } else {
                println!("Wrong!\nYou lose!\nGoodbye!");
                return;
            }
        }
    }
    println!("Congratulations!\nYou Win!\nGoodbye!");
}
