use pyo3::prelude::*;
use pyo3::types::IntoPyDict;


pub struct AudioSegment {
    segment: PyObject,
}


pub fn export(segment: AudioSegment, key: &str, file_format: &str) -> Result<(), String> {
    let py_segment = segment.segment;

    Python::with_gil(|py| {
        println!("let's see - {}", py_segment);
        py_segment.call_method1(py, "export", (key, file_format))
    }).unwrap();

    Ok(())
}

pub fn from_file(key: &str) -> Result<AudioSegment, String> {
    let output: PyResult<PyObject> = Python::with_gil(|py| {
        let locals = [("os", py.import("os")?), ("pydub", py.import("pydub")?)].into_py_dict(py);
        let code = format!("pydub.AudioSegment.from_from({key})");
        let segment: PyObject = py.eval(&code, None, Some(locals))?.extract()?;

        println!("let's see - {}", segment);

        Ok(segment)
    });

    match output {
        Ok(audio) => Ok(AudioSegment{segment: audio}),
        _ => Err("some error idk".to_string()),
    }
}

// pub fn reverse(segment: PyObject) -> Result<AudioSegment, String> {
//     Ok(())
// }