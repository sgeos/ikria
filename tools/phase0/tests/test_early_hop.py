"""Offline tests of craft bounds, parachute and touchdown decisions, and live guards.

The live-guard tests replace kRPC with plain fakes that hold real values and record
every action. Each guard has a control run, in which a fully valid article reaches the
guarded action, and a run that changes only the guarded value. Deleting a guard makes
the changed run reach the action, so its test fails. Mock objects are not used for
the game state, because comparisons between mocks raise or succeed by accident and
attribute assignment on a mock is not recorded as a call.
"""
import contextlib
import importlib.util
import io
import json
import math
import pathlib
import tempfile
import types
import unittest
from unittest import mock


def load(name):
    path = pathlib.Path(__file__).resolve().parents[1] / (name + '.py')
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


hop = load('early_hop')
prepare = load('prepare_early_hop')
setup = load('setup_early_hop')
launch_site = load('launch_site')

SAVE = 'Ikria Physical Verification'
CRAFT = 'IKR-TS-001B'
FUEL, THRUST_PERCENT = prepare.REVISIONS['B']


class DescentTests(unittest.TestCase):
    def test_safe_descent(self):
        self.assertTrue(hop.should_deploy(1200, -5, 0, True))

    def test_ascent_and_hover(self):
        for speed in (-1, 0, 10):
            self.assertFalse(hop.should_deploy(1200, speed, 0, True))

    def test_altitude_bounds(self):
        for altitude in (-1, 0, 5, 1500, 2000):
            self.assertFalse(hop.should_deploy(altitude, -5, 0, True))

    def test_burning_or_unsafe(self):
        self.assertFalse(hop.should_deploy(1200, -5, .01, True))
        self.assertFalse(hop.should_deploy(1200, -5, 0, False))

    def test_invalid_values(self):
        for values in ((math.nan, -5, 0), (1200, math.inf, 0),
                       (1200, -5, math.nan), (1200, -5, -1)):
            with self.assertRaises(ValueError):
                hop.should_deploy(*values, True)


class TouchdownTests(unittest.TestCase):
    STEP = 0.25

    def feed(self, touchdown, rows):
        """Feed (ut, down, speed) rows and return the first settled time, or None."""
        for ut, down, speed in rows:
            if touchdown.update(ut, down, speed):
                return ut
        return None

    def test_settles_after_the_settle_period_and_not_before(self):
        rows = [(i * self.STEP, True, 0.0) for i in range(40)]
        self.assertEqual(self.feed(hop.Touchdown(), rows), hop.SETTLE_S)

    def test_bounce_restarts_the_wait_and_is_counted(self):
        rows = [(0.0, True, 0.0), (0.25, True, 0.0), (0.5, False, 1.0)]
        rows += [(0.75 + i * self.STEP, True, 0.0) for i in range(40)]
        touchdown = hop.Touchdown()
        self.assertEqual(self.feed(touchdown, rows), 0.75 + hop.SETTLE_S)
        self.assertEqual(touchdown.bounces, 1)

    def test_fast_contact_is_not_settled(self):
        speed = hop.SETTLE_SPEED_M_S
        rows = [(i * self.STEP, True, speed) for i in range(40)]
        self.assertIsNone(self.feed(hop.Touchdown(), rows))

    def test_invalid_time_is_refused(self):
        for rows in ([(math.nan, True, 0.0)], [(0.0, True, math.inf)],
                     [(1.0, True, 0.0), (0.5, True, 0.0)]):
            with self.assertRaises(ValueError):
                self.feed(hop.Touchdown(), rows)


class CraftTests(unittest.TestCase):
    def setUp(self):
        self.source = 'ship = test\nthrustPercentage = 100\namount = 140\n'
        for name, count in prepare.PARTS.items():
            self.source += (f'part = {name}_123\n') * count

    def test_revisions_keep_parts_and_set_bounded_values(self):
        for revision, (fuel, thrust) in prepare.REVISIONS.items():
            result = prepare.derive(self.source, revision, set(prepare.PARTS))
            self.assertIn(f'amount = {fuel}\n', result)
            self.assertIn(f'thrustPercentage = {thrust}\n', result)
            self.assertEqual(result.count('part ='), 8)
            self.assertIn('amount = 140\n', self.source)

    def test_missing_purchase_is_refused(self):
        with self.assertRaises(ValueError):
            prepare.derive(self.source, 'A', set())

    def test_changed_parts_are_refused(self):
        with self.assertRaises(ValueError):
            prepare.derive(self.source + 'part = unknown_1\n', 'A', set(prepare.PARTS))

    def test_changed_settings_are_refused(self):
        for source in (self.source.replace('amount = 140', 'amount = 139'),
                       self.source + 'thrustPercentage = 100\n'):
            with self.assertRaises(ValueError):
                prepare.derive(source, 'A', set(prepare.PARTS))

    def test_unknown_revision_is_refused(self):
        with self.assertRaises(ValueError):
            prepare.derive(self.source, 'C', set(prepare.PARTS))


class Stop(Exception):
    """Raised by a fake actuator, so that a run ends exactly at the guarded action."""


class Engine:
    """An engine whose ignition is recorded. Assignment to a mock would not be."""

    def __init__(self, world, active=False):
        self.world = world
        self._active = active
        self.thrust_limit = THRUST_PERCENT / 100

    @property
    def active(self):
        return self._active

    @active.setter
    def active(self, value):
        self.world.act('ignite')
        self._active = value
        if self.world.stop_at_ignition:
            raise Stop('ignite')


class Unreadable:
    """A vessel whose crew cannot be read, as an unloaded or vanishing vessel may be."""
    name = 'unreadable'

    @property
    def crew(self):
        raise RuntimeError('vessel is not available')


class World:
    """A verification save holding one valid hop article on the pad."""

    def __init__(self, directory):
        self.actions = []
        self.on_sleep = None
        self.stop_at_ignition = True
        self.loaded = SAVE
        self.directory = pathlib.Path(directory)
        enum = types.SimpleNamespace
        self.sc = types.SimpleNamespace(
            GameMode=enum(career='career', sandbox='sandbox'), game_mode='career',
            VesselSituation=enum(pre_launch='pre_launch', landed='landed',
                                 splashed='splashed', flying='flying'),
            ParachuteState=enum(stowed='stowed', deployed='deployed'),
            ParachuteSafeState=enum(safe='safe', unsafe='unsafe'),
            ut=100.0, funds=0.0, science=0.0, vessels=[],
            save=lambda name: self.act('save ' + name),
            launch_vessel=self.launch,
            get_kerbal=lambda name: types.SimpleNamespace(roster_status='available'))
        self.vessel = self.article()
        self.sc.active_vessel = self.vessel
        self.sc.vessels.append(self.vessel)
        self.conn = types.SimpleNamespace(
            space_center=self.sc, close=lambda: self.act('close'),
            krpc=types.SimpleNamespace(game_scene='flight'),
            debug=types.SimpleNamespace(infinite_electricity=False, infinite_propellant=False,
                                        no_crash_damage=False, unbreakable_joints=False,
                                        ignore_max_temperature=False))
        craft = self.directory / SAVE / 'Ships' / 'VAB' / (CRAFT + '.craft')
        craft.parent.mkdir(parents=True)
        craft.write_text('ship = ' + CRAFT + '\n')

    def act(self, what):
        self.actions.append(what)

    def launch(self, *args, **kwargs):
        self.act('launch')
        raise Stop('launch')

    def article(self, name=CRAFT, crew=(), situation='pre_launch', body='Kerbin',
                recoverable=True):
        parts = [types.SimpleNamespace(name=n) for n, count in prepare.PARTS.items()
                 for _ in range(count)]
        vessel = types.SimpleNamespace(
            name=name, crew_count=len(crew), situation=situation, packed=False,
            recoverable=recoverable, mass=1688.5, dry_mass=1600.0,
            control=types.SimpleNamespace(state='none'),
            comms=types.SimpleNamespace(can_communicate=False),
            resources=types.SimpleNamespace(amount=lambda resource: float(FUEL)),
            orbit=types.SimpleNamespace(body=types.SimpleNamespace(
                name=body, reference_frame='body')))
        vessel.crew = [types.SimpleNamespace(name=c, eva=self.eva) for c in crew]
        vessel.parts = types.SimpleNamespace(
            engines=[Engine(self)], all=parts,
            parachutes=[types.SimpleNamespace(state='stowed', safe_state='safe',
                                              deploy=lambda: self.act('deploy'))])

        def recover():
            self.act('recover ' + vessel.name)
            if vessel.recoverable and vessel in self.sc.vessels:
                self.sc.vessels.remove(vessel)
        vessel.recover = recover
        return vessel

    def sleep(self, seconds):
        if self.on_sleep is not None:
            self.on_sleep()

    def eva(self):
        self.act('eva')
        raise Stop('eva')

    def run(self, module, operation=None):
        """Run the script's main, returning the exception that ended it, if any."""
        output = self.directory / 'log.json'
        arguments = ['probe', str(self.directory)]
        if operation:
            arguments.append(operation)
        arguments += [str(output), '--craft', CRAFT]
        modules = {'krpc': types.SimpleNamespace(connect=lambda **_: self.conn),
                   'guard': types.SimpleNamespace(loaded_save=lambda *_: self.loaded)}
        with mock.patch.dict('sys.modules', modules), \
                mock.patch('sys.argv', arguments), \
                mock.patch.object(module.signal, 'signal'), \
                mock.patch.object(module.signal, 'alarm'), \
                mock.patch('time.sleep', new=self.sleep), \
                contextlib.redirect_stdout(io.StringIO()):
            try:
                module.main()
            except Exception as error:  # noqa: BLE001, the caller inspects it
                return error
        return None


class LiveGuardCase(unittest.TestCase):
    def world(self):
        directory = tempfile.TemporaryDirectory()
        self.addCleanup(directory.cleanup)
        return World(directory.name)

    def assertRefused(self, world, module, operation, forbidden):
        error = world.run(module, operation)
        self.assertIsInstance(error, RuntimeError, f'not refused, actions {world.actions}')
        self.assertNotIsInstance(error, Stop)
        for action in forbidden:
            self.assertNotIn(action, world.actions)
        self.assertEqual(world.actions[-1:], ['close'])


class HopGuardTests(LiveGuardCase):
    def test_control_valid_article_reaches_ignition(self):
        world = self.world()
        self.assertIsInstance(world.run(hop), Stop)
        self.assertEqual(world.actions, ['ignite', 'close'])

    def refused(self, change):
        world = self.world()
        change(world, world.vessel)
        self.assertRefused(world, hop, None, ['ignite'])

    def test_wrong_save(self):
        self.refused(lambda w, v: setattr(w, 'loaded', 'Ikria Automated Space Agency'))

    def test_wrong_game_mode(self):
        self.refused(lambda w, v: setattr(w.sc, 'game_mode', 'sandbox'))

    def test_wrong_vessel(self):
        self.refused(lambda w, v: setattr(v, 'name', 'another vessel'))

    def test_crew_aboard(self):
        self.refused(lambda w, v: setattr(v, 'crew_count', 1))

    def test_not_on_the_pad(self):
        self.refused(lambda w, v: setattr(v, 'situation', 'flying'))

    def test_packed(self):
        self.refused(lambda w, v: setattr(v, 'packed', True))

    def test_second_engine(self):
        self.refused(lambda w, v: v.parts.engines.append(Engine(w)))

    def test_engine_already_active(self):
        self.refused(lambda w, v: setattr(v.parts.engines[0], '_active', True))

    def test_second_parachute(self):
        self.refused(lambda w, v: v.parts.parachutes.append(v.parts.parachutes[0]))

    def test_parachute_not_stowed(self):
        self.refused(lambda w, v: setattr(v.parts.parachutes[0], 'state', 'deployed'))

    def test_changed_inventory(self):
        self.refused(lambda w, v: v.parts.all.append(types.SimpleNamespace(name='basicFin')))

    def test_changed_fuel(self):
        self.refused(lambda w, v: setattr(v.resources, 'amount', lambda r: FUEL * 2.0))

    def test_changed_thrust_limit(self):
        self.refused(lambda w, v: setattr(v.parts.engines[0], 'thrust_limit', 1.0))


class Script:
    """Scripted flight telemetry. The first row applies when the probe asks for its
    flight object, and each later row when the probe sleeps between samples, so every
    sample reads one consistent row. An exhausted script ends the run."""

    mean_altitude = speed = pitch = latitude = longitude = g_force = 0.0

    def __init__(self, world, rows):
        self.world, self.rows = world, list(rows)
        world.on_sleep = self.advance

    def flight(self, frame):
        self.advance()
        return self

    def advance(self):
        if not self.rows:
            raise Stop('script exhausted')
        ut, situation, altitude, speed, fuel = self.rows.pop(0)
        self.world.sc.ut = ut
        self.world.vessel.situation = situation
        self.world.vessel.resources.amount = lambda resource: fuel
        self.surface_altitude, self.vertical_speed = altitude, speed


class HopLandingTests(LiveGuardCase):
    def fly(self, rows):
        world = self.world()
        world.stop_at_ignition = False
        world.vessel.flight = Script(world, rows).flight
        error = world.run(hop)
        return world, error, hop_log(world)

    def ascent(self):
        rows = [(100.0, 'pre_launch', 1.0, 0.0, float(FUEL))]
        rows += [(100.5 + i, 'flying', 50.0 + i, -3.0, 0.0) for i in range(3)]
        return rows

    def test_bounce_is_not_a_landing(self):
        rows = self.ascent()
        rows += [(104.0, 'landed', 0.5, 0.0, 0.0), (104.25, 'flying', 1.0, 2.0, 0.0)]
        rows += [(104.5 + 0.25 * i, 'landed', 0.5, 0.0, 0.0) for i in range(30)]
        world, error, log = self.fly(rows)
        self.assertIsNone(error)
        self.assertEqual(log['outcome'], 'landed')
        self.assertEqual(log['bounces'], 1)
        # Settled a full settle period after the last touchdown, not at the first.
        self.assertEqual(log['settled_ut'], 104.5 + hop.SETTLE_S)
        self.assertEqual(world.actions.count('deploy'), 1)

    def test_run_that_ends_bouncing_has_no_outcome(self):
        rows = self.ascent()
        rows += [(104.0, 'landed', 0.5, 0.0, 0.0), (104.25, 'flying', 1.0, 2.0, 0.0)]
        world, error, log = self.fly(rows)
        self.assertIsInstance(error, Stop)
        self.assertNotIn('outcome', log)
        self.assertEqual(log['bounces'], 1)


def hop_log(world):
    return json.loads((world.directory / 'log.json').read_text())


class SetupGuardTests(LiveGuardCase):
    def test_control_empty_pad_reaches_launch(self):
        world = self.world()
        world.sc.vessels.clear()
        self.assertIsInstance(world.run(setup, 'place'), Stop)
        self.assertEqual(world.actions, ['launch', 'close'])

    def test_wrong_save_or_mode_touches_nothing(self):
        for operation in ('place', 'empty', 'recover'):
            for change in (lambda w: setattr(w, 'loaded', 'Ikria Automated Space Agency'),
                           lambda w: setattr(w.sc, 'game_mode', 'sandbox')):
                with self.subTest(operation=operation):
                    world = self.world()
                    change(world)
                    self.assertRefused(world, setup, operation, [])
                    self.assertEqual(world.actions, ['close'])

    def test_crewed_vessel_on_the_pad_is_recovered_before_launch(self):
        world = self.world()
        world.vessel.crew = [types.SimpleNamespace(name='Jebediah Kerman')]
        self.assertIsInstance(world.run(setup, 'place'), Stop)
        self.assertEqual(world.actions, ['recover ' + CRAFT, 'launch', 'close'])

    def test_unrecoverable_crewed_vessel_refuses_launch(self):
        world = self.world()
        world.vessel.crew = [types.SimpleNamespace(name='Jebediah Kerman')]
        world.vessel.recoverable = False
        self.assertRefused(world, setup, 'place', ['launch', 'recover ' + CRAFT])

    def test_unreadable_vessel_refuses_launch(self):
        world = self.world()
        world.sc.vessels.append(Unreadable())
        self.assertRefused(world, setup, 'place', ['launch'])

    def test_crew_still_present_after_recovery_refuses_launch(self):
        world = self.world()
        world.vessel.crew = [types.SimpleNamespace(name='Jebediah Kerman')]
        world.vessel.recover = lambda: world.act('recover ' + CRAFT)
        self.assertRefused(world, setup, 'place', ['launch'])

    def test_crew_on_another_body_does_not_block_launch(self):
        world = self.world()
        world.sc.vessels.clear()
        lander = world.article(name='lander', crew=['Bill Kerman'], situation='landed',
                               body='Mun', recoverable=False)
        world.sc.vessels.append(lander)
        self.assertIsInstance(world.run(setup, 'place'), Stop)
        self.assertEqual(world.actions, ['launch', 'close'])

    def crewed_pad_world(self):
        world = self.world()
        world.vessel.crew = world.article(crew=['Jebediah Kerman']).crew
        world.vessel.crew_count = 1
        return world

    def test_control_crewed_article_reaches_crew_removal(self):
        world = self.crewed_pad_world()
        self.assertIsInstance(world.run(setup, 'empty'), Stop)
        self.assertEqual(world.actions, ['eva', 'close'])

    def test_empty_refuses_on_any_changed_condition(self):
        changes = {
            'wrong vessel': lambda v: setattr(v, 'name', 'another vessel'),
            'no crew': lambda v: setattr(v, 'crew_count', 0),
            'two crew': lambda v: setattr(v, 'crew_count', 2),
            'not on the pad': lambda v: setattr(v, 'situation', 'flying'),
            'engine active': lambda v: setattr(v.parts.engines[0], '_active', True),
        }
        for label, change in changes.items():
            with self.subTest(label):
                world = self.crewed_pad_world()
                change(world.vessel)
                self.assertRefused(world, setup, 'empty', ['eva'])

    def test_control_landed_empty_article_is_recovered(self):
        world = self.world()
        world.vessel.situation = 'landed'
        self.assertIsNone(world.run(setup, 'recover'))
        self.assertEqual(world.actions[0], 'recover ' + CRAFT)

    def test_recover_refuses_on_any_changed_condition(self):
        changes = {
            'wrong vessel': lambda v: setattr(v, 'name', 'another vessel'),
            'crew aboard': lambda v: setattr(v, 'crew_count', 1),
            'still flying': lambda v: setattr(v, 'situation', 'flying'),
        }
        for label, change in changes.items():
            with self.subTest(label):
                world = self.world()
                world.vessel.situation = 'landed'
                change(world.vessel)
                self.assertRefused(world, setup, 'recover', ['recover ' + CRAFT])


class LaunchSiteTests(unittest.TestCase):
    def test_recovery_records_each_crew(self):
        directory = tempfile.TemporaryDirectory()
        self.addCleanup(directory.cleanup)
        world = World(directory.name)
        world.vessel.crew = [types.SimpleNamespace(name='Jebediah Kerman')]
        recorded = []
        launch_site.clear_crewed_launch_site(world.sc, recorded.append, sleep=lambda _: None)
        self.assertEqual(recorded, [['Jebediah Kerman']])
        self.assertEqual(launch_site.crewed_at_launch_sites(world.sc), [])


if __name__ == '__main__':
    unittest.main()
