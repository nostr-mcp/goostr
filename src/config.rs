use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use serde_yaml::{Mapping, Value};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ExtensionEntry {
    enabled: bool,
    #[serde(rename = "type")]
    kind: String,
    name: String,
    description: String,
    display_name: String,
    timeout: u64,
    bundled: bool,
    available_tools: Vec<String>,
    cmd: String,
    args: Vec<String>,
    envs: BTreeMap<String, String>,
    env_keys: Vec<String>,
}

fn ensure_parent_dir(path: &Path) -> Result<()> {
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir).with_context(|| format!("creating {}", dir.display()))?;
    }
    Ok(())
}

fn read_yaml(path: &Path) -> Result<Value> {
    if !path.exists() {
        return Ok(Value::Mapping(Mapping::new()));
    }
    let s = fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let v: Value =
        serde_yaml::from_str(&s).with_context(|| format!("parsing {}", path.display()))?;
    Ok(v)
}

fn write_yaml(path: &Path, v: &Value) -> Result<()> {
    ensure_parent_dir(path)?;
    let serialized = serde_yaml::to_string(v)?;
    fs::write(path, serialized).with_context(|| format!("writing {}", path.display()))?;
    Ok(())
}

fn as_mapping_mut<'a>(v: &'a mut Value, context: &str) -> Result<&'a mut Mapping> {
    match v {
        Value::Mapping(m) => Ok(m),
        Value::Null => {
            *v = Value::Mapping(Mapping::new());
            if let Value::Mapping(m) = v {
                Ok(m)
            } else {
                Err(anyhow!("failed to create mapping for {context}"))
            }
        }
        _ => Err(anyhow!("expected mapping for {context}")),
    }
}

fn root_mapping_mut(doc: &mut Value) -> Result<&mut Mapping> {
    as_mapping_mut(doc, "root")
}

fn extensions_mapping_mut(root: &mut Mapping) -> Result<&mut Mapping> {
    let exts_val = root
        .entry(Value::String("extensions".to_string()))
        .or_insert(Value::Mapping(Mapping::new()));
    as_mapping_mut(exts_val, "extensions")
}

pub fn root() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".config")
        .join("goose")
}

pub fn path() -> PathBuf {
    root().join("config.yaml")
}

pub fn upsert_stdio_extension(
    id: &str,
    name: &str,
    display_name: &str,
    description: &str,
    cmd: &str,
    enabled: bool,
    timeout: u64,
    bundled: bool,
    available_tools: &[String],
    args: &[String],
    envs: &BTreeMap<String, String>,
    env_keys: &[String],
) -> Result<()> {
    let path = path();
    let mut doc = read_yaml(&path)?;
    let root = root_mapping_mut(&mut doc)?;
    let exts = extensions_mapping_mut(root)?;
    let entry = ExtensionEntry {
        enabled,
        kind: "stdio".to_string(),
        name: name.to_string(),
        description: description.to_string(),
        display_name: display_name.to_string(),
        timeout,
        bundled,
        available_tools: available_tools.to_vec(),
        cmd: cmd.to_string(),
        args: args.to_vec(),
        envs: envs.clone(),
        env_keys: env_keys.to_vec(),
    };
    let entry_val = serde_yaml::to_value(entry)?;
    exts.insert(Value::String(id.to_string()), entry_val);
    write_yaml(&path, &doc)
}

pub fn remove_extension(id: &str) -> Result<bool> {
    let path = path();
    let mut doc = read_yaml(&path)?;
    let root = root_mapping_mut(&mut doc)?;
    let exts_val = root.get_mut(&Value::String("extensions".to_string()));
    let mut changed = false;
    if let Some(Value::Mapping(exts)) = exts_val {
        if exts.remove(&Value::String(id.to_string())).is_some() {
            changed = true;
        }
        if exts.is_empty() {
            root.remove(&Value::String("extensions".to_string()));
            changed = true;
        }
    }
    if changed {
        write_yaml(&path, &doc)?;
    }
    Ok(changed)
}
