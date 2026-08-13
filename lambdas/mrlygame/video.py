import json
import os
import shlex
import subprocess
import sys

# FFMPEG

def run_ffmpeg(cmd):
    result = subprocess.run(cmd, capture_output=True, text=True)
    if result.returncode != 0:
        raise RuntimeError(f"ffmpeg failed: {result.stderr}")

# HELPERS

def timeline_entries(home, folder, timeline):
    return [
        (os.path.join(home, folder, f"{entry['frame']:04d}.png"), entry["us"] / 1_000_000)
        for entry in timeline
    ]

def encode(entries, audio, output, size, fps, dry):
    concat_path = os.path.join(os.path.dirname(output), "concat.txt")
    with open(concat_path, "w") as f:
        for path, duration in entries:
            f.write(f"file '{os.path.abspath(path)}'\n")
            f.write(f"duration {duration}\n")
        if entries:
            f.write(f"file '{os.path.abspath(entries[-1][0])}'\n")
    vf = f"scale={size}:{size}:flags=neighbor"
    if os.path.exists(audio):
        cmd = [
            "ffmpeg", "-y", "-f", "concat", "-safe", "0", "-i", concat_path,
            "-i", audio, "-vf", vf, "-c:v", "libx264",
            "-c:a", "aac", "-b:a", "128k", "-ar", "44100", "-r", str(fps),
            "-af", "afade=t=in:st=0:d=0.25,areverse,afade=t=in:st=0:d=0.25,areverse",
            "-pix_fmt", "yuv420p", output,
        ]
    else:
        cmd = [
            "ffmpeg", "-y", "-f", "concat", "-safe", "0", "-i", concat_path,
            "-vf", vf, "-c:v", "libx264", "-r", str(fps),
            "-pix_fmt", "yuv420p", output,
        ]
    if dry:
        print(shlex.join(cmd))
        return
    run_ffmpeg(cmd)
    os.remove(concat_path)
    print(f"Saved: {output}")

# QUEST VIDEO

def assemble(home, dry=False):
    with open(os.path.join(home, "quest.json")) as f:
        quest = json.load(f)
    manifest = quest["manifest"]
    entries = timeline_entries(home, "frames", manifest["frames"])
    entries += timeline_entries(home, "heatmap", manifest["heatmap"])
    encode(
        entries,
        os.path.join(home, manifest["audio"]),
        os.path.join(home, f"{quest['key']}.mp4"),
        manifest["size"],
        max(manifest["fps"], manifest["heatmap_fps"]) * 2,
        dry,
    )

# TERMINAL

def help():
    commands = [
        ("<dir>", "assemble <key>.mp4 from a mrlygame emit directory"),
        ("<dir> dry", "write concat.txt and print the ffmpeg invocation only"),
    ]
    width = max(len(name) for name, _ in commands)
    print("mrlyvideo")
    print()
    for name, desc in commands:
        print(f"  {name:<{width}}  {desc}")
    print()

def terminal():
    match sys.argv[1:]:
        case [home]:
            assemble(home)
        case [home, "dry"] | ["dry", home]:
            assemble(home, dry=True)
        case _:
            help()
            sys.exit(2)

if __name__ == "__main__":
    terminal()
