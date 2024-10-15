#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

fn main() {
    let suits = vec!["Hearts", "Diamonds", "Spades"];
    let values = vec!["Ace", "Two", "Three"];

    let mut cards = vec![];

    for suit in suits {
        for value in &values {
            let card = format!("{} of {}", value, suit);
            cards.push(card);
        }
    }
    let deck = Deck{ cards: cards};
    println!("Here is your deck: {:#?}", deck);
}
