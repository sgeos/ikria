"""Offline tests of craft bounds and parachute decision boundaries."""
import importlib.util
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


class LiveBoundaryTests(unittest.TestCase):
    def invoke(self, module, operation, save, vessel):
        sc = mock.Mock()
        sc.game_mode = sc.GameMode.career
        sc.active_vessel = vessel
        sc.ut = 0
        conn = mock.Mock(space_center=sc)
        with tempfile.TemporaryDirectory() as directory:
            output = str(pathlib.Path(directory) / 'log.json')
            arguments = ['probe', directory]
            if operation:
                arguments.append(operation)
            arguments += [output, '--craft', 'IKR-TS-001B']
            modules = {
                'krpc': types.SimpleNamespace(connect=lambda **_: conn),
                'guard': types.SimpleNamespace(loaded_save=lambda *_: save),
            }
            with mock.patch.dict('sys.modules', modules), \
                    mock.patch('sys.argv', arguments), \
                    mock.patch.object(module.signal, 'signal'), \
                    mock.patch.object(module.signal, 'alarm'):
                with self.assertRaises(RuntimeError):
                    module.main()
            sc.launch_vessel.assert_not_called()
            conn.close.assert_called_once()
        return sc

    def test_wrong_save_never_launches_or_ignites(self):
        for module, operation in ((setup, 'place'), (hop, None)):
            with self.subTest(module=module.__name__):
                vessel = mock.Mock()
                self.invoke(module, operation, 'Ikria Automated Space Agency', vessel)
                self.assertEqual(vessel.mock_calls, [])

    def test_wrong_vessel_never_removes_crew(self):
        vessel = mock.Mock()
        vessel.name = 'another vessel'
        self.invoke(setup, 'empty', 'Ikria Physical Verification', vessel)
        self.assertEqual(vessel.mock_calls, [])

    def test_crew_aboard_prevents_ignition(self):
        vessel = mock.Mock()
        vessel.name = 'IKR-TS-001B'
        vessel.crew_count = 1
        self.invoke(hop, None, 'Ikria Physical Verification', vessel)
        self.assertEqual(vessel.mock_calls, [])


if __name__ == '__main__':
    unittest.main()
