use pokebrains::fetch_pokemon_info;
use pokebrains::{Colorize, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let data = fetch_pokemon_info("Latios").await;
    match data {
        Ok(info) => {
            println!();
            println!("Pokemon : {}", info.name.bright_white().bold());
            print!("{}", "Types: ".bright_red().bold());
            for t in info.types.iter() {
                print!(" {} ", t.r#type.name.bright_red());
            }
            println!();
            println!("{}", "Stats: ".bright_blue().bold());
            for stat in info.stats {
                println!(
                    " {} : {}",
                    stat.stat.name.bright_blue(),
                    stat.base_stat.to_string().bright_blue()
                );
            }
            println!(
                " Height: {}\n Weight: {}",
                info.height.to_string().bright_magenta(),
                info.weight.to_string().bright_magenta()
            );
            println!();
            println!("{}", "Abilities: ".on_bright_green().bold());
            for ability in info.abilities {
                println!(
                    " {} (hidden: {})",
                    ability.ability.name.bright_green(),
                    ability.is_hidden.to_string().bright_green()
                );
            }
            println!();
            println!("{}", "Moves: ".bright_red().bold());
            for mv in info.moves.iter().take(4) {
                println!(" {}", mv.r#move.name.to_string().red());
            }

            println!(
                "Total Moves: {}",
                info.moves.len().to_string().bright_yellow()
            );
        }
        Err(e) => {
            eprintln!("Error fetching data: {}", e);
        }
    }

    Ok(())
}
