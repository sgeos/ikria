"""Sizing for solid-motor burns, from the game's part data, for TP-0002 and the program
document.

The Sepratron I burns at a fixed rate, so its fuel load sets its burn time and its
impulse, and its thrust limiter scales thrust and flow together, which lengthens the
burn without changing the impulse. A burn is therefore chosen by fuel load. This module
gives the rocket-equation velocity change, the fuel a target velocity change needs, and
a planar point-mass ascent integration for an airless body.

Assumptions. Vacuum figures apply to airless bodies. The ascent model is planar, holds a
fixed pitch above the local horizon during each burn, treats the body as a sphere at the
site's radius, and adds the surface's rotation speed. It ignores terrain, which a site
choice must check, and attitude error, which the flight measures.
"""
import math

G0 = 9.80665
SEP_THRUST_VAC_N = 18_000.0
SEP_ISP_VAC = 154.0
SEP_ISP_ASL = 118.0
SEP_FUEL_UNITS = 8.0
FUEL_T_PER_UNIT = 0.0075
SEP_DRY_T = 0.0125


def delta_v(isp, m0_t, m1_t):
    if not 0 < m1_t <= m0_t:
        raise ValueError("masses must satisfy 0 < m1 <= m0")
    return isp * G0 * math.log(m0_t / m1_t)


def fuel_for(isp, m0_t, dv):
    """Propellant mass in tonnes that gives `dv` from an initial mass `m0_t`."""
    if dv < 0:
        raise ValueError("a velocity change is not negative")
    return m0_t * (1.0 - math.exp(-dv / (isp * G0)))


def units(fuel_t, motors):
    """Solid-fuel units per motor for a total propellant mass shared by `motors`."""
    per = fuel_t / motors / FUEL_T_PER_UNIT
    if per > SEP_FUEL_UNITS:
        raise ValueError(f"{per:.2f} units exceeds a Sepratron's 8")
    return per


def burn(state, mu, motors, fuel_units, pitch_deg, dt=0.01):
    """Integrate one fixed-pitch burn of `motors` sepratrons carrying `fuel_units` each,
    from state (x, y, vx, vy, m) in a planar inertial frame centred on the body. Pitch is
    measured above the local horizon, toward the direction of rotation."""
    x, y, vx, vy, m = state
    flow = SEP_THRUST_VAC_N / (SEP_ISP_VAC * G0) / 1000.0 * motors   # t/s
    fuel = motors * fuel_units * FUEL_T_PER_UNIT
    t = 0.0
    steps = 0
    while fuel > 1e-9:
        steps += 1
        if steps > 1_000_000:
            raise RuntimeError("burn integration did not terminate")
        r = math.hypot(x, y)
        up = (x / r, y / r)
        east = (-up[1], up[0])
        p = math.radians(pitch_deg)
        d = (math.cos(p) * east[0] + math.sin(p) * up[0], math.cos(p) * east[1] + math.sin(p) * up[1])
        h = min(dt, fuel / flow)
        a_thrust = SEP_THRUST_VAC_N * motors / (m * 1000.0)
        g = mu / r ** 2
        ax = a_thrust * d[0] - g * up[0]
        ay = a_thrust * d[1] - g * up[1]
        vx, vy = vx + ax * h, vy + ay * h
        x, y = x + vx * h, y + vy * h
        m -= flow * h
        fuel -= flow * h
        t += h
    return (x, y, vx, vy, m), t


def elements(state, mu, radius):
    x, y, vx, vy, _ = state
    r = math.hypot(x, y)
    v2 = vx * vx + vy * vy
    energy = v2 / 2 - mu / r
    h = x * vy - y * vx
    a = -mu / (2 * energy)
    e = math.sqrt(max(0.0, 1 + 2 * energy * h * h / mu ** 2))
    return {"apoapsis_alt_m": a * (1 + e) - radius, "periapsis_alt_m": a * (1 - e) - radius,
            "speed_mps": math.sqrt(v2), "r_alt_m": r - radius}
