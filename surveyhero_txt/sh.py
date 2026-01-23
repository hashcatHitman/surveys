#!/usr/bin/env python3
import sys
import subprocess
import os
from pathlib import Path
from typing import Optional


def main():
    s2s_path = None
    try:
        s2s_path = Path(os.environ["S2S_PATH"])
    except KeyError:
        s2s_path = Path(__file__).parent / "sh_txt"

    if not s2s_path.exists():
        print(f"s2s_path={s2s_path} does not exist")
        sys.exit(1)

    build_res = subprocess.run(
        [
            "cargo",
            "build",
            "--manifest-path",
            str(s2s_path / "Cargo.toml"),
        ]
    )
    if build_res.returncode != 0:
        print(f"cargo build failed with {build_res.returncode}")
        sys.exit(1)

    print(sys.argv)

    subprocess.run(
        [
            str(s2s_path / "target" / "debug" / "sh_txt"),
        ]
        + sys.argv[1:]
    )


if __name__ == "__main__":
    main()
