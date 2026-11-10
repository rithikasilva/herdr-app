//! Local version information and helpers for the bundled update notes.
use std::sync::Mutex;

#[derive(Clone, Default, serde::Serialize)]
pub struct CoreInfo {
    pub version: Option<String>,
    pub managed: Option<String>,
    pub ready: Option<String>,
    pub updating: bool,
    pub error: Option<String>,
}
static CORE: Mutex<CoreInfo> = Mutex::new(CoreInfo { version: None, managed: None, ready: None, updating: false, error: None });
pub fn core() -> CoreInfo { CORE.lock().unwrap().clone() }
pub fn set_core(version: Option<String>, managed: Option<String>) {
    let mut core = CORE.lock().unwrap();
    core.version = version;
    core.managed = managed;
}
pub fn ready() -> Option<String> { None }
pub fn status() -> serde_json::Value {
    serde_json::json!({
        "app": { "version": env!("CARGO_PKG_VERSION"), "ready": null, "updates": false },
        "notes": crate::update_notes::notes(None), "core": core(),
        "checked": null, "checks": false, "checking": false,
    })
}

pub(crate) fn sha256(bytes: &[u8]) -> String {
    use sha2::Digest;
    sha2::Sha256::digest(bytes).iter().map(|b| format!("{b:02x}")).collect()
}

pub(crate) fn newer(a: &str, b: &str) -> bool {
    fn parts(v: &str) -> (Vec<u64>, Option<&str>) {
        let (core, pre) = v.trim_start_matches('v').split_once('-').map_or((v.trim_start_matches('v'), None), |(c, p)| (c, Some(p)));
        (core.split('.').map(|n| n.parse().unwrap_or(0)).collect(), pre)
    }
    let ((ca, pa), (cb, pb)) = (parts(a), parts(b));
    match ca.cmp(&cb) {
        std::cmp::Ordering::Equal => match (pa, pb) {
            (None, Some(_)) => true,
            (Some(x), Some(y)) => x > y,
            _ => false,
        },
        o => o == std::cmp::Ordering::Greater,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn updates_are_disabled() {
        let status = super::status();
        assert_eq!(status["checks"], false);
        assert_eq!(status["checking"], false);
        assert_eq!(status["app"]["updates"], false);
        assert!(status["app"]["ready"].is_null());
        assert!(status["core"]["ready"].is_null());
    }
}
