use pyo3::{prelude::*, pyclass::boolean_struct::False, wrap_pymodule};
use std::path::Path;
use lightningcss::{
  bundler::{Bundler, FileProvider}, targets::Browsers,
};
use lightningcss::stylesheet::{
    MinifyOptions, ParserOptions, PrinterOptions, PseudoClasses, StyleAttribute, StyleSheet,
};
use pyo3::types::PyDict;
use parcel_sourcemap::SourceMap;


#[pyclass]
struct Config {
    targets: Option<Browsers>,
}

struct BundleResults {
  css: String,
  //map: 
}

/// Bundle the css
#[pyfunction(minify=false,source_map=true,project_root="\"/\"")]
pub fn bundle(filename: String, minify: bool, source_map: bool, project_root: &str) -> PyResult<(String, String)> {

    let targets = Browsers {
        safari: Some((13 << 16) | (2 << 8)),
        ..Browsers::default()
    };

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
        targets: None,
        analyze_dependencies: None,
        pseudo_classes: None,
      };

    return match stylesheet.to_css(opts) {
        Ok(res) => Ok((res.code, "bob".to_string())),
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