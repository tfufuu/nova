//! Implementierung eines einfachen Prozessmanagers.

use crate::{SystemError, SystemResult};
use async_trait::async_trait;
use novade_core::info; // Logging
use novade_core::types::NovaId; // Für Test-App Erstellung
use novade_domain::entities::application::Application;
use std::process::{Child, Command, Stdio}; // Stdio für das Abtrennen des Prozesses

/// Typalias für eine Prozess-ID.
pub type ProcessId = u32;

/// Trait für einen Prozessmanager, der Anwendungen starten kann.
#[async_trait]
pub trait ProcessManager: Send + Sync {
    /// Startet die gegebene Anwendung.
    ///
    /// Gibt die ID des gestarteten Prozesses zurück.
    async fn launch_application(&self, app: &Application) -> SystemResult<ProcessId>;
    
    // Zukünftig könnten hier Methoden zur Prozessüberwachung, -beendung etc. folgen.
    // async fn terminate_process(&self, pid: ProcessId) -> SystemResult<()>;
    // async fn get_process_status(&self, pid: ProcessId) -> SystemResult<ProcessStatus>;
}

/// Eine Standardimplementierung des `ProcessManager` Traits.
/// Verwendet `std::process::Command` zum Starten von Prozessen.
#[derive(Default, Debug, Clone)]
pub struct DefaultProcessManager;

impl DefaultProcessManager {
    pub fn new() -> Self {
        DefaultProcessManager
    }
}

#[async_trait]
impl ProcessManager for DefaultProcessManager {
    async fn launch_application(&self, app: &Application) -> SystemResult<ProcessId> {
        info!(
            app_id = %app.id,
            app_name = %app.name,
            executable = %app.executable_path,
            "Starte Anwendung"
        );

        let mut command = Command::new(&app.executable_path);

        if let Some(args) = &app.arguments {
            command.args(args);
        }

        if let Some(cwd) = &app.working_directory {
            // Überprüfen, ob das Arbeitsverzeichnis existiert, bevor es gesetzt wird.
            if !std::path::Path::new(cwd).is_dir() {
                return Err(SystemError::ProcessManagementError {
                    command: app.executable_path.clone(),
                    reason: format!("Arbeitsverzeichnis '{}' existiert nicht oder ist keine Directory.", cwd),
                });
            }
            command.current_dir(cwd);
        }
        
        command.stdin(Stdio::null());
        command.stdout(Stdio::null());
        command.stderr(Stdio::null());

        #[cfg(unix)]
        {
            use std::os::unix::process::CommandExt;
            // SAFETY: setsid ist ein gültiger Systemaufruf und wird hier korrekt verwendet,
            // um den neuen Prozess von der aktuellen Session zu lösen.
            unsafe {
                command.pre_exec(|| {
                    if libc::setsid() == -1 {
                        // Fehlerfall für setsid, sollte eigentlich nicht passieren, wenn pre_exec erfolgreich ist
                        // und der Prozess noch nicht gestartet wurde. Dennoch defensive Fehlerbehandlung.
                        return Err(std::io::Error::last_os_error());
                    }
                    Ok(())
                });
            }
        }
        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            const DETACHED_PROCESS: u32 = 0x00000008;
            command.creation_flags(DETACHED_PROCESS);
        }

        let child: Child = command.spawn().map_err(|e| {
            SystemError::ProcessManagementError {
                command: app.executable_path.clone(),
                reason: format!("Fehler beim Starten des Prozesses: {}", e),
            }
        })?;
        
        let pid = child.id();
        info!(%pid, app_name = %app.name, "Anwendung erfolgreich gestartet.");
        
        Ok(pid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use novade_domain::entities::application::ApplicationType;
    use novade_core::types::Version; // Für Application::version
    use tokio; // Für async tests

    // Hilfsfunktion zum Erstellen einer Test-Anwendung
    fn create_test_app(name: &str, command: &str, args: Option<Vec<String>>, cwd: Option<String>) -> Application {
        Application {
            id: NovaId::new(),
            name: name.to_string(),
            display_name: Some(name.to_string()),
            executable_path: command.to_string(),
            arguments: args,
            working_directory: cwd,
            icon_name: None,
            app_type: ApplicationType::Cli, // Für Tests einfacher
            categories: None,
            keywords: None,
            description: None,
            version: Some(Version::new(1,0,0)),
        }
    }

    #[tokio::test]
    async fn test_launch_simple_command_exists() {
        let manager = DefaultProcessManager::new();
        let cmd_name;
        let args;

        if cfg!(unix) {
            cmd_name = "true";
            args = None;
        } else if cfg!(windows) {
            cmd_name = "cmd";
            args = Some(vec!["/C".to_string(), "exit".to_string(), "0".to_string()]);
        } else {
            panic!("Unsupported OS for test_launch_simple_command_exists");
        }
        
        let app = create_test_app("TestTrue", cmd_name, args, None);
        let result = manager.launch_application(&app).await;
        
        assert!(result.is_ok(), "Fehler beim Starten von '{}': {:?}", cmd_name, result.err());
        if let Ok(pid) = result {
            assert!(pid > 0);
            // Unter Unix wird der 'true'-Prozess sehr schnell beenden.
            // Unter Windows wird 'cmd /C exit 0' ebenfalls schnell beenden.
            // Ein Kill ist hier nicht zwingend nötig und würde Tests komplexer machen.
        }
    }
    
    #[tokio::test]
    async fn test_launch_command_not_found() {
        let manager = DefaultProcessManager::new();
        let app = create_test_app("NonExistent", "/hopefully/this/command/does/not/exist", None, None);
        let result = manager.launch_application(&app).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            SystemError::ProcessManagementError { command, reason } => {
                assert_eq!(command, "/hopefully/this/command/does/not/exist");
                assert!(!reason.is_empty());
            }
            e => panic!("Unerwarteter Fehlertyp: {:?}", e),
        }
    }
    
    #[tokio::test]
    async fn test_launch_with_invalid_cwd() {
        let manager = DefaultProcessManager::new();
        let cmd_name = if cfg!(unix) { "true" } else { "cmd" };
        let args = if cfg!(windows) { Some(vec!["/C".to_string(), "exit".to_string(), "0".to_string()]) } else { None };
        let app = create_test_app("InvalidCwdTest", cmd_name, args, Some("/path/that/most/definitely/does/not/exist".to_string()));
        let result = manager.launch_application(&app).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            SystemError::ProcessManagementError { command: _, reason } => {
                 assert!(reason.contains("Arbeitsverzeichnis"));
                 assert!(reason.contains("existiert nicht"));
            }
            e => panic!("Unerwarteter Fehlertyp: {:?}", e),
        }
    }
}
