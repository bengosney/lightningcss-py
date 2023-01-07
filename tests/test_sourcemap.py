import json

import lightningcss_py as lcss
import pytest


@pytest.fixture(scope="session")
def css_files(tmp_path_factory) -> tuple[str, list[str]]:
    css_paths = []
    basepath = tmp_path_factory.mktemp("data")
    stylesheets = []
    for c in ["a", "b", "c"]:
        fn = basepath / f"{c}.css"
        css_paths.append(fn)
        with open(fn, "w") as f:
            f.write(f".{c} {{color: rgba(255, 0, 0, .5);}}")
        stylesheets.append(f'@import "{c}.css";')
        print(fn)

    fn = basepath / "styles.css"
    with open(fn, "w") as f:
        f.write("\n".join(stylesheets))
    css_paths.append(fn)

    return str(fn), [str(p).lstrip("/") for p in css_paths]


def test_sourcemap(css_files):
    css_file, all_files = css_files
    _, sourcemap_json = lcss.bundle(css_file, source_map=True)

    sourcemap = json.loads(sourcemap_json)

    assert "sources" in sourcemap
    for css_path in all_files:
        assert css_path in sourcemap["sources"]


def test_no_sourcemap(css_files):
    css_file, _ = css_files
    _, sourcemap = lcss.bundle(css_file, source_map=False)
    assert sourcemap is None
