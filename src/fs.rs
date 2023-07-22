use crate::error::CWError;
use hex;
use nargo_halo2::artifacts::program::PreprocessedProgram;
use noirc_abi::{
    input_parser::{Format, InputValue},
    Abi, InputMap, MAIN_RETURN_NAME,
};
use serde_json;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

pub(crate) fn get_current_dir() -> PathBuf {
    std::env::current_dir().unwrap()
}

pub(crate) fn read_program_from_file<P: AsRef<Path>>(
    circuit_path: P,
) -> Result<PreprocessedProgram, CWError> {
    let input_string = std::fs::read(&circuit_path).map_err(|_| CWError::FilesystemError)?;
    let program = serde_json::from_slice(&input_string).expect("could not deserialize program");
    Ok(program)
}

pub(crate) fn read_crs<P: AsRef<Path>>(crs_path: P) -> Result<Vec<u8>, CWError> {
    let input_string = std::fs::read(crs_path).map_err(|_| CWError::FilesystemError)?;
    Ok(input_string)
}

pub(super) fn read_hex_data_proof<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, CWError> {
    let hex_data: Vec<_> = std::fs::read(path).map_err(|_| CWError::FilesystemError)?;
    let raw_bytes = hex::decode(hex_data).map_err(|e| {
        println!("error: {}", e);
        CWError::HexInvalid
    })?;
    Ok(raw_bytes)
}

pub(crate) fn read_inputs_from_file<P: AsRef<Path>>(
    path: P,
    file_name: &str,
    abi: &Abi,
) -> Result<(InputMap, Option<InputValue>), CWError> {
    let format = Format::Toml;
    if abi.is_empty() {
        return Ok((BTreeMap::new(), None));
    }

    let file_path = path.as_ref().join(file_name).with_extension(format.ext());
    if !file_path.exists() {
        return Err(CWError::FilesystemError);
    }

    let input_string = std::fs::read_to_string(file_path).unwrap();
    let mut input_map = format
        .parse(&input_string, abi)
        .map_err(|_| CWError::InputParseError)?;
    let return_value = input_map.remove(MAIN_RETURN_NAME);

    Ok((input_map, return_value))
}
