import unittest

from mrlynet import Gym

SNAKE = {"name": "snake", "apps": ["snake"]}


class GymTest(unittest.TestCase):
    def test_three_seeds_boot_step_read_and_close_with_no_process_left(self):
        with Gym() as gym:
            booted = gym.boot(SNAKE, [1, 2, 3])
            ticks = [
                next(m["verb"] for m in legal if m["verb"].endswith(".snake.tick"))
                for legal in gym.moves()
            ]
            stepped = gym.step([{"verb": verb} for verb in ticks])
            read = gym.read("snake/play", ["over"])
        self.assertEqual([e["tick"] for e in booted], [0, 0, 0])
        self.assertEqual([e["last"]["ok"] for e in stepped], [True, True, True])
        self.assertEqual(
            [(e["tick"], e["state"]) for e in read], [(1, {"over": None})] * 3
        )
        self.assertEqual(gym.proc.returncode, 0)


if __name__ == "__main__":
    unittest.main()
