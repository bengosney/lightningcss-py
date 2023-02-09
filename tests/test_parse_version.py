import lightningcss_py as lcss


def test_version_1():
    assert lcss.browser_version("1") == 65536


def test_version_1_0():
    assert lcss.browser_version("1.0") == 65536


def test_version_1_0_0():
    assert lcss.browser_version("1.0.0") == 65536


def test_browser_object_print():
    browsers = lcss.Browsers(chrome="95.0")
    expected = (
        "Browsers("
        "android: None, "
        "chrome: 95.0, "
        "edge: None, "
        "firefox: None, "
        "ie: None, "
        "ios_saf: None, "
        "opera: None, "
        "safari: None, "
        "samsung: None"
        ")"
    )
    assert f"{browsers}" == expected
