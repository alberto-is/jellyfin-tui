use std::process::{Child, Command, Stdio};

pub struct SleepInhibitor {
    child: Option<Child>,
}

impl SleepInhibitor {
    pub fn new() -> Self {
        Self { child: None }
    }

    pub fn acquire(&mut self) {
        if self.child.is_some() {
            return;
        }
        match Command::new("systemd-inhibit")
            .args([
                "--what=sleep",
                "--who=jellyfin-tui",
                "--why=Playing audio",
                "--mode=delay",
                "sleep",
                "infinity",
            ])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
        {
            Ok(child) => {
                log::info!("Acquired sleep inhibitor (pid {})", child.id());
                self.child = Some(child);
            }
            Err(e) => {
                log::warn!("Failed to acquire sleep inhibitor: {}", e);
            }
        }
    }

    pub fn release(&mut self) {
        if let Some(mut child) = self.child.take() {
            let _ = child.kill();
            let _ = child.wait();
            log::info!("Released sleep inhibitor");
        }
    }
}

impl Drop for SleepInhibitor {
    fn drop(&mut self) {
        self.release();
    }
}
