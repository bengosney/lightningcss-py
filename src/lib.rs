use lightningcss::stylesheet::{ParserOptions, PrinterOptions};
// MinifyOptions, StyleAttribute, StyleSheet, PseudoClasses
use lightningcss::{
    bundler::{Bundler, FileProvider},
    targets::Browsers,
};
use parcel_sourcemap::SourceMap;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::collections::HashMap;
use std::path::Path;

use log::info;
use pyo3_log;

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

#[pyfunction]
pub fn browser_version(version: String) -> u32 {
    return parse_version(&version);
}

fn targets_to_browsers(targets: &PyDict) -> Option<Browsers> {
    let mut target_struct = Browsers::default();

    let target_map: HashMap<String, String> = match targets.extract() {
        Ok(t) => t,
        Err(_) => return Some(target_struct),
    };

    for (k, v) in target_map.iter() {
        let val = Some(parse_version(v));
        match k.as_str() {
            "android" => target_struct.android = val,
            "chrome" => target_struct.chrome = val,
            "edge" => target_struct.edge = val,
            "firefox" => target_struct.firefox = val,
            "ie" => target_struct.ie = val,
            "ios_saf" => target_struct.ios_saf = val,
            "opera" => target_struct.opera = val,
            "safari" => target_struct.safari = val,
            "samsung" => target_struct.samsung = val,
            _ => {}
        }
    }

    return Some(target_struct);
}

/// Bundle the css
#[pyfunction(
    minify = false,
    source_map = false,
    project_root = "\"/\"",
    targets = "None"
)]
pub fn bundle(
    filename: String,
    targets: Option<&PyDict>,
    minify: bool,
    source_map: bool,
    project_root: &str,
) -> PyResult<String> {
    let target_struct = match targets {
        Some(t) => targets_to_browsers(t),
        None => None,
    };

    let mut source_map_obj = match source_map {
        true => {
            let mut sm = SourceMap::new(&project_root);
            sm.add_source(&filename);
            Some(sm)
        }
        false => None,
    };

    let fs = FileProvider::new();
    let mut bundler = Bundler::new(&fs, None, ParserOptions::default());
    let stylesheet = bundler.bundle(Path::new(&filename)).unwrap();

    let opts = PrinterOptions {
        minify: minify,
        source_map: source_map_obj.as_mut(),
        targets: target_struct,
        analyze_dependencies: None,
        pseudo_classes: None,
    };

    let res = match stylesheet.to_css(opts) {
        Ok(res) => res,
        Err(_) => todo!(),
    };

    match source_map_obj.clone() {
        Some(sm) => {
            info!("source maps");
            for i in sm.get_sources() {
                info!("source: {}", i);
            }
        }
        None => {}
    }

    return Ok(res.code);
}

/// A Python module implemented in Rust.
#[pymodule]
fn lightningcss_py(_py: Python, m: &PyModule) -> PyResult<()> {
    pyo3_log::init();
    m.add_function(wrap_pyfunction!(bundle, m)?)?;
    m.add_function(wrap_pyfunction!(browser_version, m)?)?;
    Ok(())
}
