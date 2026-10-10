# Stellar 14 Bug Hunt

Welcome to the Stellar 14 build days Meetup, Saturday 10 October 2026, at Payaza Africa, Labone (Ndabaningi Sithole Rd), Accra.

This repo is broken on purpose. It is a small Stellar flavoured wallet written in Rust, plus a tiny Soroban tip jar contract, and we planted 16 bugs in it. Your job today is to fork it, find a bug, fix it on its own branch, and open a pull request. We review and merge the pull requests live on the projector, so for many of you this will be your first open source contribution.

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

5. Soroban tests run with plain `cargo test`, so the Rust setup above is enough to work on Track B. The first build downloads `soroban-sdk` and takes a few minutes, so start it early.
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

Each bug has a matching GitHub issue with the same number. Claim a bug by commenting on its issue before you start.

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

Bug 8 is the one we fix together on the projector as the live demo, so watch that one first.

## Rules of the hunt

1. **One bug per branch, one bug per pull request.**
2. **Branch name:** `fix/<bug-number>-<short-name>`, for example `fix/07-paging`.
3. **PR title:** `Fix #N: <what you fixed>`, for example `Fix #7: start pages at the first item`. Put `Fixes #N` in the PR description so the issue closes when we merge.
4. **Claim before you code.** Comment "I'm on this" on the issue. If a bug is already claimed, team up with that person, or pick another one.
5. **Only touch the file for your bug.** Do not edit `types.rs`, `Cargo.toml`, or the tests.
6. **Lots of people, 16 bugs.** You are welcome to fix any bug in your own fork to practise. We merge the first clean PR for each bug, and we review the others out loud so everyone learns from them.
7. Stuck for more than 10 minutes? Raise your hand. Reading the compiler error out loud together is half the fix.

## Bonus PRs

Fixed your bug, or everything is claimed? These count on the leaderboard too:

* Add one more unit test to any bug file that checks an edge case.
* Improve a doc comment so the next person understands the function faster.
* Add a short example in a doc comment that shows how to call a function.

Bonus PR titles start with `Docs:` or `Test:` instead of `Fix #N:`.

## CI

Every pull request runs GitHub Actions. There is one check per bug, named `bug-01` up to `bug-16`. `main` starts with every bug check red. Your PR is ready when the check for your bug turns green.

## Credits

Stellar 14 build days, by Stellar Ghana and The Foundry. Hosted by Kaleel ([@nyuiela](https://github.com/nyuiela)).

Useful links:

* The Rust book: https://doc.rust-lang.org/book/
* Stellar developer docs: https://developers.stellar.org
* Soroban SDK docs: https://docs.rs/soroban-sdk

MIT licensed. See [LICENSE](LICENSE).
