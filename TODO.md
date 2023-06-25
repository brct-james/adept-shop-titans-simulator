# New Todo List

## Note

1. I belive the skill filter now removes manually excluded skills, preset skills, skills incompatible with preset, and skills incompatible with team[0]'s class. It would be worth double checking this is working as intended once a good test case is identified

## Short-Term

1. Design system for swapping out weapons/shield

- Easiest to list alternative weapons/shield-slot-items which the system will pick based on the mastery skills in the skillset/build
- - The most allowed types I see are 4, so lets give it base + 4 more to be safe. That is both for the weapon slot and shield slot so 2 base + 8 alternates
- For all skillsets without mastery, use 'base hero' as defined
- Verify the mastery skills have incompatible skills populated
- Verify that the skillsets meant for testing with the alternative weapons are populated into the valid skills list correctly

2. System can resume (save out study index to study docket)
3. After this is working, run a full skillset analysis (will need to add the alternative weapons/shield for the relevant heroes in hero builder) at 50k
4. Build out the google sheets to include hero builder and study docket in same sheet. Dropdowns for all weapons, alt weapons, skills, etc. and dropdowns for heroes, dungeons, etc. in study docket.

## Mid-Term

1. Instead of panicking, should log error at the docket level and continue iteration
2. Calculate score internally and use that to run additional simulations if the skillset/build looks to be performing well enough (10k-25k-50k)

- Ex: Run 10k sims, then if Score > X run 15k more, then if Score > Y run 25k more (note WarSwoBluPFr for Daimyo - need extra logic to catch edge)
- Consider issues with minobservedrounds reference table for non-duos and other dungeons (could maybe adapt this to be generated for each dungeon if we can programmatically create the logarithmic association)

3. Set up static host for output
4. Get most recent bps from the official sheet
5. Queue up backlog

## Long-Term

1. Address distribution issues (make program usable outside of the commandline, package executable)
2. Website with comprehensive skill analysis - ranked duo results, interactive hero suggestions, API that accepts posts for queueing jobs on the rust backend and hosts the output
3. Integration with doweb - API access to get ranked skills given a certain hero? hosting duo results and/or an API that accepts posts for queueing jobs on the rust backend and hosts the output for ad-hoc analysis from users
