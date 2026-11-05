import json
import os
import subprocess

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, "..", ".."))


class Fault(Exception):
    def __init__(self, code, note):
        super().__init__(f"{code}: {note}")
        self.code = code
        self.note = note


class Gym:
    def __init__(self, repo=None):
        cli = os.path.join(repo or REPO, "doors", "cli", "cli.ts")
        self.config = None
        self.size = 0
        self.proc = subprocess.Popen(
            ["bun", cli, "gym"],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            text=True,
            encoding="utf-8",
        )

    def __enter__(self):
        return self

    def __exit__(self, *_):
        self.close()

    def ask(self, request):
        self.proc.stdin.write(json.dumps(request) + "\n")
        self.proc.stdin.flush()
        line = self.proc.stdout.readline()
        if not line:
            raise Fault("crash", f"the gym exited with {self.proc.wait()}")
        reply = json.loads(line)
        if isinstance(reply, dict):
            raise Fault(reply["error"]["code"], reply["error"]["note"])
        return reply

    def boot(self, config, seeds=None):
        if seeds is None:
            request = {"op": "boot", "configs": [config]}
        else:
            request = {"op": "boot", "config": config, "seeds": list(seeds)}
        out = self.ask(request)
        self.config = config
        self.size = len(out)
        return out

    def step(self, acts):
        return self.ask({"op": "call", "acts": list(acts)})

    def moves(self):
        return self.ask({"op": "list"})

    def read(self, path="", shape=None):
        request = {"op": "read", "path": path}
        if shape is not None:
            request["shape"] = shape
        return self.ask(request)

    def lines(self):
        return self.ask({"op": "lines"})

    def reset(self, seed):
        if isinstance(seed, int):
            seed = range(seed, seed + max(1, self.size))
        return self.boot(self.config, seed)

    def close(self):
        if self.proc.poll() is None:
            try:
                self.ask({"op": "close"})
            except (BrokenPipeError, Fault):
                pass
        self.proc.stdin.close()
        self.proc.stdout.close()
        try:
            self.proc.wait(timeout=10)
        except subprocess.TimeoutExpired:
            self.proc.kill()
            self.proc.wait()
