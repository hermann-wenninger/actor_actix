use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Player {
    name: String,
    score: u32,
}

impl Player {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            score: 0,
        }
    }

    fn give_points_to(target: &Rc<RefCell<Player>>, points: u32) {
        let mut target_ref = target.borrow_mut();
        target_ref.score += points;
        println!("{} bekommt {} Punkte! Neuer Stand: {}", target_ref.name, points, target_ref.score);
    }

    fn show(&self) {
        println!("{} hat {} Punkte.", self.name, self.score);
    }
}

fn main() {
    // Erstelle Spieler mit Rc<RefCell<_>>
    let alice = Rc::new(RefCell::new(Player::new("Alice")));
    let bob = Rc::new(RefCell::new(Player::new("Bob")));
    let carol = Rc::new(RefCell::new(Player::new("Carol")));

    // Alle Spieler in einem Vektor
    let players = vec![alice.clone(), bob.clone(), carol.clone()];

    // Alice gibt Bob 10 Punkte
    Player::give_points_to(&bob, 10);

    // Carol gibt Bob 5 Punkte
    Player::give_points_to(&bob, 5);

    // Alle anzeigen
    for player in players {
        player.borrow().show();
    }
}
