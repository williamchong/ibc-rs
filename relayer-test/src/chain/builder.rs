use super::util;
use super::manager::ChainManager;

#[derive(Debug)]
pub struct ChainBuilder {
    pub command_path: String,

    pub base_store_dir: String,
}

impl ChainBuilder {
    pub fn new(
        command_path: &str,
        base_store_dir: &str,
    ) -> Self {
        Self {
            command_path: command_path.to_string(),
            base_store_dir: base_store_dir.to_string(),
        }
    }

    pub fn new_chain(
        &self
    ) -> ChainManager
    {
        let chain_num = util::random_u32();
        let chain_id = format!("ibc-{:x}", chain_num);

        let rpc_port = util::random_unused_tcp_port();
        let grpc_port = util::random_unused_tcp_port();

        let home_path = format!("{}/{}", self.base_store_dir, chain_id);

        ChainManager::new(
            self.command_path.clone(),
            chain_id,
            home_path,
            rpc_port,
            grpc_port
        )
    }
}