#![allow(dead_code)]

use rand::Rng;

extern crate rand;

struct Weapon {
    physical_damage: i32,
    fire_damage: i32,
    lightning_damage: i32,
}

fn basic_attack(w: Weapon) {
    let beast_health = 10;
    println!("The beasts health is {}", beast_health);
    if w.physical_damage > beast_health {
        println!("The best is dead!")
    } else if w.physical_damage < beast_health {
        println!("Uh-oh")
    } else if w.physical_damage == beast_health {
        println!("Its a draw!")
    }
}
fn main() {
    let new_weapon = Weapon {
        physical_damage: rand::thread_rng().gen_range(0..100), 
        fire_damage: rand::thread_rng().gen_range(0..100),
        lightning_damage: rand::thread_rng().gen_range(0..100) 
    };
    println!("Physical Damage: {}, Fire Damage: {}, Lightning Damage: {}", new_weapon.physical_damage, new_weapon.fire_damage, new_weapon.lightning_damage);
    basic_attack(new_weapon);

}
