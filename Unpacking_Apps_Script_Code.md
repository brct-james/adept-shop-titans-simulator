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
13. Continue analysis from line 176 onward