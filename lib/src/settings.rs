use serde::{ Deserialize, Serialize };

#[derive(Deserialize, Serialize)]
pub struct Settings {
	pub git_executable: String
}

impl Default for Settings {
	fn default() -> Self {
		let git_executable = {
			#[cfg(not(target_os = "windows"))]
			let executable = "git";
			#[cfg(target_os = "windows")]
			let executable = "git.exe";

			executable.into()
		};

		Self { git_executable }
	}
}
