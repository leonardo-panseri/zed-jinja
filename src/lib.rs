use std::fs;
use zed_extension_api::{
    self as zed, set_language_server_installation_status, settings::LspSettings, LanguageServerId,
    LanguageServerInstallationStatus, Result,
};

struct JinjaLSPBinary {
    path: String,
    args: Option<Vec<String>>,
}

struct JinjaExtension {
    cached_binary_path: Option<String>,
}

impl JinjaExtension {
    fn language_server_binary(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<JinjaLSPBinary> {
        let binary_settings = LspSettings::for_worktree("jinja-lsp", worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.binary);
        let binary_args = binary_settings
            .as_ref()
            .and_then(|binary_settings| binary_settings.arguments.clone());

        if let Some(path) = binary_settings.and_then(|binary_setting| binary_setting.path) {
            return Ok(JinjaLSPBinary {
                path,
                args: binary_args,
            });
        }

        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).map_or(false, |stat| stat.is_file()) {
                return Ok(JinjaLSPBinary {
                    path: path.clone(),
                    args: binary_args,
                });
            }
        }

        set_language_server_installation_status(
            language_server_id,
            &LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let release = zed::latest_github_release(
            "uros-5/jinja-lsp",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;
        let asset_name = "jinja-lsp.zip";
        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| format!("no asset found matching {:?}", asset_name))?;

        let (platform, arch) = zed::current_platform();
        let binary_name = format!(
            "jinja-lsp-{os}-{arch}{extension}",
            arch = match arch {
                zed::Architecture::Aarch64 => "arm64",
                zed::Architecture::X86 => "?", // jinja-lsp does not support x86
                zed::Architecture::X8664 => "x64",
            },
            os = match platform {
                zed::Os::Mac => "darwin",
                zed::Os::Linux => "linux",
                zed::Os::Windows => "windows",
            },
            extension = match platform {
                zed::Os::Windows => ".exe",
                _ => "",
            }
        );

        let version_dir = format!("jinja-lsp-{}", release.version);
        let binary_path = format!("{version_dir}/{binary_name}");

        if !fs::metadata(&binary_path).map_or(false, |stat| stat.is_file()) {
            set_language_server_installation_status(
                language_server_id,
                &LanguageServerInstallationStatus::Downloading,
            );
            zed::download_file(
                &asset.download_url,
                &version_dir,
                zed::DownloadedFileType::Zip,
            )
            .map_err(|e| format!("failed to download file: {e}"))?;

            zed::make_file_executable(&binary_path)?;

            let entries =
                fs::read_dir(".").map_err(|e| format!("failed to list working directory {e}"))?;
            for entry in entries {
                let entry = entry.map_err(|e| format!("failed to load directory entry {e}"))?;
                if entry.file_name().to_str() != Some(&version_dir) {
                    fs::remove_dir_all(entry.path()).ok();
                }
            }

            fs::metadata(&binary_path)
                .map_err(|_| format!("{platform:?}-{arch:?} not supported"))?;
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(JinjaLSPBinary {
            path: binary_path,
            args: binary_args,
        })
    }
}

impl zed::Extension for JinjaExtension {
    fn new() -> Self {
        JinjaExtension {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let jinja_lsp_binary = self.language_server_binary(language_server_id, worktree)?;
        Ok(zed::Command {
            command: jinja_lsp_binary.path,
            args: jinja_lsp_binary.args.unwrap_or_else(|| Default::default()),
            env: Default::default(),
        })
    }

    fn language_server_initialization_options(
        &mut self,
        server_id: &LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> Result<Option<zed_extension_api::serde_json::Value>> {
        let settings = LspSettings::for_worktree(server_id.as_ref(), worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.initialization_options.clone())
            .unwrap_or_default();
        Ok(Some(settings))
    }

    fn language_server_workspace_configuration(
        &mut self,
        server_id: &LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> Result<Option<zed_extension_api::serde_json::Value>> {
        let settings = LspSettings::for_worktree(server_id.as_ref(), worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.settings.clone())
            .unwrap_or_default();
        Ok(Some(settings))
    }
}

zed::register_extension!(JinjaExtension);
