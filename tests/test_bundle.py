import lightningcss_py as lcss
import pytest


@pytest.fixture(scope="session")
def css_file(tmp_path_factory):
    fn = tmp_path_factory.mktemp("data") / "styles.css"
    with open(fn, "w") as f:
        f.write(
            """.a {
color: rgba(255, 0, 0, .5);
}"""
        )

    return str(fn)


def test_bundle(css_file):
    css = lcss.bundle(css_file)
    assert css == ".a {\n  color: #ff000080;\n}\n"


def test_bundle_targets_chrome_95(css_file):
    css = lcss.bundle(css_file, targets={"chrome": "95"})
    assert css == ".a {\n  color: #ff000080;\n}\n"


def test_bundle_targets_chrome_1(css_file):
    css = lcss.bundle(css_file, targets={"chrome": "1"})
    assert css == ".a {\n  color: rgba(255, 0, 0, .5);\n}\n"
