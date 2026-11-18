# Reverse Prompt

> **Navigation**: [Process](README.md)

Written by a Claude line on 2026-11-18.

## T9, the science ledger

**Done.** `crates/ikria-ledger` enumerates every science subject the Phase 0 count
counts, 3,104 of them, each with the identifier the game writes in saves and its Hard
value, and joins them with a save's subjects through `crates/ikria-save`. It reports
possible, collected and remaining science by subject, body, experiment and situation,
lists unmatched and repeated save subjects, and accounts for every point of a save's
science. Differences in a subject's cap or multiplier are reported, never silently
resolved.

**A correction to the record.** The enumeration found the Phase 0 Hard total short by
9 points. Breaking Ground's science definitions split the 34 surface features 12, 11 and
11 across caps of 30, 45 and 60, where the Phase 0 script had 12, 12 and 10. VR-0005,
its script and its data now carry the correct split, with a note that keeps the old
figure, and the Hard total is 217,827. An independent review confirmed the split
against the game's file.

**Review.** The independent review also found an unverified assumption, that surface
features carry no body multiplier, which the ledger now checks against each save
subject's own multiplier. It found 21 location names that are display names, now
flagged as unverified, a residue that kept collected subjects open, and repeated save
records missing from the totals. All are fixed.

**Verified.** The acceptance totals in tests, 37 tests in the crate, and a read-only run
on the old Rookie save, which matches 174 of 177 subjects with no cap or multiplier
differences. The gate exits zero on macOS with every file staged.

**Not verified.** Identifiers away from Kerbin, the 25 unverified location names and the
surface-feature multiplier, until a save holds such subjects.

**For the operator.** Nothing.
