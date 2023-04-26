import logging
from contextlib import contextmanager
from time import perf_counter

import lightningcss_py as lcss

FORMAT = "%(levelname)s %(name)s %(asctime)-15s %(filename)s:%(lineno)d %(message)s"  # noqa

logging.basicConfig(format=FORMAT)
logging.getLogger().setLevel(logging.INFO)
logging.info("Test 1")


@contextmanager
def timeit(name):
    s = perf_counter()
    try:
        yield
    finally:
        e = perf_counter()
        print(f"Time for {name}: {e - s:0.4f} seconds")


def compile(src: str, dest: str) -> str:
    with timeit("bundle"):
        res = lcss.bundle(
            src,
            minify=True,
        )

    print("Saving css...")
    with open(dest, "w") as f:
        f.write(res.css)

    if res.source_map:
        print("Saving source map...")
        with open(f"{dest}.json", "w") as f:
            f.write(res.source_map)

    return res.css


if __name__ == "__main__":
    src = "css/main.css"
    dest = "css/main.min.css"
    css = compile(src, dest)
    # print(css)
