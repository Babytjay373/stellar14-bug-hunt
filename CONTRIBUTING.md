# Contributing: the team pull request flow

Today you play in a team, and every team works from one **team fork**. This is the same flow from the Git cheat sheet on the screen.

## Once per team (captain)

1. Open https://github.com/nyuiela/stellar14-bug-hunt and click **Fork**. This is your team fork.
2. On the team fork, go to **Settings**, **Collaborators**, **Add people**, and add every teammate by GitHub username.
3. Teammates accept the invite from their email or https://github.com/notifications.

## Once per driver (anyone who will type in a round)

```bash
git clone https://github.com/<captain-username>/stellar14-bug-hunt.git
cd stellar14-bug-hunt
git remote add upstream https://github.com/nyuiela/stellar14-bug-hunt.git
cargo test
```

`origin` is your team fork. `upstream` is the original repo.

## Every issue, every round (the driver types, the team helps)

### 1. Start from a fresh main

```bash
git checkout main
git pull upstream main
```

### 2. Make a branch for one bug

```bash
git checkout -b fix/07-kente
```

Use `fix/<bug-number>-<team>`.

### 3. Reproduce, fix, test

Run the command from the bug list in the README, read the error out loud, fix the code, and run it again until you see `test result: ok`.

```bash
cargo test -p wallet-basics bug07
```

### 4. Check what changed

```bash
git status
git diff
```

You should see one file changed: the file for your bug.

### 5. Add and commit

```bash
git add track-a-basics/src/bugs/bug07_paging.rs
git commit -m "Fix #7: start pages at the first item"
```

### 6. Push the branch to the team fork

```bash
git push origin fix/07-kente
```

### 7. Open the pull request

GitHub shows a **Compare and pull request** button after you push. Click it, then:

* base repository: `nyuiela/stellar14-bug-hunt`, base: `main`
* title: `Team Kente: Fix #7`
* description: `Fixes #7` and one sentence on what was wrong

### 8. Watch CI

The `bug-07` check runs on your PR. Green scores points. Red means read the log, fix, and push again to the same branch. The PR updates by itself.

### 9. Review and merge

Between rounds the host reviews the first clean PR for each issue on the projector and merges it. Your team still keeps its points if another team got merged.

### 10. Next issue

Go back to step 1. Always start a new fix from a fresh `main`.

## Driver rotation

Pick a new driver every round, so everyone in the team types and pushes at least once today. If a driver's laptop is not set up, they can drive on a teammate's laptop and commit as themselves:

```bash
git -c user.name="Your Name" -c user.email="you@example.com" commit -m "Fix #7: start pages at the first item"
```

Use the email on your GitHub account so the commit shows up on your profile.

## Content Crew: your first PR in the browser

No install needed. Open the file on GitHub, click the pencil icon, make your change, then **Commit changes**, **Propose changes**, **Create pull request**. GitHub makes the fork and branch for you. Title it `Crew: <what you did>`. See [CREW.md](CREW.md).

## Good habits

* One bug per branch and per PR. Small PRs get merged fast.
* Do not edit tests to make them pass.
* Read the first error first. The rest often disappear once it is fixed.
* If you used AI to help, still run the test yourself and make sure the driver can explain the fix in one sentence.
