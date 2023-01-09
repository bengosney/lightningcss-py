import lightningcss_py as lcss
import pytest


@pytest.fixture(scope="session")
def css_file(tmp_path_factory):
    fn = tmp_path_factory.mktemp("data") / "styles.css"
    with open(fn, "w") as f:
        f.write(
            """.a {
    &.b {
        colour: red;
    }
}"""
        )

    return str(fn)


def test_nesting(css_file):
    css, _ = lcss.bundle(css_file, nesting=True, targets={"chrome": "95.0"})
    print(css)
    assert css == """.a.b {\n  colour: red;\n}\n"""


def test_nesting_off(css_file):
    with pytest.raises(ValueError):
        lcss.bundle(css_file, nesting=False)
