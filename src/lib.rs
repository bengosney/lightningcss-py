use lightningcss::stylesheet::{ParserOptions, PrinterOptions};
use lightningcss::{
    bundler::{Bundler, FileProvider},
    targets::Browsers,
};
use parcel_sourcemap::SourceMap;
use pyo3::prelude::*;
use std::path::Path;

use pyo3::exceptions::PyValueError;
use pyo3_log;

#[allow(dead_code)]
#[pyclass(name = "Browsers")]
#[derive(Clone)]
pub struct BrowsersPy {
    #[pyo3(get, set)]
    android: Option<String>,
    #[pyo3(get, set)]
    chrome: Option<String>,
    #[pyo3(get, set)]
    edge: Option<String>,
    #[pyo3(get, set)]
    firefox: Option<String>,
    #[pyo3(get, set)]
    ie: Option<String>,
    #[pyo3(get, set)]
    ios_saf: Option<String>,
    #[pyo3(get, set)]
    opera: Option<String>,
    #[pyo3(get, set)]
    safari: Option<String>,
    #[pyo3(get, set)]
    samsung: Option<String>,
}

#[pymethods]
impl BrowsersPy {
    #[new]
    fn init(
        android: Option<String>,
        chrome: Option<String>,
        edge: Option<String>,
        firefox: Option<String>,
        ie: Option<String>,
        ios_saf: Option<String>,
        opera: Option<String>,
        safari: Option<String>,
        samsung: Option<String>,
    ) -> Self {
        return BrowsersPy {
            android,
            chrome,
            edge,
            firefox,
            ie,
            ios_saf,
            opera,
            safari,
            samsung,
        };
    }

    fn __repr__(&self) -> String {
        format!(
            "Browsers({:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?})",
            self.android,
            self.chrome,
            self.edge,
            self.firefox,
            self.ie,
            self.ios_saf,
            self.opera,
            self.safari,
            self.samsung
        )
    }
    fn __str__(&self) -> String {
        self.__repr__()
    }
}

impl From<BrowsersPy> for Browsers {
    fn from(value: BrowsersPy) -> Self {
        Browsers {
            android: optional_parse_version(value.android),
            chrome: optional_parse_version(value.chrome),
            edge: optional_parse_version(value.edge),
            firefox: optional_parse_version(value.firefox),
            ie: optional_parse_version(value.ie),
            ios_saf: optional_parse_version(value.ios_saf),
            opera: optional_parse_version(value.opera),
            safari: optional_parse_version(value.safari),
            samsung: optional_parse_version(value.samsung),
        }
    }
}

fn _unparse_version(int: u32) -> String {
    return format!(
        "{}.{}.{}",
        (int & 0x00FF0000) >> 16,
        (int & 0x0000FF00) >> 8,
        int & 0x000000FF
    )
    .to_string();
}

fn parse_version(version: &String) -> u32 {
    let mut parts = version
        .split(".")
        .filter_map(|c| c.parse::<u32>().ok())
        .collect::<Vec<u32>>();
    parts.resize(3, 0);

    return (parts[0] << 16) | (parts[1] << 8) | parts[2];
}

fn optional_parse_version(version: Option<String>) -> Option<u32> {
    match version {
        Some(ver) => Some(parse_version(&ver)),
        None => None,
    }
}

#[pyfunction]
pub fn browser_version(version: String) -> u32 {
    return parse_version(&version);
}

/// Bundle the css
#[pyfunction(
    minify = false,
    source_map = false,
    project_root = "\"/\"",
    targets = "None",
    nesting = true
)]
pub fn bundle(
    filename: String,
    targets: Option<BrowsersPy>,
    minify: bool,
    source_map: bool,
    project_root: &str,
    nesting: bool,
) -> PyResult<(String, Option<String>)> {
    let mut source_map_obj = match source_map {
        true => Some(SourceMap::new(&project_root)),
        false => None,
    };

    let fs = FileProvider::new();

    let parser_options = ParserOptions {
        nesting: nesting,
        ..ParserOptions::default()
    };

    let mut bundler = Bundler::new(&fs, source_map_obj.as_mut(), parser_options);
    let stylesheet = match bundler.bundle(Path::new(&filename)) {
        Ok(s) => s,
        Err(e) => return Err(PyValueError::new_err(format!("{}", e))),
    };

    let opts = PrinterOptions {
        minify: minify,
        source_map: source_map_obj.as_mut(),
        targets: match targets {
            Some(t) => Some(t.into()),
            None => None,
        },
        analyze_dependencies: None,
        pseudo_classes: None,
    };

    let res = match stylesheet.to_css(opts) {
        Ok(res) => res,
        Err(e) => return Err(PyValueError::new_err(format!("{}", e))),
    };

    let source_map_output: Option<String> = match source_map_obj {
        Some(s) => s.to_owned().to_json(Some(project_root)).ok(),
        None => None,
    };

    return Ok((res.code, source_map_output));
}

/// A Python module implemented in Rust.
#[pymodule]
fn lightningcss_py(_py: Python, m: &PyModule) -> PyResult<()> {
    pyo3_log::init();
    m.add_function(wrap_pyfunction!(bundle, m)?)?;
    m.add_function(wrap_pyfunction!(browser_version, m)?)?;
    m.add_class::<BrowsersPy>()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::parse_version;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_version_1() {
        assert_eq!(parse_version(&"1".to_string()), 65536);
    }

    #[test]
    fn test_version_1_0() {
        assert_eq!(parse_version(&"1.0".to_string()), 65536);
    }

    #[test]
    fn test_version_1_0_0() {
        assert_eq!(parse_version(&"1.0.0".to_string()), 65536);
    }

    /*
    #[test]
    fn test_version_number() {
        assert_eq!(browser_version(1), 65536);
    }
    */
}
