use crate::EM_HOME;
use dialoguer::theme::ColorfulTheme;
use std::path::PathBuf;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct Emulator {
    name: String,
    lists: Vec<String>,
    executable: PathBuf,
}

impl Emulator {
    pub fn new() -> Self {
        let executable = EM_HOME.clone().join("emulator/emulator");
        let list = duct::cmd::<_, _>(&executable, ["-list-avds"])
            .stderr_capture()
            .read();

        let lists = list
            .unwrap_or_default()
            .split('\n')
            .filter_map(|x| {
                if x.is_empty() || x.contains('|') {
                    None
                } else {
                    Some(x.trim().to_string())
                }
            })
            .collect::<Vec<_>>();

        Self {
            name: String::default(),
            lists,
            executable,
        }
    }

    pub fn select(mut self) -> Result<Self> {
        if self.lists.len() > 1 {
            let avd = dialoguer::FuzzySelect::with_theme(&ColorfulTheme::default())
                .items(&self.lists)
                .with_prompt("Type to find if you have bunch of useless emulators")
                .interact()?;
            self.name = (*self.lists[avd]).parse().unwrap();
        } else {
            self.name = (*self.lists.first().unwrap()).parse().unwrap();
        }

        Ok(self)
    }

    pub fn launch(&self) -> Result<()> {
        duct::cmd(&self.executable, ["-avd", &self.name])
            .before_spawn(|cmd| {
                use std::os::windows::process::CommandExt;
                {
                    const CREATE_NEW_PROCESS_GROUP: u32 = 0x00000200;
                    const CREATE_NO_WINDOW: u32 = 0x08000000;
                    cmd.creation_flags(CREATE_NEW_PROCESS_GROUP | CREATE_NO_WINDOW);
                }
                Ok(())
            })
            .stderr_null()
            .stdin_null()
            .stdout_null()
            .start()?;

        Ok(())
    }
}
