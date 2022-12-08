# shop-titans-simulator
A rust-based simulator for the game Shop Titans

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
* which would obviously replace the skill(s) with the appropriate alternative i.e. wand master to sword master (edited)
[11:53 AM]
So you'd still end up testing all of those skills, but you don't waste time generating 4x the total simulations for combos that are dead ends
[11:53 AM]
Or is there something I'm overlooking there
```

3. Maybe develop an easy prefilter and postfilter system, eh? Something that could filter out all specified skills before study creation, and then another that is used during trials to filter out combos conditionally? Also need to save the specific filter sets and/or the skills that were actually used with the results for later display on a webpage. Perhaps postfilter for restricting skills by SLOT so more easy to build cores for duos?
4. System for generating 'Rounds' of Trials, such that rounds vary by the number of simulations per trial and by how the combinations are created (e.g. initially source deterministically from the csv but later rounds are based on the best combos of the previous round - perhaps in batches for resuming)
5. Make sure hero_builder.csv identifier has uniqueness enforced

Note: These might be specific to SingleHeroSkillStudy so may not be problematic afterall
6. Studies appear to have a preset skills list, rather than using the hero_builder from the team to fill in blank skills
7. Studies appear to take an entire dungeon and not allow customizing the difficulty or minibosses for example.
8. Studies only take one subject hero, problematically

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

10. Ensure class bonuses are applied correctly in sim: Chieftain threat -> attack mod, mercenary + effect from champ skills, lord protect, samurai/daimyo auto evade & first hit crit ignoring element barriers, berserker/jarl bonuses at hp thresholds, trickster polonia stuff, conq consecutive crits, wanderer max eva, ninja/sensei bonuses till damaged and recovery, dancer/acrobat guaranteed crits, cleric autosurvive, spellblade/knight use any element but 30% power against barriers, geo/astramancer attack per point in any element

11. Script should be able to handle testing variants of heroes as well (e.g. with different equips) with the same easy configuration steps

12. Ideally some kind of resume after crash functionality could be nice particularly for larger trialsets

13. Need a way to lock skills or gear from being permuted - so for example can test bow skill line vs wand skill line

14. Additionally, auto-combinations should exclude skills that dont match the equipped gear - if there is no dagger equipped then dagger master should be skipped

15. Perhaps it would be good to have a way to restrict what skills/equipment is available - for example restricting to T10 gear and below or removing a set of X skills for some reason. Goes hand in hand with locking stuff I think

16. When it comes to ranking builds, think of a way to weight the order of the skills in the build if there are empty slots remaining (because if epics are in slots 1-2 that is better for rolling than in 2-3 for example)

### Notes:

- - Armadillo is 15 per 1, lizard is 3 per 1, shark is 20 per 1, dinosaur is 25 per 1, mundras ins 1 per 1

- Some way to manually specify which if any miniboss should be spawned

- Optimizations

- make sure only one champion per team