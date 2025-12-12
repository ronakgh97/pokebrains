use pokebrains::fetch_pokemon_info;
use pokebrains::{Colorize, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let data = fetch_pokemon_info("Darkrai").await;
    match data {
        Ok(info) => {
            println!("Fetched data for: {}", info.name.bold());
            print!("Pokemon types: ");
            for t in info.types.iter() {
                print!("{} ", t.r#type.name.bright_white());
            }
            println!();
            println!("Pokemon Stats");
            println!("Height: {}\nWeight: {}", info.height, info.weight);
            for stat in info.stats {
                println!("  {}: {}", stat.stat.name, stat.base_stat);
            }
            println!("Pokemon Abilities:");
            for ability in info.abilities {
                println!("  {} (hidden: {})", ability.ability.name, ability.is_hidden);
            }
            println!("Pokemon Moves:");
            for mv in info.moves.iter().take(4) {
                println!("  {}", mv.r#move.name);
            }

            println!("Total Moves: {}", info.moves.len());
        }
        Err(e) => {
            eprintln!("Error fetching data: {}", e);
        }
    }

    Ok(())
}
