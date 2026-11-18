## What and why

<!-- What does this change do, and what problem does it solve? -->

## How it was verified

<!-- Tick what you ran. CI runs the matching checks when registry/, site/ or cli/ change. -->

- [ ] `python3 registry/validate.py` (catalogue changes)
- [ ] `python3 site/scripts/generate-catalogue.py` leaves no diff in `site/dist` (catalogue or listing changes)
- [ ] `python3 site/scripts/check-site.py` (site changes)
- [ ] In `cli/`: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings` and `cargo test` (installer changes)

## Notes

<!-- Anything reviewers should know: trade-offs, follow-ups, screenshots. -->
<!-- Detection rules: say which app version you checked before setting "verified": true. -->
