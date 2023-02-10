from typing import Optional

class Browsers:
    android: Optional[str]
    chrome: Optional[str]
    edge: Optional[str]
    firefox: Optional[str]
    ie: Optional[str]
    ios_saf: Optional[str]
    opera: Optional[str]
    safari: Optional[str]
    samsung: Optional[str]

    def __init__(
        self,
        android: Optional[str] = None,
        chrome: Optional[str] = None,
        edge: Optional[str] = None,
        firefox: Optional[str] = None,
        ie: Optional[str] = None,
        ios_saf: Optional[str] = None,
        opera: Optional[str] = None,
        safari: Optional[str] = None,
        samsung: Optional[str] = None,
    ) -> None: ...

class CompiledCss:
    css: str
    source_map: Optional[str]

def browser_version(version: str) -> int: ...
def bundle(
    filename: str,
    minify: bool = False,
    source_map: bool = False,
    project_root: str = "/",
    targets: Optional[Browsers] = None,
    nesting: bool = True,
) -> CompiledCss: ...
