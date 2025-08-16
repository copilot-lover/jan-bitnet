use std::{collections::HashMap, sync::Arc};

use crate::core::utils::download::DownloadManagerState;
use rand::{distributions::Alphanumeric, Rng};
use rmcp::{service::RunningService, RoleClient};
use tokio::task::JoinHandle;

/// Server handle type for managing the proxy server lifecycle
pub type ServerHandle = JoinHandle<Result<(), Box<dyn std::error::Error + Send + Sync>>>;
use tokio::{process::Child, sync::Mutex};
use crate::core::utils::extensions::inference_llamacpp_extension::server::SessionInfo as LlamaSessionInfo;
use crate::core::utils::extensions::inference_bitnet_extension::server::SessionInfo as BitnetSessionInfo;

pub struct LLamaBackendSession {
    pub child: Child,
    pub info: LlamaSessionInfo,
}

pub struct BitnetBackendSession {
    pub child: Child,
    pub info: BitnetSessionInfo,
}

#[derive(Default)]
pub struct AppState {
    pub app_token: Option<String>,
    pub mcp_servers: Arc<Mutex<HashMap<String, RunningService<RoleClient, ()>>>>,
    pub download_manager: Arc<Mutex<DownloadManagerState>>,
    pub mcp_restart_counts: Arc<Mutex<HashMap<String, u32>>>,
    pub mcp_active_servers: Arc<Mutex<HashMap<String, serde_json::Value>>>,
    pub mcp_successfully_connected: Arc<Mutex<HashMap<String, bool>>>,
    pub server_handle: Arc<Mutex<Option<ServerHandle>>>,
    pub llama_server_process: Arc<Mutex<HashMap<i32, LLamaBackendSession>>>,
    pub bitnet_server_process: Arc<Mutex<HashMap<i32, BitnetBackendSession>>>,
}
pub fn generate_app_token() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect()
}
