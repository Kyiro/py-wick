use john_wick_parse::dispatch::Extractor as WickExtractor;
use john_wick_parse::{
    read_asset,
    read_texture
};
use pyo3::{
    prelude::*,
    exceptions,
    Python,
    wrap_pyfunction
};
use pythonize::pythonize;
/*
    TODO
    - Proper Packages (similiar to node-wick)
      A workaround is currently being used.
*/

#[pyclass]
struct Extractor {
    extractor: WickExtractor
}

#[pymethods]
impl Extractor {
    #[new]
    pub fn __new__(path: &str, key: &str) -> PyResult<Self> {
        match WickExtractor::new(path, Some(&key)) {
            Ok(data) => Ok(Extractor {
                extractor: data
            }),
            Err(err) => Err(exceptions::PyException::new_err(err.to_string()))
        }
    }
    
    pub fn get_file_list(&mut self) -> Vec<String> {
        self.extractor.get_file_list().to_vec()
    }
    
    pub fn get_file(&mut self, file: &str) -> PyResult<Vec<u8>> {
        match self.extractor.get_file(file) {
            Ok(data) => Ok(data),
            Err(err) => Err(exceptions::PyException::new_err(err.to_string()))
        }
    }
    
    pub fn get_mount_point(&mut self) -> &str {
        self.extractor.get_mount_point()
    }
}

/* TEMP SOLUTION TO ACTUAL PACKAGES (SIMILIAR TO NODE-WICK) */
#[pyclass]
struct Package {
    uasset: Vec<u8>,
    ubulk: Option<Vec<u8>>
}

#[pymethods]
impl Package {
    #[new]
    pub fn __new__(uasset: Vec<u8>, ubulk: Option<Vec<u8>>) -> Self {
        Package {
            uasset: uasset,
            ubulk: ubulk
        }
    }
    
    pub fn get_texture(&mut self) -> PyResult<Vec<u8>> {
        let package = match read_asset(
            &self.uasset, 
            match self.ubulk { Some(ref a) => Some(a.as_slice()), None => None }
        ) {
            Ok(data) => data,
            Err(err) => return Err(exceptions::PyException::new_err(err.to_string()))
        };
        
        match read_texture(package) {
            Ok(data) => Ok(data),
            Err(err) => Err(exceptions::PyException::new_err(err.to_string()))
        }
    }
    
    pub fn get_data(&mut self, py: Python<'_>) -> PyResult<pyo3::Py<pyo3::PyAny>> {
        let package = match read_asset(
            &self.uasset, 
            match self.ubulk { Some(ref a) => Some(a.as_slice()), None => None }
        ) {
            Ok(data) => data,
            Err(err) => return Err(exceptions::PyException::new_err(err.to_string())),
        };
        
        match pythonize(py, &package) {
            Ok(data) => Ok(data),
            Err(err) => Err(exceptions::PyException::new_err(err.to_string()))
        }
    }
}

#[pyfunction]
pub fn read_pak_key(path: String) -> PyResult<String> {
    let header = match WickExtractor::new_header(&path) {
        Ok(data) => data,
        Err(err) => return Err(exceptions::PyException::new_err(err.to_string()))
    };

    Ok(header.get_key_guid().to_string())
}

#[pymodule]
fn py_wick(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(read_pak_key, m)?)?;
    m.add_class::<Extractor>()?;
    m.add_class::<Package>()?;
    Ok(())
}