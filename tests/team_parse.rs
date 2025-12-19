use pokebrains::Team;
static TEAM: &str = "#\
Dragonite @ Choice Scarf
Ability: Inner Focus
EVs: 100 HP / 64 Atk / 52 Def / 132 SpA / 84 SpD / 76 Spe
- Blizzard
- Draco Meteor
- Body Slam
- Earthquake

Zoroark @ Assault Vest
Ability: Illusion
EVs: 60 HP / 36 Atk / 116 Def / 84 SpA / 68 SpD / 120 Spe
Lonely Nature
- Calm Mind
- Foul Play
- Shadow Claw
- Dark Pulse

Chansey (F) @ Lucky Punch
Ability: Natural Cure
EVs: 208 HP / 156 Def / 144 SpD
- Return
- Blizzard
- Aromatherapy
- Facade

Azumarill @ Life Orb
Ability: Thick Fat
EVs: 80 HP / 100 Atk / 84 Def / 60 SpA / 68 SpD / 116 Spe
- Aqua Tail
- Play Rough
- Ice Punch
- Body Slam

Charizard-Mega-X @ Charizardite X
Ability: Tough Claws
EVs: 124 HP / 80 Atk / 64 Def / 72 SpA / 72 SpD / 96 Spe
- Fire Blast
- Fire Punch
- Crunch
- Aerial Ace

Gengar @ Rocky Helmet
Ability: Levitate
EVs: 104 HP / 36 Atk / 100 Def / 116 SpA / 72 SpD / 80 Spe
- Dark Pulse
- Destiny Bond
- Drain Punch
- Hex
";

#[test]
fn deserialize_team() {
    let team = Team::deserialize(TEAM);
    assert_eq!(team.pokemon.len(), 6);

    assert_eq!(team.pokemon[0].name, "Dragonite");
    assert_eq!(team.pokemon[1].name, "Zoroark");
    assert_eq!(team.pokemon[2].name, "Chansey");
    assert_eq!(team.pokemon[3].name, "Azumarill");
    assert_eq!(team.pokemon[4].name, "Charizard-Mega-X");
    assert_eq!(team.pokemon[5].name, "Gengar");

    team.display();
}
