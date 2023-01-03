import lightningcss_py as lcss
from icecream import ic


def compile(src: str, dest: str) -> None:
    with open(dest, "w") as f:
        css, other = lcss.bundle(src)
        ic(other)
        f.write(css)


if __name__ == "__main__":
    src = "style.css"
    dest = "style.min.css"
    compile(src, dest)
