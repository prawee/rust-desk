#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}
fn main() {
    let deck: Deck = Deck { cards: vec![] };
    println!("Heres your deck: {:?}", deck);
}
