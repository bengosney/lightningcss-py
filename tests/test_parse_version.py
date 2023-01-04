import lightningcss_py as lcss


def test_version_1():
    assert lcss.parse_version("1") == 65536


def test_version_1_0():
    assert lcss.parse_version("1.0") == 65536


def test_version_1_0_0():
    assert lcss.parse_version("1.0.0") == 65536
