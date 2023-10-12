use std::process::Command;

pub fn start_service(service_name: &str) -> Result<(), String> {
    let output = Command::new("cmd")
        .args(&["/C", "net", "start", service_name])
        .output()
        .map_err(|e| format!("Failed to start service: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "Failed to start service: {}",
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}

pub fn stop_service(service_name: &str) -> Result<(), String> {
    let output = Command::new("cmd")
        .args(&["/C", "net", "stop", service_name])
        .output()
        .map_err(|e| format!("Failed to stop service: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "Failed to stop service: {}",
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}