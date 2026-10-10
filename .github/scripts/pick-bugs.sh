#!/usr/bin/env bash
# Prints a JSON list of the bug checks to run, for example ["07"] or ["13","14","15","16"].
# Usage: pick-bugs.sh all
#        pick-bugs.sh <base-sha>      (compares base-sha with HEAD)
set -euo pipefail
ALL_A="01 02 03 04 05 06 07 08 09 10 11 12"
ALL_B="13 14 15 16"
bugs=""
if [ "${1:-all}" = "all" ]; then
  bugs="$ALL_A $ALL_B"
else
  for f in $(git diff --name-only "$1" HEAD); do
    case "$f" in
      track-a-basics/src/bugs/bug[0-9][0-9]_*.rs) bugs="$bugs $(basename "$f" | cut -c4-5)" ;;
      track-b-soroban/src/lib.rs) bugs="$bugs 13" ;;
      track-b-soroban/src/storage.rs) bugs="$bugs 14" ;;
      track-b-soroban/src/checks.rs) bugs="$bugs 15" ;;
      track-b-soroban/src/events.rs) bugs="$bugs 16" ;;
      track-b-soroban/*) bugs="$bugs $ALL_B" ;;
      track-a-basics/*|Cargo.toml|Cargo.lock) bugs="$bugs $ALL_A" ;;
      .github/workflows/ci.yml|.github/scripts/*) bugs="$bugs $ALL_A $ALL_B" ;;
    esac
  done
fi
printf '%s\n' $bugs | sed '/^$/d' | sort -u | jq -R . | jq -sc .
