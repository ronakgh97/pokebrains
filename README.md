# PokéBrains

A Rust-based Pokémon Showdown battle client that connects to live battles, parses events, and analyzes/sugggests
game-move in
real-time.

## Current State

- WebSocket connection to Pokémon Showdown servers
- Battle event parsing and logging
- Real-time battle monitoring
- Basic event tracking (no AI decision-making yet)

## Pokebrains Demo

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

## Teamdex Demo

```shell
    ███        ▄████████    ▄████████   ▄▄▄▄███▄▄▄▄        ████████▄     ▄████████ ▀████    ▐████▀
▀█████████▄   ███    ███   ███    ███ ▄██▀▀▀███▀▀▀██▄      ███   ▀███   ███    ███   ███▌   ████▀
   ▀███▀▀██   ███    █▀    ███    ███ ███   ███   ███      ███    ███   ███    █▀     ███  ▐███
    ███   ▀  ▄███▄▄▄       ███    ███ ███   ███   ███      ███    ███  ▄███▄▄▄        ▀███▄███▀
    ███     ▀▀███▀▀▀     ▀███████████ ███   ███   ███      ███    ███ ▀▀███▀▀▀        ████▀██▄
    ███       ███    █▄    ███    ███ ███   ███   ███      ███    ███   ███    █▄    ▐███  ▀███
    ███       ███    ███   ███    ███ ███   ███   ███      ███   ▄███   ███    ███  ▄███     ███▄
   ▄████▀     ██████████   ███    █▀   ▀█   ███   █▀       ████████▀    ██████████ ████       ███▄

    

Enter your team requirements: 
↪ Gen 6, include Mega charizard X

Generating team...

Charizard-Mega-X @ Choice Scarf
Ability: Mega Launcher
EVs: 252 Atk / 4 SpA / 252 Spe
Jolly Nature
- Dragon Ascent
- Fire Blast
- Thunderbolt
- Swords Dance

Tapu Koko @ Life Orb
Ability: Starmask
EVs: 252 SpA / 252 SpD
Modest Nature
- Ice Beam
- Shadow Ball
- Psychic
- Trick

Ferrothorn @ Leftovers                                                                                                                                             
Ability: Iron Barbs                                                                                                                                                
EVs: 252 HP / 252 Def / 4 SpD                                                                                                                                      
Relaxed Nature                                                                                                                                                     
- Leech Seed                                                                                                                                                       
- Spiky Shield                                                                                                                                                     
- Sludge Bomb                                                                                                                                                      
- Swords Dance                                                                                                                                                     
                                                                                                                                                                   
Togekiss @ Choice Band                                                                                                                                             
Ability: Serene Grace                                                                                                                                              
EVs: 252 HP / 252 Atk / 4 Spe                                                                                                                                      
Adamant Nature                                                                                                                                                     
- Bullet Seed                                                                                                                                                      
- Close Combat                                                                                                                                                     
- Swagger                                                                                                                                                          
- Play Rough                                                                                                                                                       
                                                                                                                                                                   
Sylveon @ Leftovers                                                                                                                                                
Ability: Pixilate                                                                                                                                                  
EVs: 252 SpA / 252 Spe                                                                                                                                             
Timid Nature                                                                                                                                                       
- Moonblast                                                                                                                                                        
- Dazzling Gleam                                                                                                                                                   
- Ice Beam                                                                                                                                                         
- Psychic                                                                                                                                                          
                                                                                                                                                                   
Gligar @ Life Orb                                                                                                                                                  
Ability: Sand Force                                                                                                                                                
EVs: 252 HP / 4 Def / 252 SpD                                                                                                                                      
Sassy Nature                                                                                                                                                       
- Swords Dance                                                                                                                                                     
- Earthquake                                                                                                                                                       
- Rock Slide                                                                                                                                                       
- Slack Off                                                                                                                                                        

Enter your team requirements: 
↪ Is the team valid?

Generating team...
                                                                                                                                                                   
The team has several issues due to **Gen 5 compatibility** (Pokémon Showdown uses Gen 8 by default). Here's how to fix it:
                                                                                                                                                                   
---                                                                                                                                                                
                                                                                                                                                                   
### 🔥 **Charizard-Mega-X**                                                                                                                                        
- **Item**: Must be *Charizardite X* (not Choice Scarf)                                                                                                            
- **Ability**: Cannot use *Mega Launcher* (Gen 5 doesn't have this ability)                                                                                        
- **Move**: *Dragon Ascent* is Gen 6+; replace with **Outrage** or **Dragon Claw**                                                                                 
                                                                                                                                                                   
---                                                                                                                                                                
                                                                                                                                                                   
### 🌟 **Tapu Koko**                                                                                                                                               
- **Invalid in Gen 5**: Replace with a Gen 5 Pokémon like **Alola Muk** (use *Starmask* ability if needed)                                                         
                                                                                                                                                                   
---                                                                                                                                                                
                                                                                                                                                                   
### ⚡ **Ferrothorn**                                                                                                                                               
- **Spiky Shield** is Gen 6+; replace with **Sludge Bomb** or **Leech Seed**                                                                                       
                                                                                                                                                                   
---                                                                                                                                                                
                                                                                                                                                                   
### 🐾 **Togekiss**                                                                                                                                                
- **Swagger** is banned in most metagames                                                                                                                          
- **Bullet Seed/Play Rough** are Gen 4–5 moves, but *Bullet Seed* is banned in many formats                                                                        
- Consider replacing with **Alakazam** or **Blissey**                                                                                                              
                                                                                                                                                                   
---                                                                                                                                                                
                                                                                                                                                                   
### 🌙 **Sylveon**                                                                                                                                                 
- **Pixilate** and **Dazzling Gleam** are Gen 7+; replace with **Alomomola** (for healing) or **Crobat** (for speed)                                               
                                                                                                                                                                   
---                                                                                                                                                                
                                                                                                                                                                   
### ⛰️ **Gligar**                                                                                                                                                  
- **Sand Force** is Gen 7+; replace with *Sand Veil* or *Sturdy*                                                                                                   
- **Slack Off** is Gen 6+; replace with **Rest** or **Sleep Talk**                                                                                                 
                                                                                                                                                                   
---                                                                                                                                                                
                                                                                                                                                                   
### ✅ Revised Team (Gen 5 Compatible)                                                                                                                                                                                                                                                                            
Charizardite X @ Choice Scarf                                                                                                                                      
Ability: Blaze                                                                                                                                                     
EVs: 252 Atk / 4 SpA / 252 Spe                                                                                                                                     
Jolly Nature                                                                                                                                                       
- Outrage                                                                                                                                                          
- Fire Blast                                                                                                                                                       
- Thunderbolt                                                                                                                                                      
- Swords Dance                                                                                                                                                     
                                                                                                                                                                   
Alola Muk @ Life Orb                                                                                                                                               
Ability: Starmask                                                                                                                                                  
EVs: 252 SpA / 252 SpD                                                                                                                                             
Modest Nature                                                                                                                                                      
- Ice Beam                                                                                                                                                         
- Shadow Ball                                                                                                                                                      
- Psychic                                                                                                                                                          
- Trick                                                                                                                                                            
                                                                                                                                                                   
Ferrothorn @ Leftovers                                                                                                                                             
Ability: Iron Barbs                                                                                                                                                
EVs: 252 HP / 252 Def / 4 SpD                                                                                                                                      
Relaxed Nature                                                                                                                                                     
- Leech Seed                                                                                                                                                       
- Sludge Bomb                                                                                                                                                      
- Earthquake                                                                                                                                                       
- Swords Dance                                                                                                                                                     
                                                                                                                                                                   
Alakazam @ Choice Band                                                                                                                                             
Ability: Magic Guard                                                                                                                                               
EVs: 252 SpA / 252 Spe                                                                                                                                             
Timid Nature                                                                                                                                                       
- Psychic                                                                                                                                                          
- Shadow Ball                                                                                                                                                      
- Thunderbolt                                                                                                                                                      
- Rest                                                                                                                                                             
                                                                                                                                                                   
Crobat @ Choice Scarf                                                                                                                                              
Ability: Infiltrator                                                                                                                                               
EVs: 252 Spe / 4 HP / 252 SpD                                                                                                                                      
Timid Nature                                                                                                                                                       
- Air Slash                                                                                                                                                        
- U-turn                                                                                                                                                           
- Nasty Plot                                                                                                                                                       
- Sucker Punch                                                                                                                                                     
                                                                                                                                                                   
Gligar @ Focus Sash                                                                                                                                                
Ability: Sand Veil                                                                                                                                                 
EVs: 252 Def / 4 HP / 252 Spe                                                                                                                                      
Modest Nature                                                                                                                                                      
- Earthquake                                                                                                                                                       
- Rock Slide                                                                                                                                                       
- Stealth Rock                                                                                                                                                     
- Rest                                                                                                                                                             

This team avoids banned moves, invalid abilities, and Gen 5-incompatible Pokémon. Let me know if you want adjustments
for specific metagames (e.g.,                
UU, RU)!

Enter your team requirements: 
↪

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

