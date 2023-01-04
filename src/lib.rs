use lightningcss::stylesheet::{
    MinifyOptions, ParserOptions, PrinterOptions, PseudoClasses, StyleAttribute, StyleSheet,
};
use lightningcss::{
    bundler::{Bundler, FileProvider},
    targets::Browsers,
};
use parcel_sourcemap::SourceMap;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::collections::HashMap;
use std::path::Path;

fn targets_to_browsers(targets: &PyDict) -> Option<Browsers> {
    let mut target_struct = Browsers::default();

    let target_map: HashMap<String, Option<u32>> = match targets.extract() {
        Ok(t) => t,
        Err(_) => return Some(target_struct),
    };

    for (k, v) in target_map.iter() {
        let val = Some(v.unwrap_or(0).to_owned() << 16);
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
    source_map = true,
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

    return match stylesheet.to_css(opts) {
        Ok(res) => Ok(res.code),
        Err(_) => todo!(),
    };

    /*
    let map = if let Some(mut source_map) = source_map {
      if let Some(input_source_map) = &config.input_source_map {
        if let Ok(mut sm) = SourceMap::from_json("/", input_source_map) {
          let _ = source_map.extends(&mut sm);
        }
      }

      source_map.to_json(None).ok()
    } else {
      None
    };
    // */
}

/// A Python module implemented in Rust.
#[pymodule]
fn lightningcss_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(bundle, m)?)?;
    Ok(())
}
