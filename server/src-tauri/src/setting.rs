use confy;
use confy::ConfyError;
use serde::{Deserialize, Serialize};

/// 設定パラメータ
/// 
/// # Fields
/// 
/// * `host` - MQTTブローカーのホスト名
/// * `port` - MQTTブローカーのポート番号
/// * `ca_path` - MQTTブローカーのCA証明書のパス
/// * `client_cert_path` - MQTTブローカーのクライアント証明書のパス
#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Parameter {
    pub host: String,
    pub port: u16,
    pub ca_path: String,
    pub client_cert_path: String,
}

/// confyから設定パラメータを読み込む
/// 
/// # Returns
/// 
/// * `Ok(Parameter)` - 成功
/// * `Err(ConfyError)` - 失敗
pub fn read_setting() -> Result<Parameter, ConfyError> {
    let err = confy::load("WinMobileController", "WinMobileController");
    match err {
        Ok(parameter) => Ok(parameter),
        Err(err) => Err(err),
    }
}

/// confyに設定パラメータを保存する
/// 
/// # Arguments
/// 
/// * `parameter` - 設定パラメータ
/// 
/// # Returns
/// 
/// * `Ok(())` - 成功
/// * `Err(ConfyError)` - 失敗
pub fn save_setting(parameter: Parameter) -> Result<(), ConfyError> {
    let err = confy::store("WinMobileController", "WinMobileController", parameter);
    match err {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}