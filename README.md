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
>>> https://play.pokemonshowdown.com/battle-gen6ou-2501511108
Enter your username: 
>>> ronak777
AI Agent initialized successfully!
Connecting to room: battle-gen6ou-2501511108
Connection established!

Generating initial strategy...

[DEBUG] Prompt Sent to Agent:
Battle Title: kashimo777 vs. ronak777
Generation: 6
You are assisting: ronak777

Player 1: "kashimo777", Team: ["Amoonguss", "Bisharp", "Clefable", "Dragonite", "Excadrill", "Latios"]
Player 2: "ronak777", Team: ["Dragonite", "Zoroark", "Chansey", "Azumarill", "Charizard", "Gengar"]

Which Pokemon should lead with and why?


[AI SUGGESTION]
For ronak777, the optimal choice for a lead Pokemon depends on several factors including type matchups, abilities, and synergies within the team. Here's an analysis:

1. **Dragonite (Shared by both teams)**: This is a powerful Dragon/Flying type with high stats in Attack, Defense, and Speed. It can deal significant damage with its STAB (Same Type Attack Bonus) Dragon-type moves. However, it's shared between the teams, so using it as a lead might not provide an immediate advantage if kashimo777 also leads with Dragonite.

2. **Zoroark**: This is a Dark/Fiction type Pokemon with excellent offensive stats and access to powerful moves like Night Daze and Foul Play (which copies the opponent's attack stat). Its ability, Illusion, can confuse the opponent about its true type, providing an element of surprise. It also has decent defenses and can set up hazards or use Taunt to disrupt the opponent's strategy.

3. **Chansey**: This is a dual-type Normal/Fairy Pokemon known for its high HP stat, making it a great tank. Its ability, Natural Cure heals status conditions at the end of each turn. While it lacks offensive prowess, it can stall and wear down opponents with Toxic or status moves while using its recovery moves to stay healthy.

Considering these options:

- **Zoroark** seems like a solid choice for ronak777 as the lead Pokemon. Its Dark typing provides a good matchup against Amoonguss (Grass/Poison) and Clefable (Fairy), two of kashimo777's potential leads. Zoroark can also use its Illusion ability to potentially catch kashimo777 off guard about the true type of its lead, giving ronak777 an early psychological advantage.

- **Dragonite** might be a better choice if ronak777 anticipates kashimo777 leading with something other than Dragonite or a type that Dragonite is weak against (like Ice, which isn't on either team). 

- **Chansey**, while a solid choice for stalling and wearing down opponents over time, might not be the best lead as it lacks the offensive punch to quickly threaten kashimo777's lineup.

So, ronak777 should consider leading with Zoroark due to its versatile typing, powerful moves, and potential for surprise via Illusion.

Generating turn suggestion...

[DEBUG] Prompt Sent to Agent:
[Against: kashimo777] Latios switched in Latios (100/100)
[Assist: ronak777] Gengar switched in Gengar (100/100)
Based on the current battle state, what is the optimal move or switch?


[AI SUGGESTION]
Given the current battle state where both Latios and Gengar have been switched in simultaneously, here's an analysis to determine the optimal move:

- **Latios** (Psychic/Dragon) has high Special Attack and Speed stats. Its primary STAB moves are Psychic and Dragon Pulse, which can deal significant damage to many Pokemon, especially those weak to Psychic or Dragon types.

- **Gengar** (Ghost/Poison) is known for its high Special Attack and decent Speed. Its STAB moves include Shadow Ball and Sludge Wave, both of which are super effective against Latios due to Ghost's immunity to Psychic attacks and Poison's effectiveness against Dragons.

Considering these factors:

1. **Gengar's optimal move**: Given that Gengar is super effective against Latios, the best move would be Shadow Ball. This move not only deals high damage but also has a chance to flinch (make Latios miss its next turn) due to Gengar's ability, "Insomnia".

2. **Latios' potential switch**: If ronak777 is concerned about the matchup and wants to avoid taking heavy damage from Shadow Ball, they could consider switching out Latios for a Pokemon that resists or is immune to Ghost-type moves. Examples include Ground types (like Landorus-Therian or Gliscor) or Fairy types (like Xerneas or Sylveon).

3. **Consideration of team synergy**: If ronak777 has a Pokemon in their lineup that can take advantage of a potential Gengar switch (for example, a Chansey to stall with Toxic or a Charizard to outspeed and OHKO), they might choose not to switch Latios immediately.

Therefore, the optimal move for ronak777's Gengar in this situation is **Shadow Ball**. This move maximizes damage output and takes advantage of the super effective type matchup against Latios. If ronak777 decides to switch Latios out instead, it would depend on their team composition and strategy.
```

## NOTE!!

I know...I know
AI Suggestions are very MID and WORSE!!!, probably because of less trained on Pokémon

## TODOs

- Add more context to AI using TEAM_BUILDER and PokéAPI
- Add PokéAPI integration for stats and type matchups and AI reasoning help/context
- TUI interface for better visualization
- Historical battle data persistence
- Team battle support

## Contributing

Pull requests welcome! This is an experimental/fun project to combine nostalgia wth new AI slop thingy.

