# PokéBrains

A Rust-based Pokémon Showdown battle client that connects to live battles, parses events, and analyzes gameplay in
real-time.

## Current State

- WebSocket connection to Pokémon Showdown servers
- Battle event parsing and logging
- Real-time battle monitoring
- Basic event tracking (no AI decision-making yet)

## Parse Demo

```shell
    
   ▄███████▄  ▄██████▄     ▄█   ▄█▄    ▄████████ ▀█████████▄     ▄████████    ▄████████  ▄█  ███▄▄▄▄      ▄████████ 
  ███    ███ ███    ███   ███ ▄███▀   ███    ███   ███    ███   ███    ███   ███    ███ ███  ███▀▀▀██▄   ███    ███ 
  ███    ███ ███    ███   ███▐██▀     ███    █▀    ███    ███   ███    ███   ███    ███ ███▌ ███   ███   ███    █▀  
  ███    ███ ███    ███  ▄█████▀     ▄███▄▄▄      ▄███▄▄▄██▀   ▄███▄▄▄▄██▀   ███    ███ ███▌ ███   ███   ███        
▀█████████▀  ███    ███ ▀▀█████▄    ▀▀███▀▀▀     ▀▀███▀▀▀██▄  ▀▀███▀▀▀▀▀   ▀███████████ ███▌ ███   ███ ▀███████████ 
  ███        ███    ███   ███▐██▄     ███    █▄    ███    ██▄ ▀███████████   ███    ███ ███  ███   ███          ███ 
  ███        ███    ███   ███ ▀███▄   ███    ███   ███    ███   ███    ███   ███    ███ ███  ███   ███    ▄█    ███ 
 ▄████▀       ▀██████▀    ███   ▀█▀   ██████████ ▄█████████▀    ███    ███   ███    █▀  █▀    ▀█   █▀   ▄████████▀  
                          ▀                                     ███    ███
    
Enter room ID: 
>>> https://play.pokemonshowdown.com/battle-gen9randombattle-2499921326
Enter your username: 
>>> Pochaxxx 
Connecting to room: battle-gen9randombattle-2499921326
Connection established!


Index: 0
 Event: [" TURN 1 ", "[Assist] Pachirisu used U-turn on [Against] Ampharos", "[Against] Ampharos HP: 89/100", "[Assist] Iron Treads switched to Iron Treads (100/100)", "[Against] Ampharos used Agility on [Against] Ampharos", "[Against] Ampharos's spe rose by 2"]
Index: 1
 Event: [" TURN 2 ", "[Against] Ampharos used Focus Blast on [Assist] Iron Treads", "Super effective on [Assist] Iron Treads!", "[Assist] Iron Treads HP: 11/100", "[Assist] Iron Treads's spd fell by 1", "[Against] Ampharos HP: 79/100 (from [from] item: Life Orb)", "[Assist] Iron Treads used Earthquake on [Against] Ampharos", "Super effective on [Against] Ampharos!", "[Against] Ampharos HP: 12/100"]
Index: 2
 Event: [" TURN 3 ", "[Assist] Mismagius switched to Mismagius (100/100)", "[Against] Ampharos used Dazzling Gleam on [Assist] Mismagius", "[Assist] Mismagius HP: 67/100", "[Against] Ampharos HP: 2/100 (from [from] item: Life Orb)"]
Index: 3
 Event: [" TURN 4 ", "[Against] Ampharos used Thunderbolt on [Assist] Mismagius", "[Assist] Mismagius HP: 8/100", "[Against] Ampharos HP: 0 fnt (from [from] item: Life Orb)", "[Against] Ampharos fainted!", "[Assist] Mismagius used Shadow Ball on [Against] Ampharos", "[Against] Iron Boulder switched to Iron Boulder (100/100)"]
Index: 4
 Event: [" TURN 5 ", "[Against] Iron Boulder used Mighty Cleave on [Assist] Mismagius", "[Assist] Mismagius HP: 0 fnt", "[Assist] Mismagius fainted!", "[Assist] Krookodile switched to Krookodile (100/100)", "[Assist] Krookodile's ability: Intimidate", "[Against] Iron Boulder's atk fell by 1"]
Index: 5
 Event: [" TURN 6 ", "[Against] Iron Boulder used Swords Dance on [Against] Iron Boulder", "[Against] Iron Boulder's atk rose by 2", "[Assist] Krookodile used Knock Off on [Against] Iron Boulder", "[Against] Iron Boulder resisted the attack", "[Against] Iron Boulder HP: 77/100", "[Assist] Krookodile HP: 91/100 (from [from] item: Life Orb)"]
Index: 6
 Event: [" TURN 7 ", "[Against] Iron Boulder used Close Combat on [Assist] Krookodile", "Super effective on [Assist] Krookodile!", "[Assist] Krookodile HP: 0 fnt", "[Against] Iron Boulder's def fell by 1", "[Against] Iron Boulder's spd fell by 1", "[Assist] Krookodile fainted!", "[Assist] Oricorio switched to Oricorio-Pa'u (100/100)"]

```

## TODOs

- Implement AI-powered move suggestions using rig-core
- Add PokéAPI integration for stats and type matchups
- Team battle support
- TUI interface for better visualization
- Battle strategy analysis and recommendations
- Historical battle data persistence

## Contributing

Pull requests welcome! This is an experimental/fun project to defeat that Pokémon **nerd** with AI slop.

