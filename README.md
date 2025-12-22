# PokéBrains

A Rust-based Pokémon Showdown battle client that connects to live battles, parses events, and analyzes/sugggests
game-move in
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
↪ https://play.pokemonshowdown.com/battle-gen6ou-2504415399
Enter your username: 
↪ ronak777
AI Agent initialized successfully!
Connecting to room: battle-gen6ou-2504415399
Connection established!


Generating initial suggestions...


[DEBUG] Prompt Sent to Agent:
Battle Title: kashimo777 vs. ronak777
Generation: 6
You are assisting: ronak777

Player 1: "kashimo777", Team: ["Amoonguss", "Bisharp", "Clefable", "Dragonite", "Excadrill", "Latios"]
Player 2: "ronak777", Team: ["Dragonite", "Keldeo", "Chansey", "Azumarill", "Charizard", "Gengar"]

Which Pokemon should lead with and why?

Action: Gengar
Reason: Gengar's Ghost/Poison typing resists common threats like Fire/Steel, and its high Special Attack allows it to set up and OHKO key opponents
with moves like Shadow Ball or Sludge Bomb.


Generating turn suggestions...


[DEBUG] Prompt Sent to Agent:
[Against: kashimo777]: Latios sent out (Latios) HP: 100/100
[Assist: ronak777]: Gengar sent out (Gengar) HP: 100/100
Based on the current battle state, what is the optimal move or switch?

Action: Ice Beam
Reason: Ice Beam (Ice) is super effective against Dragon types like Latios, and Gengar's Poison typing grants it immunity to Ground moves, making it a
strong counter.
```

## NOTE!!

I know...I know
AI Suggestions are very MID and WORSE!!!, probably because of less trained on Pokémon, like When Genger knows Ice
Beam????
But sure....the AI's reasoning and suggestions can improved over time with better prompts, more context, and better
models.

## TODOs

- Add more context to AI using TEAM_BUILDER and PokéAPI
- Add PokéAPI integration for stats and type matchups and AI reasoning help/context
- TUI interface for better visualization
- Historical battle data persistence
- Team battle support

## Contributing

Pull requests welcome! This is an experimental/fun project to combine nostalgia wth new AI slop thingy.

