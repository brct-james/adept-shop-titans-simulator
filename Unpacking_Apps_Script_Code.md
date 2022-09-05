# Unpacking Apps Script Code

## Main (COdigo.gs)

onEdit if on Quest Sim, run simV2

else:

## simV2.gs

1. Get class, innateskilltier, hp, atk, def, threat, critChance, critMult, evasion, element, elementtype, armadillo, lizard, shark, dinosaur, mundra, atkMod, defMod for each hero in team
2. Get zone, hpval, damage, cap, aoedmgbase, aoechance, islcog, isboss, isextreme, miniboss, barriertype, barrierhpval, extremecritbonus? for encounter mob
3. Set evasion = -1, hpmod dmgmod critchancemod = 1, critchance = .1, barriermod = .2, aoedmg = aoedmgbase / dmg, aoechance = aoechance/100, numfighters numrogues numspellcasters = 0 for encounter mob
4. switch MiniBoss:
```
case "Agile":
    evasion = 0.4
case "Dire":
    hpmod = 1.5
    critchancemod = 3
case "Huge":
    hpmod = 2
case "Legendary":
    hpmod = 1.5
    dmgmod = 1.25
    critchancemod = 1.5
    evasion = 0.1
```
5.
```
  Mob_HP_val = Math.round(Mob_HP_val*Mob_HP_mod);
  Mob_Damage = Math.round(Mob_Damage*Mob_Damage_mod);
```
6. if not lcog: ignore 5th hero
7. if zone == "Barren Boss Hard", ignore 4th hero
8. if not boss, ignore mundra (set to 0)
9. for each hero, count fighters, rogues, spellcasters by class name
10. set:
```
  var Champion               = "None";
  var Champion_Tier          = 0;
  var Champion_Attack_bonus  = 0.0;
  var Champion_Defense_bonus = 0.0;
  var Rudo_bonus             = 0.0;
```
11. If champion name in class list, set that hero as champion:
```
  for (var i = 0; i < Hero_Class.length; i++) {
    if (Hero_HP_val[i] > 0 && (Hero_Class[i] == "Argon" || Hero_Class[i] == "Ashley" || Hero_Class[i] == "Donovan" || Hero_Class[i] == "Hemma" || Hero_Class[i] == "Lilu" || Hero_Class[i] == "Polonia" || Hero_Class[i] == "Rudo" || Hero_Class[i] == "Sia" || Hero_Class[i] == "Yami"))  {
      Champion = Hero_Class[i];
      Champion_Tier = Hero_Tier[i];
    }
  }
```
12. Normalize percents:
```
  for (var i = 0; i < Hero_Class.length; i++) {
    
    if(Mob_Is_Extreme=="Yes") 
      Hero_Evasion[i] = Hero_Evasion[i]-20;
    Extreme_Crit_bonus.push(1.0);
    
    Hero_Crit_chance[i] = Hero_Crit_chance[i]/100.0;
    Hero_Evasion[i] = Hero_Evasion[i]/100.0;
    
    Hero_Attack_mod[i] = 1.0 + Hero_Attack_mod[i] / 100;
    Hero_Attack[i]= Hero_Attack[i]/Hero_Attack_mod[i];
    Hero_Defense_mod[i] = 1.0 + Hero_Defense_mod[i] / 100.0;
    Hero_Defense[i] = Hero_Defense[i]/Hero_Defense_mod[i];
    Hero_Defense[i] = Hero_Defense[i] * (Hero_Defense_mod[i]+0.2*Hero_Mundra[i]);
  }
```
13. Line 176 onward: Heroes also have:
```
Survive Chance
Guaranteed Crit
Evade
Lost Innate
Consecutive Crit Bonus
Berserker Stage
Berserker Level
Jarl_hp1
Jarl_hp2
Jarl_hp3
Ninja_bonus
Ninja_evasion
Eva_cap
Hemma_bonus
```
14. Line 226 onward: Setup Simulation Run
```
//Hero chance to get targeted
  var Hero1_Target;
  var Hero2_Target;
  var Hero3_Target;
  var Hero4_Target;
  var Target_Tot;
  var target_chance;
  var Num_Heroes = 0;
  var Heroes_Alive = 0;
  var Update_Target = true;
  var Round = 0;
  var Shark_active;
  var Dinosaur_active;
  var Who_Hemma;
  var Lord_present;
  var Lord_hero;
  
  var Times_quest_won = 0;
  
  var Polonia_loot = 0;
  var Polonia_loot_tot = 0;
  var Polonia_loot_cap = 20;
  var Polonia_loot_cap_hit = 0;
  var loot_chance;
  var count_loot = false;
  var Num_tricksters = 0;
   
  var Times_Hero_survived = [];
  var Hero_Damage_fight = [];
  var Hero_Damage_Dealt_avg = [];
  var Hero_Damage_Dealt_max = [];
  var Hero_Damage_Dealt_min = [];
  var Hero_HP_Remaining_avg = [];
  var Hero_HP_Remaining_max = [];
  var Hero_HP_Remaining_min = [];
```
15. Line 265+: For each hero initialize these storage lists
```
  for (var i = 0; i < Hero_Class.length; i++) {
    Times_Hero_survived.push(0);
    Hero_Damage_fight.push(0);
    Hero_Damage_Dealt_avg.push(0);
    Hero_Damage_Dealt_max.push(0);
    Hero_Damage_Dealt_min.push(1000000000);
    Hero_HP_Remaining_avg.push(0);
    Hero_HP_Remaining_max.push(0);
    Hero_HP_Remaining_min.push(100000);   
  }
```
16. track rounds avg/max/min
```
  var Rounds_avg = 0;
  var Rounds_max = 0;
  var Rounds_min = 1000;
```
17. Get booster info and set up other variables:
```
  var Booster = SpreadsheetApp.getActiveSheet().getRange(8, 2).getValue();
  var Booster_Attack_bonus  = 0.0;
  var Booster_Defense_bonus = 0.0;
  var Hemma_mult = 0;
  var target = 0;
```
18. Check for a lord alive amongst team:
```
  for (var i = 0; i < Hero_Class.length; i++) {
    if(Hero_Class[i]=="Lord" && Hero_HP_val[i] > 0)  {
      Lord_present = true;
      Lord_hero = i;
      break;
    }
  }
```
19. Line 300+: Calculate champion bonuses
20. line 346 set Who_Hemma to index of hemma in heroes list
21. line 445+: Set vars for each simulation and setup sim conditions

### Notes/TODO:
- I think the spreadsheet doesn't use spirit qty but rather the actual value of the spirits (e.g. lizard gives +3hp so for qty 1 the spreadsheet would actually be 3 lizard...) Double check and then modify as appropriate

- - Armadillo is 15 per 1, lizard is 3 per 1, shark is 20 per 1, dinosaur is 25 per 1, mundras ins 1 per 1

- Must implement SimResult and aggregation in trials

- IMplement logger for round-by-round actions for debugging (with some kind of toggle that can be passed for saving or not, or only saving if fail round)

- Some way to manually specify which if any miniboss should be spawned

- Optimizations

- Round f64s to 2 decimals for output in csvs

- make sure only one champion per team

- pull statistics and dungeon info from spreadsheet rather than manually entered

- team will have its own identifier and use the hero identifiers to select from the heroes csv, team is its own csv
- trial will then have another csv that selects dungeons from their own csv by identifier

- redo readme

- tammara is giving HUGE crits - like 7.7m damage, I am assuming that is because I input her stats wrong, check once hero builder is done


### Todo:

1. Create logic for automatically setting up trials

2. Create hero builder script

3. ~~Pull gear directly from data spreadsheet~~

4. Either pull skills from data spreadsheet and parse, or manually create skill sheet

5. Validate results for extreme and boss encounters, as well as cinderlake normals

6. In hero builder script ensure scaling covers all things that need to scale AND that the Hero cant somehow bypass scaling AND that things like eva and crit chance come from HeroClass AND throughough elementType should be converted from string to ElementType at least once to validate AND write a method to validate skills later