use std::path::Path;
use std::process::Command;

fn escape_json(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

fn run_command(program: &str, args: &[&str]) -> Option<String> {
    let output = Command::new(program).args(args).output().ok()?;
    if !output.status.success() {
        return None;
    }
    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if stdout.is_empty() {
        None
    } else {
        Some(stdout)
    }
}

fn query_registry_value(path: &str, value_name: &str) -> Option<String> {
    let output = run_command("reg", &["query", path, "/v", value_name])?;

    for line in output.lines() {
        if line.contains(value_name) {
            // Typical format:
            // DisplayVersion    REG_SZ    22H2
            // version           REG_SZ    135.0.7049.96
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                return Some(parts[2..].join(" "));
            }
        }
    }

    None
}

fn get_file_version(path: &str) -> Option<String> {
    if !Path::new(path).exists() {
        return None;
    }

    let ps_script = format!(
        "(Get-Item '{}').VersionInfo.ProductVersion",
        path.replace('\'', "''")
    );

    run_command("powershell", &["-NoProfile", "-Command", &ps_script])
}

fn get_windows_info() -> String {
    let product_name = query_registry_value(
        r"HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion",
        "ProductName",
    )
    .unwrap_or_else(|| "Unknown".to_string());

    let display_version = query_registry_value(
        r"HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion",
        "DisplayVersion",
    )
    .or_else(|| {
        query_registry_value(
            r"HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion",
            "ReleaseId",
        )
    })
    .unwrap_or_else(|| "Unknown".to_string());

    let current_build = query_registry_value(
        r"HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion",
        "CurrentBuild",
    )
    .unwrap_or_else(|| "Unknown".to_string());

    format!(
        "{{\"product_name\":\"{}\",\"display_version\":\"{}\",\"build\":\"{}\"}}",
        escape_json(&product_name),
        escape_json(&display_version),
        escape_json(&current_build)
    )
}

fn get_chrome_version() -> Option<String> {
    query_registry_value(r"HKCU\Software\Google\Chrome\BLBeacon", "version")
        .or_else(|| query_registry_value(r"HKLM\Software\Google\Chrome\BLBeacon", "version"))
        .or_else(|| get_file_version(r"C:\Program Files\Google\Chrome\Application\chrome.exe"))
        .or_else(|| {
            get_file_version(r"C:\Program Files (x86)\Google\Chrome\Application\chrome.exe")
        })
}

fn get_edge_version() -> Option<String> {
    query_registry_value(r"HKCU\Software\Microsoft\Edge\BLBeacon", "version")
        .or_else(|| query_registry_value(r"HKLM\Software\Microsoft\Edge\BLBeacon", "version"))
        .or_else(|| {
            get_file_version(r"C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe")
        })
        .or_else(|| get_file_version(r"C:\Program Files\Microsoft\Edge\Application\msedge.exe"))
}

fn get_firefox_version() -> Option<String> {
    query_registry_value(r"HKLM\Software\Mozilla\Mozilla Firefox", "CurrentVersion")
        .or_else(|| get_file_version(r"C:\Program Files\Mozilla Firefox\firefox.exe"))
        .or_else(|| get_file_version(r"C:\Program Files (x86)\Mozilla Firefox\firefox.exe"))
}

fn json_field(name: &str, value: Option<String>) -> String {
    match value {
        Some(v) => format!("\"{}\":\"{}\"", name, escape_json(&v)),
        None => format!("\"{}\":null", name),
    }
}

fn main() {
    let hostname = run_command("hostname", &[]).unwrap_or_else(|| "Unknown".to_string());

    let windows_info = get_windows_info();
    let chrome = get_chrome_version();
    let edge = get_edge_version();
    let firefox = get_firefox_version();

    let json = format!(
        "{{\
            \"hostname\":\"{}\",\
            \"os\":{},\
            \"browsers\":{{{},{},{}}}\
        }}",
        escape_json(&hostname),
        windows_info,
        json_field("chrome", chrome),
        json_field("edge", edge),
        json_field("firefox", firefox),
    );

    println!("{}", json);
}
