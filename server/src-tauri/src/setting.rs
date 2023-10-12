use confy;
use confy::ConfyError;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Parameter {
    pub host: String,
    pub port: u16,
    pub ca_path: String,
    pub client_cert_path: String,
}


pub fn read_setting() -> Result<Parameter, ConfyError> {
    let err = confy::load("WinMobileController", "WinMobileController");
    match err {
        Ok(parameter) => Ok(parameter),
        Err(err) => Err(err),
    }
}

pub fn save_setting(parameter: Parameter) -> Result<(), ConfyError> {
    let err = confy::store("WinMobileController", "WinMobileController", parameter);
    match err {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}