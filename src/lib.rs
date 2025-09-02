use klyx::LanguageServerId;
use klyx_extension_api::{
    self as klyx,
    serde_json::{json, Value},
    Result,
};
use std::collections::HashMap;

struct CppLSP {
    cached_settings: Option<Value>,
}

impl klyx::Extension for CppLSP {
    fn new() -> Self {
        Self {
            cached_settings: None,
        }
    }

    fn uninstall(&mut self) {
        self.cached_settings = None;
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &klyx::Worktree,
    ) -> Result<klyx::Command> {
        if let Some(path) = worktree.which("clangd") {
            return Ok(self.create_clangd_command(path));
        }

        Err("clangd not found. Please install clangd using: sudo apt install clangd".into())
    }

    fn language_server_initialization_options(
        &mut self,
        _server_id: &LanguageServerId,
        _worktree: &klyx::Worktree,
    ) -> Result<Option<Value>> {
        let init_options = json!({
            "clangdFileStatus": true,
            "usePlaceholders": true,
            "completeUnimported": true,
            "semanticHighlighting": true,
            "compilationDatabaseChanges": true
        });

        Ok(Some(init_options))
    }

    fn language_server_workspace_configuration(
        &mut self,
        _server_id: &LanguageServerId,
        _worktree: &klyx::Worktree,
    ) -> Result<Option<Value>> {
        if let Some(ref settings) = self.cached_settings {
            return Ok(Some(settings.clone()));
        }

        let config = json!({
            "clangd": {
                "arguments": [
                    "--background-index",
                    "--clang-tidy",
                    "--completion-style=detailed",
                    "--header-insertion=iwyu",
                    "--pch-storage=memory",
                    "--function-arg-placeholders",
                    "--log=verbose"
                ],
                "fallbackFlags": [
                    "-std=c++17",
                    "-Wall",
                    "-Wextra"
                ]
            }
        });

        self.cached_settings = Some(config.clone());
        Ok(Some(config))
    }
}

impl CppLSP {
    fn create_clangd_command(&self, clangd_path: String) -> klyx::Command {
        let args = vec![
            "--background-index".into(),
            "--clang-tidy".into(),
            "--completion-style=detailed".into(),
            "--header-insertion=iwyu".into(),
            "--pch-storage=memory".into(),
            "--function-arg-placeholders".into(),
            "--pretty".into(),
        ];

        let mut env = HashMap::new();
        env.insert("CLANGD_FLAGS".into(), "--background-index".into());

        klyx::Command {
            command: clangd_path,
            args,
            env: env.into_iter().collect(),
        }
    }
}

klyx::register_extension!(CppLSP);
