/// Increments the patch version (e.g., from 0.0.1 to 0.0.2).
pub fn increment_patch_version(version: &str) -> String {
    let mut parts: Vec<u32> = version.split('.').map(|x| x.parse().unwrap_or(0)).collect();
    if parts.len() == 3 {
        parts[2] += 1;
    }
    format!("{}.{}.{}", parts[0], parts[1], parts[2])
}

/// Increments the major version (e.g., from 0.7.2 to 1.0.0).
pub fn increment_major_version(version: &str) -> String {
    let mut parts: Vec<u32> = version.split('.').map(|x| x.parse().unwrap_or(0)).collect();
    if parts.len() == 3 {
        parts[0] += 1;
        parts[1] = 0;
        parts[2] = 0;
    }
    format!("{}.{}.{}", parts[0], parts[1], parts[2])
}
