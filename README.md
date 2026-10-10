# Stellar 14 Bug Hunt

Welcome to the Stellar 14 build days Meetup, Saturday 10 October 2026, at Payaza Africa, Labone (Ndabaningi Sithole Rd), Accra.

This repo is broken on purpose. It is a small Stellar flavoured wallet written in Rust, plus a tiny Soroban tip jar contract, and we planted 16 bugs in it.

Today is a team game. Coders play in teams of 4 or 5. Every round unlocks 2 or 3 bugs for all teams at once, a countdown runs on the projector, and every green pull request scores points. We review and merge the winning pull requests live on the projector, so for many of you this will be your first open source contribution.

Not coding? Join the **Content Crew**. See [CREW.md](CREW.md) for the roles, your own docs only first PR, and the content competition.

* Rounds, points and which bugs unlock when: [ROUNDS.md](ROUNDS.md)
* Team PR flow: [CONTRIBUTING.md](CONTRIBUTING.md)
* Hashtag for the day: `#Stellar14Accra`

Everything here uses what we covered this week: Git and pull requests, structs, enums and match, traits, Option and Result, ownership and borrowing, and a first look at Soroban.

Event page: https://luma.com/ptk31d3s

## What is inside

| Folder | What it is | Who it is for |
| --- | --- | --- |
| `track-a-basics/` | Track A, Rust basics. A wallet and payments library. Bugs 1 to 12. | Everyone, especially if this is your first Rust or first PR |
| `track-b-soroban/` | Track B, Soroban. A tip jar smart contract. Bugs 13 to 16. | Stretch track for people who are comfortable with Track A |

`track-a-basics/src/types.rs` is correct. Every bug lives in its own file, so two people fixing two different bugs never touch the same file.

## Setup

You need four things. If any of them is missing, come to the help desk at the front and we will set it up with you.

1. **Rust**, installed with rustup: https://rustup.rs
   On macOS or Linux run `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`. On Windows download `rustup-init.exe` from the same page.
   Check it with `cargo --version`.
2. **VS Code** with the **rust-analyzer** extension: https://code.visualstudio.com
3. **Git**: https://git-scm.com/downloads. Check it with `git --version`.
4. **A GitHub account** you can sign in to: https://github.com/signup

Track B only:

5. Track B uses soroban-sdk 28, which needs Rust 1.91 or newer. Run `rustup update` if your Rust is older. Soroban tests run with plain `cargo test`, so the Rust setup above is enough to work on Track B. The first build downloads `soroban-sdk` and takes a few minutes, so start it early.
6. Optional: the Stellar CLI, if you also want to build the contract to Wasm with `stellar contract build`: https://developers.stellar.org/docs/tools/cli/install-cli

## Get the code

1. Click **Fork** at the top right of https://github.com/nyuiela/stellar14-bug-hunt
2. Clone your fork and step into it:

```bash
git clone https://github.com/<your-username>/stellar14-bug-hunt.git
cd stellar14-bug-hunt
git remote add upstream https://github.com/nyuiela/stellar14-bug-hunt.git
```

3. Run the tests to see what is broken:

```bash
cargo test
```

You will see red. That is the point. The full step by step PR flow is in [CONTRIBUTING.md](CONTRIBUTING.md).

## How the bugs are split

* **Bugs 1 to 6 are compile errors.** Each one sits behind its own cargo feature, so one compile error never blocks the rest of the room. Plain `cargo test` skips them. To work on one, switch its feature on:
  `cargo test -p wallet-basics --features bug01 bug01`
* **Bugs 7 to 12 compile, but their tests fail.** Plain `cargo test` shows them. To run just one:
  `cargo test -p wallet-basics bug07`
* **Bugs 13 to 16 are in the Soroban contract.** Run them from the Track B folder:
  `cd track-b-soroban && cargo test bug13`

A bug is fixed when its command prints `test result: ok`. Do not change or delete a test to make it pass. Fix the code the test is checking.

## Bug list

Each bug has a matching GitHub issue with the same number. All of them are visible, but only the bugs of the live round score. See [ROUNDS.md](ROUNDS.md).

| # | Issue title | Difficulty | File | Reproduce |
| --- | --- | --- | --- | --- |
| 1 | Archiving a payment will not compile (E0382) | easy | `track-a-basics/src/bugs/bug01_move.rs` | `cargo test -p wallet-basics --features bug01 bug01` |
| 2 | Treasury report fights the borrow checker (E0502) | medium | `track-a-basics/src/bugs/bug02_borrow.rs` | `cargo test -p wallet-basics --features bug02 bug02` |
| 3 | Status label forgets a transaction status (E0004) | easy | `track-a-basics/src/bugs/bug03_match.rs` | `cargo test -p wallet-basics --features bug03 bug03` |
| 4 | Stroops to XLM conversion has the wrong type (E0308) | easy | `track-a-basics/src/bugs/bug04_types.rs` | `cargo test -p wallet-basics --features bug04 bug04` |
| 5 | Asset list cannot describe assets (E0277) | medium | `track-a-basics/src/bugs/bug05_trait.rs` | `cargo test -p wallet-basics --features bug05 bug05` |
| 6 | Balance lookup reads a field on an Option (E0609) | easy | `track-a-basics/src/bugs/bug06_option.rs` | `cargo test -p wallet-basics --features bug06 bug06` |
| 7 | Payment history pages skip the first item | easy | `track-a-basics/src/bugs/bug07_paging.rs` | `cargo test -p wallet-basics bug07` |
| 8 | Wallet cannot spend its exact balance | easy | `track-a-basics/src/bugs/bug08_compare.rs` | `cargo test -p wallet-basics bug08` |
| 9 | Withdrawing too much crashes the wallet | medium | `track-a-basics/src/bugs/bug09_withdraw.rs` | `cargo test -p wallet-basics bug09` |
| 10 | Unknown assets get a fee instead of None | medium | `track-a-basics/src/bugs/bug10_fees.rs` | `cargo test -p wallet-basics bug10` |
| 11 | Typing text as an amount is accepted as zero | medium | `track-a-basics/src/bugs/bug11_parse.rs` | `cargo test -p wallet-basics bug11` |
| 12 | Decimal XLM amounts convert to the wrong number of stroops | stretch | `track-a-basics/src/bugs/bug12_xlm.rs` | `cargo test -p wallet-basics bug12` |
| 13 | Anyone can tip from someone else's balance | stretch | `track-b-soroban/src/lib.rs` | `cd track-b-soroban && cargo test bug13` |
| 14 | Balance always reads zero after a deposit | stretch | `track-b-soroban/src/storage.rs` | `cd track-b-soroban && cargo test bug14` |
| 15 | Tipping more than you have is allowed | stretch | `track-b-soroban/src/checks.rs` | `cd track-b-soroban && cargo test bug15` |
| 16 | Tip event names the wrong receiver | stretch | `track-b-soroban/src/events.rs` | `cd track-b-soroban && cargo test bug16` |

Bug 8 is the one the host fixes on the projector as the live demo. It does not score.

## Teams

* Teams of 4 or 5, formed in the room by counting off. Every team needs at least one person whose laptop already runs `cargo test`. That person is the **captain**.
* Team names for today: **Kente, Jollof, Black Stars, Lumens, Adinkra, Kelewele** (and **Waakye** if we need a seventh).
* The captain forks the repo once. That fork is the **team fork**. The captain adds every teammate as a collaborator on it (Settings, Collaborators, Add people), and teammates accept the invite from their email or https://github.com/notifications.

## Rules of the game

1. **One PR per team per issue**, opened from the team fork into `nyuiela/stellar14-bug-hunt`.
2. **Branch name:** `fix/<bug-number>-<team>`, for example `fix/07-kente`.
3. **PR title:** `Team <name>: Fix #N`, for example `Team Kente: Fix #7`. Put `Fixes #N` in the description.
4. **One driver per round.** The driver types and pushes from their own laptop. Everyone else reads errors, checks the docs and spots bugs. Rotate the driver every round so everyone types at least once.
5. **Only work on the issues of the live round.** The countdown on the projector is the official clock. PRs must be open before it hits zero.
6. **Green CI scores.** Easy 1, medium 2, stretch 3, plus 1 bonus for the first team to go green on that issue. Full table in [ROUNDS.md](ROUNDS.md).
7. **Only touch the file for your bug.** Never edit tests, `types.rs` or `Cargo.toml`.
8. **No spoilers.** Do not post fixes in issue comments or show them to other teams.
9. Stuck for more than 5 minutes? Raise a hand. Reading the compiler error out loud together is half the fix.

## CI

Every pull request runs GitHub Actions, and only the check for the bug you touched runs, so results come back fast. The checks are named `bug-01` up to `bug-16`. Your PR scores when the check for your bug turns green. `main` starts with every bug red on purpose.

Docs only PRs (Content Crew, glossary, attendees wall) skip the Rust checks and get a quick `docs` check instead.

## Credits

Stellar 14 build days, by Stellar Ghana and The Foundry. Hosted by Kaleel ([@nyuiela](https://github.com/nyuiela)).

Useful links:

* The Rust book: https://doc.rust-lang.org/book/
* Stellar developer docs: https://developers.stellar.org
* Soroban SDK docs: https://docs.rs/soroban-sdk

MIT licensed. See [LICENSE](LICENSE).
