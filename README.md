# Adept - Shop Titans Combat Simulator

A rust-based simulator for the game Shop Titans

## Instructions for Populating study_docket.csv

- `Completed`: Whether the study has been completed or not
  - **NOTE:** Set to 'false' or the entry will be skipped
  - If you want to re-run a docket, simply find all 'true' and replace with 'false' before starting program again
- `Identifier`: The unique identifier for the study
  - If not unique, multiple studies may have their results combined!
    - While typically you would not want to combine the results, this may be helpful behavior for things like comparing variance
  - Example: 'Daimyo_Atk_Main_Duo'
- `Description`: Describes the study - not used by the script, for your reference only. Example: 'Optimize Daimyo for ATK with Lord Duo'
- `Type`: Selects the type of study to run in the system. Must match exactly one of the options below:
  - `StaticDuoSkillStudy`: Expects two heroes in the team, and will vary the skills of the **first** hero. Outputs a duo_skillz_results.csv as well as the normal trial_results.csv
- `Skill Name Format`: Selects the format used for skill names in this config. Must match exactly one of the options below:
  - `Abbreviated`: If you are using the 3-letter abbreviation for the skills (defined in data_sheets/skill_abbreviation_map.csv)
  - `FullTierOne`: If you are using the full tier one name for the skills
  - `FullAnyTier`: If you are using the full name of any tier for the skills
    - **NOTE:** It is recommended to use the other options for your own clarity. As with both other formats, the system will look up the tier one name of the skills and scale them to the appropriate tier based on the number of elements on the hero regardless of which tier name you use here.
- `Simulation Qty`: How many simulations to run _per skillset_. 1 minimum, 50000 maximum
  - **NOTE::** Runtime can quickly become excessive with higher sim quantities, especially if not excluding many skills and/or not setting many static preset skills. In testing, 1000 is a good minimum that maintains consistency, and 10000 can be appropriate for refining the upper end skillsets. It is not recommended to use 25000+ for your initial round of testing.
- `Runoff Scoring Threshold`: **Unimplemented**
- `Team Hero Identifiers`: The semi-colon-separated list of hero identifiers to include in the team
  - **NOTE:** See the notes on the `Type` column above. Some studies expect hero identifiers in a specific order and will otherwise give unexpected results
  - Whitespace around each list item is trimmed
  - Hero identifiers are looked up from entries in input/hero_builder.csv and must match exactly
  - Example: "Daimyo-Atk_Test_Main; Lord_Control"
- `Team Booster`: What booster to apply to the team. Must exactly match from the options below:
  - `None`
  - `Power Booster`
  - `Super Power Booster`
  - `Mega Power Booster`
- `Static Preset Skills`: The semi-colon-separated list of skill names following your specified `Skill Name Format` to be used as static (unchanging) for the hero being varied upon. Up to 4 may be specified, one for each skill slot.
  - **NOTE:** May be left empty to have no static skills, varying all 4 skill slots
  - **NOTE:** Each non-static skillslot exponentially increases the number of variations that must be trialed. Use caution when leaving this blank if you are also using high simulation quantity and not excluding any skills
  - Whitespace is trimmed around each item
  - Example Using FullTierOne Format: "Warlord; All Natural;Whirlwind Attack ; Power Attack"
  - Example Using Abbreviated Format: "War;All;Whi;Pow"
- `Dungeon Specifications`: The pipe-separated ('|') list of dungeon-specs to include. Each dungeon-spec is defined as follows (colon-separated):
  - **NOTE:** Currently multiple dungeons are not supported - only the first dungeon in the list is used. This is the case for static*duo_skill_study, which is the only implemented study type. In the future, this and other study types \_may* support automatic retrialing of the top X% of skillsets on a harder dungeon from this list. For now, just use the single-dungeon configuration.
  - `[Dungeon Identifier]:[Difficulty]:[Miniboss Setting]`
    - `Dungeon Identifier`: Dungeon identifiers must match exactly one specified in input/dungeons.yaml. Included by default are the following Dungeons:
      - `Howling Woods`
      - `Aurora Caves`
      - `Whispering Bog`
      - `Barren Wastes`
      - `Sun God's Tomb`
      - `Chronos Ruins`
      - `Haunted Castle`
      - `Sunken Temple`
      - `Bleakspire Peak`
      - `Cinderlake Volcano`
      - `Void Dimension`
    - `Difficulty`: Difficulty must match exactly one specified below:
      - `Easy`
      - `Medium`
      - `Hard`
      - `Extreme`
      - `Boss Easy`
      - `Boss Medium`
      - `Boss Hard`
      - `Boss Extreme`
    - `Miniboss Setting`: Must be one of the following (ignored for Boss difficulties)
      - `No Minibosses`
      - `Only Minibosses`
      - `Random Minibosses`: 50% chance of spawning a miniboss. Included for legacy reasons, I generally recommend running two simulations one with only minibosses and one with no minibosses
  - If you only wish to use one dungeon, follow this example: 'Bleakspire Peak: Boss Hard : No Minibosses'
  - For multiple dungeons, follow this example: 'Bleakspire Peak:Boss Hard:No Minibosses|Bleakspire Peak:Hard :Only Minibosses'
- `Automatic Rank Difficulty Optimization`: Unimplemented
- `Excluded Skills`: The semi-colon-separated list of skill names following your specified `Skill Name Format` that will be excluded from the list of skills that are to be varied upon.
  - Commonly used to remove skills with no combat effect (like +XP) to speed up processing
  - Can be left blank to exclude no skills

## Update Notes:

1. Quintessence Purity is missing Water affinity, must check/update if downloading new blueprints sheet.

## MVP

### Requirements:

1. [DONE] The set of all valid skills for a single hero (with or without a static supporting team) can be simulated
2. Simulation output is aggregated in a useful format by Study/Trial
3. Skillsets are ranked and results output with emphasis on human-readability
4. Tool can be easily used by anyone familiar with the command line on both linux and windows
5. Default data is complete and included with the distribution (e.g. data sheets & input files)
6. Processing an entire set of skills should not take an unreasonable amount of time

### Roadmap:

1. Generate Skill Combinations
2. Auto-Generate Trials and Run Simulations from Skill Combos <-- We are here
3. Aggregation Functions, Ranking, and Better Trial-Based Logging
4. Ensure Data is Ready for Distribution (e.g. hero classes are all created)
5. CLI Functionality and Build for Release
6. Testing with Peetee

### Checkpoint Items:

1. Logic for filtering out incompatible skill combos as well as skill combos with more than one bronze or gold weapon skill (e.g. adept, marksman, etc.) - you can have one of each but not two of either
2. Logic for passing multiple equipment sets for each hero in a team? Or just for one hero?

```md
So do you think this methodology would be valid:

1. When starting the simulator you give a list of equipment sets. The simulator uses the weapon in the 1st set (or maybe I add a column called 'isPrimarySet') for the entire proceeding round(s) of simulations.
2. For the final round of simulations (50k+), the simulator will take the builds above 70 score, and if they have a weapon skill, generate additional combos using the weapons from the other skill sets.
   [11:52 AM]

- which would obviously replace the skill(s) with the appropriate alternative i.e. wand master to sword master (edited)
  [11:53 AM]
  So you'd still end up testing all of those skills, but you don't waste time generating 4x the total simulations for combos that are dead ends
  [11:53 AM]
  Or is there something I'm overlooking there
```

3. Maybe develop an easy prefilter and postfilter system, eh? Something that could filter out all specified skills before study creation, and then another that is used during trials to filter out combos conditionally? Also need to save the specific filter sets and/or the skills that were actually used with the results for later display on a webpage. Perhaps postfilter for restricting skills by SLOT so more easy to build cores for duos?
4. System for generating 'Rounds' of Trials, such that rounds vary by the number of simulations per trial and by how the combinations are created (e.g. initially source deterministically from the csv but later rounds are based on the best combos of the previous round - perhaps in batches for resuming)
5. Make sure hero_builder.csv identifier has uniqueness enforced

Note: These might be specific to SingleHeroSkillStudy so may not be problematic afterall 6. Studies appear to have a preset skills list, rather than using the hero_builder from the team to fill in blank skills 7. Studies appear to take an entire dungeon and not allow customizing the difficulty or minibosses for example. 8. Studies only take one subject hero, problematically

9. A translator to select a dungeon from "Bleakspire Peaks Boss Hard"

## Goals:

1. Rank skill loadouts for individual classes (develop a ranking criteria or relative scoring metric, OR use survival rate but automatically retry anything > 95% at the next highest difficulty until the entire set of builds is ordered correctly and unambiguously)

2. Compare classes relative to each other - both at peak performance and with 'good enough' skills

3. Team combinations - what are the optimal classes to group with each other

4. Eventually a website where you can input the skills for a given hero and it will tell you the rank relative to the maximum for that class based on our simulation data. Could also suggest which skills should be replaced and what your best replacement options are

## Notes/TODO:

### In-Progress:

- Automatic Trial Generation (Study Struct)
- - [DONE] Generate deterministic skillsets for each combination
- - Build machinery for automatically running trials
- - - Trials must be saved to a CSV, along with a status for whether they were completed or not
- - - Trials must be loadable from that CSV to resume execution if an issue occurred
- - - Trials must log outputs to a CSV (the results of every simulation) - each trial has its own csv in a directory
- - - - If an error occurred or if a flag is set, verbose output including the rounds of each simulation will be saved to its own folder as a csv or ansi? or markdown? file for each trial. To be clear - output will always be logged, but if the flag is set or if an error occurs the file will not be deleted after that simulation is completed
- - Build machinery for trial result aggregation
- - - Studies will read the data from the trial output folder to create a results summary like Peetee's DuoSkillz output, which they will then save as well

- Changes through 11.0.2:
- - Elemental barriers are basically magic, so Rudo hates ‘em. Starting at Champion Skill rank 3, Rudo and his team now deal 50% more damage to elemental barriers for the duration of the effect.
- - The Mage/Archmage classes now receive bonus ATK from staves. To make space for it, the Archmage lost its HP bonus, which was instead added to its base stat progression.
- - New Blueprint Lines: Meals & Desserts
- - Fresh Spirit (10% rest time)
- - Hero classes can now learn the mastery skill of any weapon they can use, including the gold ones (might already be an assumption in the code?)
- -

### Todo:

1. Create logic for automatically setting up trials

2. Import champion info (Separate logic for importing/creating champions as they gain additional bonuses by rank which also determines innate tier)

3. Validate results for extreme and boss encounters, as well as cinderlake normals

4. Validate hero builder for all classes

5. In hero builder script ensure scaling covers all things that need to scale AND that the Hero cant somehow bypass scaling AND that things like eva and crit chance come from HeroClass AND throughough elementType should be converted from string to ElementType at least once to validate AND write a method to validate skills later

6. Insert all the remaining hero classes into yaml... red is DONE, arch druid is DONE

7. Automatic translation between skill tiers / names (Input/Output options for both 'Cleave T4' and 'WhateverTheT4NameIs' and for output only just 'Cleave' where appropriate for builds and such)

8. When importing skills, ensure skill_tier == 1 if tier_1_name == skill_name, that there are only 4 entries per tier_1_name variant, etc.

9. Ensure class bonuses are applied correctly in sim: Chieftain threat -> attack mod, mercenary + effect from champ skills, lord protect, samurai/daimyo auto evade & first hit crit ignoring element barriers, berserker/jarl bonuses at hp thresholds, trickster polonia stuff, conq consecutive crits, wanderer max eva, ninja/sensei bonuses till damaged and recovery, dancer/acrobat guaranteed crits, cleric autosurvive, spellblade/knight use any element but 30% power against barriers, geo/astramancer attack per point in any element

10. Script should be able to handle testing variants of heroes as well (e.g. with different equips) with the same easy configuration steps

11. Ideally some kind of resume after crash functionality could be nice particularly for larger trialsets

12. Need a way to lock skills or gear from being permuted - so for example can test bow skill line vs wand skill line

13. Additionally, auto-combinations should exclude skills that dont match the equipped gear - if there is no dagger equipped then dagger master should be skipped

14. Perhaps it would be good to have a way to restrict what skills/equipment is available - for example restricting to T10 gear and below or removing a set of X skills for some reason. Goes hand in hand with locking stuff I think

15. When it comes to ranking builds, think of a way to weight the order of the skills in the build if there are empty slots remaining (because if epics are in slots 1-2 that is better for rolling than in 2-3 for example)

### Notes:

- - Armadillo is 15 per 1, lizard is 3 per 1, shark is 20 per 1, dinosaur is 25 per 1, mundras ins 1 per 1

- Some way to manually specify which if any miniboss should be spawned

- Optimizations

- make sure only one champion per team
