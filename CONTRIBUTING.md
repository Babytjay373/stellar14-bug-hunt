# Contributing: the pull request flow

This is the same flow from the Git cheat sheet on the screen. Do it once today and it will feel normal forever.

## 1. Fork

Open https://github.com/nyuiela/stellar14-bug-hunt and click **Fork**. You now have your own copy on GitHub.

## 2. Clone your fork

```bash
git clone https://github.com/<your-username>/stellar14-bug-hunt.git
cd stellar14-bug-hunt
```

## 3. Connect to the original repo

```bash
git remote add upstream https://github.com/nyuiela/stellar14-bug-hunt.git
```

`origin` is your fork. `upstream` is the original repo.

## 4. Claim a bug

Pick an issue at https://github.com/nyuiela/stellar14-bug-hunt/issues and comment "I'm on this".

## 5. Make a branch for that one bug

```bash
git checkout -b fix/07-paging
```

Use `fix/<bug-number>-<short-name>`.

## 6. Reproduce, fix, test

Run the command from the bug list in the README, read the error, fix the code, and run it again until you see `test result: ok`.

```bash
cargo test -p wallet-basics bug07
```

## 7. Check what changed

```bash
git status
git diff
```

You should see one file changed: the file for your bug.

## 8. Add and commit

```bash
git add track-a-basics/src/bugs/bug07_paging.rs
git commit -m "Fix #7: start pages at the first item"
```

## 9. Push your branch to your fork

```bash
git push origin fix/07-paging
```

## 10. Open the pull request

GitHub shows a **Compare & pull request** button after you push. Click it, then:

* base repository: `nyuiela/stellar14-bug-hunt`, base: `main`
* title: `Fix #7: start pages at the first item`
* in the description, write `Fixes #7` and one sentence on what was wrong

## 11. Review

Watch the checks. The `bug-07` check should turn green. A reviewer may leave a comment. Push more commits to the same branch to answer it and the PR updates by itself.

## 12. Merge

We merge on the projector. Clap for yourself.

## 13. Stay in sync before your next fix

```bash
git checkout main
git pull upstream main
git push origin main
git checkout -b fix/10-fees
```

Always start a new fix from a fresh `main`.

## Good habits

* One bug per branch and per PR. Small PRs get merged fast.
* Do not edit tests to make them pass.
* Read the first error first. The rest often disappear once it is fixed.
* If you used AI to help, still run the test yourself and make sure you can explain the fix in one sentence.
