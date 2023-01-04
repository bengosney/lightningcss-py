from contextlib import contextmanager
from time import perf_counter

import lightningcss_py as lcss


@contextmanager
def timeit(name):
    s = perf_counter()
    try:
        yield
    finally:
        e = perf_counter()
        print(f"Time for {name}: {e - s:0.4f} seconds")


def compile(src: str, dest: str) -> str:
    with open(dest, "w") as f:
        with timeit("bundle"):
            css = lcss.bundle(src, minify=True)
        f.write(css)

    return css


if __name__ == "__main__":
    src = "style.css"
    dest = "style.min.css"
    css = compile(src, dest)
    # print(css)
