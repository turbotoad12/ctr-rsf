pub mod rng;

pub const DUMMY_RSF: &str = include_str!("../dummy.rsf");

use serde::{Deserialize, Serialize};
use std::{fs, path::Path};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RsfError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("YAML parse/serialize error: {0}")]
    Yaml(#[from] serde_yaml::Error),
}

/// Top-level RSF structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rsf {
    #[serde(rename = "BasicInfo")]
    pub basic_info: BasicInfo,

    #[serde(rename = "RomFs")]
    pub rom_fs: Option<RomFs>,

    #[serde(rename = "TitleInfo")]
    pub title_info: TitleInfo,

    #[serde(rename = "Option")]
    pub option: OptionSection,

    #[serde(rename = "AccessControlInfo")]
    pub access_control_info: AccessControlInfo,

    #[serde(rename = "SystemControlInfo")]
    pub system_control_info: SystemControlInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicInfo {
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "ProductCode")]
    pub product_code: String,
    #[serde(rename = "Logo")]
    pub logo: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RomFs {
    #[serde(rename = "RootPath")]
    pub root_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleInfo {
    #[serde(rename = "Category")]
    pub category: String,
    #[serde(rename = "UniqueId")]
    pub unique_id: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionSection {
    #[serde(rename = "UseOnSD")]
    pub use_on_sd: bool,
    #[serde(rename = "FreeProductCode")]
    pub free_product_code: bool,
    #[serde(rename = "MediaFootPadding")]
    pub media_foot_padding: bool,
    #[serde(rename = "EnableCrypt")]
    pub enable_crypt: bool,
    #[serde(rename = "EnableCompress")]
    pub enable_compress: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessControlInfo {
    #[serde(rename = "CoreVersion")]
    pub core_version: u32,

    #[serde(rename = "DescVersion")]
    pub desc_version: u32,

    #[serde(rename = "ReleaseKernelMajor")]
    pub release_kernel_major: String,

    #[serde(rename = "ReleaseKernelMinor")]
    pub release_kernel_minor: String,

    #[serde(rename = "UseExtSaveData")]
    pub use_ext_save_data: bool,

    #[serde(rename = "FileSystemAccess")]
    pub file_system_access: Option<Vec<String>>,

    #[serde(rename = "MemoryType")]
    pub memory_type: String,
    #[serde(rename = "SystemMode")]
    pub system_mode: String,
    #[serde(rename = "IdealProcessor")]
    pub ideal_processor: u8,
    #[serde(rename = "AffinityMask")]
    pub affinity_mask: u8,
    #[serde(rename = "Priority")]
    pub priority: u8,
    #[serde(rename = "MaxCpu")]
    pub max_cpu: u8,
    #[serde(rename = "HandleTableSize")]
    pub handle_table_size: u32,
    #[serde(rename = "DisableDebug")]
    pub disable_debug: bool,
    #[serde(rename = "EnableForceDebug")]
    pub enable_force_debug: bool,
    #[serde(rename = "CanWriteSharedPage")]
    pub can_write_shared_page: bool,
    #[serde(rename = "CanUsePrivilegedPriority")]
    pub can_use_privileged_priority: bool,
    #[serde(rename = "CanUseNonAlphabetAndNumber")]
    pub can_use_non_alphabet_and_number: bool,
    #[serde(rename = "PermitMainFunctionArgument")]
    pub permit_main_function_argument: bool,
    #[serde(rename = "CanShareDeviceMemory")]
    pub can_share_device_memory: bool,
    #[serde(rename = "RunnableOnSleep")]
    pub runnable_on_sleep: bool,
    #[serde(rename = "SpecialMemoryArrange")]
    pub special_memory_arrange: bool,

    #[serde(rename = "SystemModeExt")]
    pub system_mode_ext: String,
    #[serde(rename = "CpuSpeed")]
    pub cpu_speed: String,
    #[serde(rename = "EnableL2Cache")]
    pub enable_l2_cache: bool,
    #[serde(rename = "CanAccessCore2")]
    pub can_access_core2: bool,

    #[serde(rename = "IORegisterMapping")]
    pub io_register_mapping: Option<Vec<String>>,
    #[serde(rename = "MemoryMapping")]
    pub memory_mapping: Option<Vec<String>>,

    #[serde(rename = "SystemCallAccess")]
    pub system_call_access: Option<std::collections::HashMap<String, u32>>,

    #[serde(rename = "ServiceAccessControl")]
    pub service_access_control: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemControlInfo {
    #[serde(rename = "SaveDataSize")]
    pub save_data_size: String,
    #[serde(rename = "RemasterVersion")]
    pub remaster_version: u32,
    #[serde(rename = "StackSize")]
    pub stack_size: String,

    #[serde(rename = "Dependency")]
    pub dependency: Option<std::collections::HashMap<String, String>>,
}

/// Load an RSF file from disk.
pub fn load_rsf<P: AsRef<Path>>(path: P) -> Result<Rsf, RsfError> {
    let text = fs::read_to_string(path)?;
    let rsf: Rsf = serde_yaml::from_str(&text)?;
    Ok(rsf)
}

/// Save an RSF structure back to disk.
pub fn save_rsf<P: AsRef<Path>>(path: P, rsf: &Rsf) -> Result<(), RsfError> {
    let text = serde_yaml::to_string(rsf)?;
    fs::write(path, text)?;
    Ok(())
}

/// Returns true only if the product code contains A–Z, a–z, or 0–9.
pub fn is_valid_product_code(code: &str) -> bool {
    code.chars().all(|c| c.is_ascii_alphanumeric())
}

/// Removes all characters that are NOT A–Z, a–z, or 0–9.
pub fn sanitize_product_code(code: &str) -> String {
    code.chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .collect()
}

impl BasicInfo {
    pub fn set_product_code(&mut self, code: &str) -> Result<(), &'static str> {
        if is_valid_product_code(code) {
            self.product_code = code.to_string();
            Ok(())
        } else {
            Err("ProductCode contains invalid characters")
        }
    }
}

use serde_yaml;

/// Load the embedded dummy.rsf into an Rsf struct.
pub fn load_embedded_rsf() -> Result<Rsf, serde_yaml::Error> {
    serde_yaml::from_str(DUMMY_RSF)
}
