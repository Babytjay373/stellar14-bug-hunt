# Rounds

Today runs as a team game. Every round unlocks 2 or 3 issues for **all teams at once**. The countdown and the scoreboard are on the projector.

All 16 issues are visible on GitHub from the start. Only work on the issues of the round that is live. A PR for an issue from a later round does not score.

| Round | Theme | Issues unlocked | Time |
| --- | --- | --- | --- |
| Demo | Host fixes it live on the projector | #8 | |
| Round 1 | Warm up | #3 (easy), #6 (easy), #7 (easy) | 15 min |
| Round 2 | Easy and medium | #1 (easy), #4 (easy), #10 (medium) | 18 min |
| Round 3 | Medium | #2 (medium), #5 (medium), #11 (medium) | 18 min |
| Round 4 | Medium and stretch, first taste of Soroban | #9 (medium), #12 (stretch), #16 (stretch, Soroban) | 18 min |
| Final round | Soroban | #13, #14, #15 (all stretch, Soroban) | 18 min |

Track B tip: the first Soroban build downloads a lot. Start `cd track-b-soroban && cargo test` on one laptop per team during Round 3 so it is ready for Round 4.

## Points

| What | Points |
| --- | --- |
| Green PR on an easy issue | 1 |
| Green PR on a medium issue | 2 |
| Green PR on a stretch issue | 3 |
| Bonus: first team to get a green PR on that issue | +1 |

* **Green** means the `bug-NN` check for that issue passed on your PR.
* One PR per team per issue. If a team opens two PRs for the same issue, only the first one counts.
* The PR must be opened before the round timer hits zero. Pushing more commits to it during the next scoreboard break is fine, but the first green bonus goes by the time the check went green.
* Only the first clean PR on each issue is merged live on the projector. The other teams still keep their points from green CI, and we close their PRs with a thank you after the round.
* Do not edit tests, `types.rs` or `Cargo.toml`. A PR that does gets zero for that issue.

## Max points per round

| Round | Max for one team (including first green bonus) |
| --- | --- |
| Round 1 | 6 |
| Round 2 | 7 |
| Round 3 | 9 |
| Round 4 | 11 |
| Final round | 12 |
