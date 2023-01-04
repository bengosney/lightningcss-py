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

/// Bundle the css
#[pyfunction(
    minify = false,
    source_map = true,
    project_root = "\"/\"",
    //targets = "None"
)]
pub fn bundle(
    filename: String,
    targets: &PyDict,
    //targets: Option<&PyDict>,
    minify: bool,
    source_map: bool,
    project_root: &str,
) -> PyResult<String> {
    let mut target_struct = Browsers::default();
    let thing: HashMap<String, Option<u32>> = targets.extract()?;
    for (k, v) in thing.iter() {
        match k.as_str() {
            "android" => {
                target_struct.android = v.to_owned();
            }
            "chrome" => {
                target_struct.chrome = v.to_owned();
            }
            "edge" => {
                target_struct.edge = v.to_owned();
            }
            "firefox" => {
                target_struct.firefox = v.to_owned();
            }
            "ie" => {
                target_struct.ie = v.to_owned();
            }
            "ios_saf" => {
                target_struct.ios_saf = v.to_owned();
            }
            "opera" => {
                target_struct.opera = v.to_owned();
            }
            "safari" => {
                target_struct.safari = v.to_owned();
            }
            "samsung" => {
                target_struct.samsung = v.to_owned();
            }
            _ => {}
        }
    }

    let mut source_map_obj = if source_map {
        let mut sm = SourceMap::new(&project_root);
        sm.add_source(&filename);
        Some(sm)
    } else {
        None
    };

    let fs = FileProvider::new();
    let mut bundler = Bundler::new(&fs, None, ParserOptions::default());
    let stylesheet = bundler.bundle(Path::new(&filename)).unwrap();

    let opts = PrinterOptions {
        minify: minify,
        source_map: source_map_obj.as_mut(),
        targets: Some(target_struct),
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
