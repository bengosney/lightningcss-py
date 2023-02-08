import lightningcss_py as lcss
import pytest


def test_nesting(css_file_nested):
    css, _ = lcss.bundle(
        css_file_nested,
        nesting=True,
        targets=lcss.Browsers(chrome="95.0"),
    )
    print(css)
    assert css == """.a.b {\n  colour: red;\n}\n"""


def test_nesting_off(css_file_nested):
    with pytest.raises(ValueError):
        lcss.bundle(css_file_nested, nesting=False)
