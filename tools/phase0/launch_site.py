"""Recover crewed vessels from the launch sites before a launch, as rule R20 requires.

Campaign rule R20, item 1, says that every launch first recovers any crewed vessel at
the launch site explicitly, refuses to launch if it cannot, and never relies on the
launch call to clear crew. In TP-0002 a launch with `recover=True` removed a pod with
Jebediah aboard and he was marked Missing. This module follows
`tools/sepratron/flightlib.py`, `clear_crewed_launch_site`, and differs in failing
closed, as follows.

- A vessel whose crew or situation cannot be read stops the launch, where the Sepratron
  helper skips it. An unreadable vessel is not known to be empty.
- The check runs in every game scene, where the Sepratron helper returns early outside
  the flight scene. A launch from the space centre clears the pad as well. If the game
  refuses to list vessels in a scene, the launch is refused and the operator recovers
  crew by hand. Whether the list is available outside the flight scene has not been
  verified against the game.
- After recovery the vessels are listed again, and any crewed vessel still there stops
  the launch.

Assumption. kRPC reports no launch-site position, so "at the launch site" is
approximated by every crewed vessel that is before launch or landed on Kerbin, the body
of every launch site the campaign uses. A crewed vessel landed elsewhere on Kerbin is
therefore recovered too. Recovery returns its crew to the roster, so the approximation
errs towards crew safety. It recovers a vessel that another test may have wanted kept,
and the scripts that call it run only in the verification save.
"""
import time

LAUNCH_BODY = 'Kerbin'
SETTLE_S = 3.0


class LaunchSiteError(RuntimeError):
    """The launch site could not be shown free of crew, so nothing may launch."""


def crewed_at_launch_sites(sc):
    """Return (vessel, crew names) for each crewed vessel that may stand at a launch site.

    Any error while reading a vessel is raised as LaunchSiteError, never skipped."""
    down = (sc.VesselSituation.pre_launch, sc.VesselSituation.landed)
    found = []
    try:
        vessels = list(sc.vessels)
        for vessel in vessels:
            crew = [member.name for member in vessel.crew]
            if crew and vessel.situation in down and vessel.orbit.body.name == LAUNCH_BODY:
                found.append((vessel, crew))
    except Exception as error:
        raise LaunchSiteError(f'could not read the vessels at the launch site: {error}') from error
    return found


def clear_crewed_launch_site(sc, record, sleep=time.sleep):
    """Recover every crewed vessel at a launch site, confirm that none remains, and
    call `record(crew)` for each recovery. Raise LaunchSiteError if any crewed vessel
    cannot be recovered or is still there afterwards. Nothing is launched here."""
    found = crewed_at_launch_sites(sc)
    for vessel, crew in found:
        try:
            recoverable = vessel.recoverable
        except Exception as error:
            raise LaunchSiteError(f'could not check recovery of {crew}: {error}') from error
        if not recoverable:
            raise LaunchSiteError(f'refusing to launch, {crew} aboard an unrecoverable vessel')
        vessel.recover()
        record(crew)
    if found:
        sleep(SETTLE_S)
        remaining = crewed_at_launch_sites(sc)
        if remaining:
            raise LaunchSiteError(
                f'crew still at the launch site after recovery: {[c for _, c in remaining]}')
