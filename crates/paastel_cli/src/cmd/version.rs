use std::fmt;

#[derive(Debug)]
pub struct CommitInfo {
    pub short_commit_hash: String,
    pub commit_hash: String,
    pub commit_date: String,
}

#[derive(Debug)]
pub struct VersionInfo {
    pub version: String,
    pub release_channel: Option<String>,
    pub commit_info: Option<CommitInfo>,
}

impl fmt::Display for VersionInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.version)?;

        if let Some(ref ci) = self.commit_info {
            write!(f, " ({} {})", ci.short_commit_hash, ci.commit_date)?;
        };
        Ok(())
    }
}

pub fn version() -> VersionInfo {
    macro_rules! option_env_str {
        ($name:expr) => {
            option_env!($name).map(|s| s.to_string())
        };
    }

    let version = option_env_str!("CFG_RELEASE").unwrap_or_else(|| {
        let minor = env!("CARGO_PKG_VERSION_MINOR").parse::<u8>().unwrap() - 1;
        let patch = env!("CARGO_PKG_VERSION_PATCH").parse::<u8>().unwrap();
        format!("1.{}.{}", minor, patch)
    });

    let release_channel = option_env_str!("CFG_RELEASE_CHANNEL");
    let commit_info =
        option_env_str!("CARGO_COMMIT_HASH").map(|commit_hash| CommitInfo {
            short_commit_hash: option_env_str!("CARGO_COMMIT_SHORT_HASH")
                .unwrap(),
            commit_hash,
            commit_date: option_env_str!("CARGO_COMMIT_DATE").unwrap(),
        });

    VersionInfo {
        version,
        release_channel,
        commit_info,
    }
}

pub fn get_version_string() -> String {
    let version = version();
    let version_string = format!("paastel {version}");
    version_string
}
