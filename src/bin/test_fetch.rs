use pokebrains::fetch_pokemon_info;
use pokebrains::{Colorize, Result};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let pokemon_name = if args.len() > 1 { &args[1] } else { "Rayquaza" };

    let data = fetch_pokemon_info(pokemon_name).await;
    match data {
        Ok(info) => {
            println!();
            println!(
                "{} {}",
                "Pokemon:".cyan().bold(),
                info.name.to_uppercase().bright_white().bold()
            );

            print!("{}", "Types:   ".cyan().bold());
            let types: Vec<String> = info
                .types
                .iter()
                .map(|t| t.r#type.name.yellow().to_string())
                .collect();
            println!("{}", types.join(", "));

            println!(
                "{} {} | {} {}",
                "Height:".cyan().bold(),
                info.height.to_string().green(),
                "Weight:".cyan().bold(),
                info.weight.to_string().green()
            );

            println!();
            println!("{}", "Stats:".cyan().bold());
            for stat in info.stats {
                println!(
                    "  {:>15} : {}",
                    stat.stat.name.white(),
                    stat.base_stat.to_string().green().bold()
                );
            }

            println!();
            println!("{}", "Abilities:".cyan().bold());
            for ability in info.abilities {
                let hidden_text = if ability.is_hidden {
                    "(Hidden)".red().italic()
                } else {
                    "".normal()
                };
                println!("  {:>15} {}", ability.ability.name.white(), hidden_text);
            }

            println!();
            println!(
                "{} (Total: {})",
                "Moves:".cyan().bold(),
                info.moves.len().to_string().green()
            );
            for mv in info.moves.iter().take(4) {
                println!("  - {}", mv.r#move.name.white());
            }
            if info.moves.len() > 4 {
                println!("  ... and {} more", info.moves.len() - 4);
            }
        }
        Err(e) => {
            eprintln!("{} {}", "Error fetching data:".red().bold(), e);
        }
    }

    Ok(())
}
