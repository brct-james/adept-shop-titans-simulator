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
- - Build machinery for trial result aggregation

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