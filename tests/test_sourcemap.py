import lightningcss_py as lcss
import pytest


@pytest.fixture(scope="session")
def css_file(tmp_path_factory):
    basepath = tmp_path_factory.mktemp("data")
    stylesheets = []
    for c in ["a", "b", "c"]:
        fn = basepath / f"{c}.css"
        with open(fn, "w") as f:
            f.write(f".{c} {{color: rgba(255, 0, 0, .5);}}")
        stylesheets.append(f'@import "{c}.css";')
        print(fn)

    fn = basepath / "styles.css"
    with open(fn, "w") as f:
        f.write("\n".join(stylesheets))

    return str(fn)


def test_sourcemap(css_file):
    _, sourcemap = lcss.bundle(css_file, source_map=True)
    assert sourcemap is not None


def test_no_sourcemap(css_file):
    _, sourcemap = lcss.bundle(css_file, source_map=False)
    assert sourcemap is None
