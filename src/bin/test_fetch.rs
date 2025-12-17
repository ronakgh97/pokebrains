use pokebrains::Result;
use pokebrains::display;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let pokemon_name = if args.len() > 1 { &args[1] } else { "Rayquaza" };

    display(pokemon_name).await?;

    Ok(())
}
