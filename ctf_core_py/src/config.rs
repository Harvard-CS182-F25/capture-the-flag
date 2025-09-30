use bevy::prelude::*;
use derivative::Derivative;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};
use serde::{Deserialize, Serialize};

#[gen_stub_pyclass]
#[pyclass(name = "Config")]
#[derive(Debug, Clone, Derivative, Serialize, Deserialize)]
#[derivative(Default)]
pub struct PyConfig {
    #[pyo3(get, set)]
    pub red_team_agent_ids: Vec<String>,

    #[pyo3(get, set)]
    pub blue_team_agent_ids: Vec<String>,

    #[pyo3(get, set)]
    pub red_team_agent_positions: Vec<(f32, f32)>,

    #[pyo3(get, set)]
    pub blue_team_agent_positions: Vec<(f32, f32)>,

    #[pyo3(get, set)]
    pub red_team_flag_positions: Vec<(f32, f32)>,

    #[pyo3(get, set)]
    pub blue_team_flag_positions: Vec<(f32, f32)>,

    #[pyo3(get, set)]
    pub red_team_capture_point_positions: Vec<(f32, f32)>,

    #[pyo3(get, set)]
    pub blue_team_capture_point_positions: Vec<(f32, f32)>,

    #[pyo3(get, set)]
    pub debug: bool,

    #[pyo3(get, set)]
    #[derivative(Default(value = "\"uv\".to_string()"))]
    pub python_exe: String,

    #[pyo3(get, set)]
    pub rate_hz: Option<f32>,
}

#[gen_stub_pymethods]
#[pymethods]
impl PyConfig {
    #[new]
    fn new() -> Self {
        Self::default()
    }

    #[staticmethod]
    fn from_json(json_str: &str) -> PyResult<Self> {
        serde_json::from_str(json_str).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to parse JSON: {}", e))
        })
    }

    fn __str__(&self) -> PyResult<String> {
        serde_json::to_string_pretty(self).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Failed to serialize to JSON: {}",
                e
            ))
        })
    }
}
