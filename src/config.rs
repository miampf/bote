use home;
use veilid_core::{ConfigCallbackReturn, FourCC, TypedKeyGroup, TypedSecretGroup};

use crate::error::Error;

/// config_callback() is used by veilid to generate a configuration for veilid at the startup.
pub fn config_callback(key: String) -> ConfigCallbackReturn {
    // HACK: This contains a lot of magic values. Maybe I should write a config file in JSON
    // instead and import it at runtime? (This code was basically copied from vldpipe: https://gitlab.com/vatueil/vldpipe/-/blob/main/src/config.rs?ref_type=heads)
    match key.as_str() {
        "program_name" => Ok(Box::new(String::from("bote"))),
        "namespace" => Ok(Box::<String>::default()),
        "capabilities.disable" => Ok(Box::<Vec<FourCC>>::default()),
        "table_store.directory" => Ok(Box::new(get_veilid_table_store_path()?)),
        "table_store.delete" => Ok(Box::new(false)),
        "block_store.directory" => Ok(Box::new(get_veilid_block_store_path()?)),
        "block_store.delete" => Ok(Box::new(false)),
        "protected_store.allow_insecure_fallback" => Ok(Box::new(true)),
        "protected_store.always_use_insecure_storage" => Ok(Box::new(false)),
        "protected_store.directory" => Ok(Box::new(get_veilid_protected_store_path()?)),
        "protected_store.delete" => Ok(Box::new(false)),
        "protected_store.device_encryption_key_password" => Ok(Box::new("".to_owned())),
        "protected_store.new_device_encryption_key_password" => {
            Ok(Box::new(Option::<String>::None))
        }
        "network.connection_initial_timeout_ms" => Ok(Box::new(2_000u32)),
        "network.connection_inactivity_timeout_ms" => Ok(Box::new(60_000u32)),
        "network.max_connections_per_ip4" => Ok(Box::new(8u32)),
        "network.max_connections_per_ip6_prefix" => Ok(Box::new(8u32)),
        "network.max_connections_per_ip6_prefix_size" => Ok(Box::new(56u32)),
        "network.max_connection_frequency_per_min" => Ok(Box::new(8u32)),
        "network.client_whitelist_timeout_ms" => Ok(Box::new(300_000u32)),
        "network.reverse_connection_receipt_time_ms" => Ok(Box::new(5_000u32)),
        "network.hole_punch_receipt_time_ms" => Ok(Box::new(5_000u32)),
        "network.network_key_password" => Ok(Box::new(Option::<String>::None)),
        "network.routing_table.node_id" => Ok(Box::new(TypedKeyGroup::new())),
        "network.routing_table.node_id_secret" => Ok(Box::new(TypedSecretGroup::new())),
        "network.routing_table.bootstrap" => Ok(Box::new(vec!["bootstrap.veilid.net".to_string()])),
        "network.routing_table.limit_over_attached" => Ok(Box::new(64u32)),
        "network.routing_table.limit_fully_attached" => Ok(Box::new(32u32)),
        "network.routing_table.limit_attached_strong" => Ok(Box::new(16u32)),
        "network.routing_table.limit_attached_good" => Ok(Box::new(8u32)),
        "network.routing_table.limit_attached_weak" => Ok(Box::new(4u32)),
        "network.rpc.concurrency" => Ok(Box::new(2u32)),
        "network.rpc.queue_size" => Ok(Box::new(1024u32)),
        "network.rpc.max_timestamp_behind_ms" => Ok(Box::new(Some(10_000u32))),
        "network.rpc.max_timestamp_ahead_ms" => Ok(Box::new(Some(10_000u32))),
        "network.rpc.timeout_ms" => Ok(Box::new(5_000u32)),
        "network.rpc.max_route_hop_count" => Ok(Box::new(4u8)),
        "network.rpc.default_route_hop_count" => Ok(Box::new(1u8)),
        "network.dht.max_find_node_count" => Ok(Box::new(20u32)),
        "network.dht.resolve_node_timeout_ms" => Ok(Box::new(10_000u32)),
        "network.dht.resolve_node_count" => Ok(Box::new(1u32)),
        "network.dht.resolve_node_fanout" => Ok(Box::new(4u32)),
        "network.dht.get_value_timeout_ms" => Ok(Box::new(10_000u32)),
        "network.dht.get_value_count" => Ok(Box::new(3u32)),
        "network.dht.get_value_fanout" => Ok(Box::new(4u32)),
        "network.dht.set_value_timeout_ms" => Ok(Box::new(10_000u32)),
        "network.dht.set_value_count" => Ok(Box::new(5u32)),
        "network.dht.set_value_fanout" => Ok(Box::new(4u32)),
        "network.dht.min_peer_count" => Ok(Box::new(20u32)),
        "network.dht.min_peer_refresh_time_ms" => Ok(Box::new(60_000u32)),
        "network.dht.validate_dial_info_receipt_time_ms" => Ok(Box::new(5_000u32)),
        "network.dht.local_subkey_cache_size" => Ok(Box::new(128u32)),
        "network.dht.local_max_subkey_cache_memory_mb" => Ok(Box::new(256u32)),
        "network.dht.remote_subkey_cache_size" => Ok(Box::new(1024u32)),
        "network.dht.remote_max_records" => Ok(Box::new(4096u32)),
        "network.dht.remote_max_subkey_cache_memory_mb" => Ok(Box::new(64u32)),
        "network.dht.remote_max_storage_space_mb" => Ok(Box::new(64u32)),
        "network.upnp" => Ok(Box::new(true)),
        "network.detect_address_changes" => Ok(Box::new(true)),
        "network.restricted_nat_retries" => Ok(Box::new(3u32)),
        "network.tls.certificate_path" => Ok(Box::new(get_veilid_certfile_path()?)),
        "network.tls.private_key_path" => Ok(Box::new(get_veilid_keyfile_path()?)),
        "network.tls.connection_initial_timeout_ms" => Ok(Box::new(2_000u32)),
        "network.application.https.enabled" => Ok(Box::new(false)),
        "network.application.https.listen_address" => Ok(Box::new("".to_owned())),
        "network.application.https.path" => Ok(Box::new(String::from("app"))),
        "network.application.https.url" => Ok(Box::new(Option::<String>::None)),
        "network.application.http.enabled" => Ok(Box::new(false)),
        "network.application.http.listen_address" => Ok(Box::new("".to_owned())),
        "network.application.http.path" => Ok(Box::new(String::from("app"))),
        "network.application.http.url" => Ok(Box::new(Option::<String>::None)),
        "network.protocol.udp.enabled" => Ok(Box::new(true)),
        "network.protocol.udp.socket_pool_size" => Ok(Box::new(16u32)),
        "network.protocol.udp.listen_address" => Ok(Box::new("".to_owned())),
        "network.protocol.udp.public_address" => Ok(Box::new(Option::<String>::None)),
        "network.protocol.tcp.connect" => Ok(Box::new(true)),
        "network.protocol.tcp.listen" => Ok(Box::new(true)),
        "network.protocol.tcp.max_connections" => Ok(Box::new(32u32)),
        "network.protocol.tcp.listen_address" => Ok(Box::new("".to_owned())),
        "network.protocol.tcp.public_address" => Ok(Box::new(Option::<String>::None)),
        "network.protocol.ws.connect" => Ok(Box::new(false)),
        "network.protocol.ws.listen" => Ok(Box::new(false)),
        "network.protocol.ws.max_connections" => Ok(Box::new(16u32)),
        "network.protocol.ws.listen_address" => Ok(Box::new("".to_owned())),
        "network.protocol.ws.path" => Ok(Box::new(String::from("ws"))),
        "network.protocol.ws.url" => Ok(Box::new(Option::<String>::None)),
        "network.protocol.wss.connect" => Ok(Box::new(false)),
        "network.protocol.wss.listen" => Ok(Box::new(false)),
        "network.protocol.wss.max_connections" => Ok(Box::new(16u32)),
        "network.protocol.wss.listen_address" => Ok(Box::new("".to_owned())),
        "network.protocol.wss.path" => Ok(Box::new(String::from("ws"))),
        "network.protocol.wss.url" => Ok(Box::new(Option::<String>::None)),
        _ => Err(Error::NotFound { whats_missing: key }.into()),
    }
}

/// get_app_directory() returns the directory where bote keeps files (~/.bote)
pub fn get_app_directory() -> Result<String, Error> {
    match home::home_dir() {
        Some(path) => {
            if let Some(path_string) = path.to_str() {
                return Ok(path_string.to_string() + "/.bote");
            }
            Err(Error::Conversion {
                from: "PathBuf".to_string(),
                into: "&str".to_string(),
            })
        }
        None => Err(Error::HomeDir),
    }
}

/// get_veilid_table_store_path() returns the path to the veilid table store.
pub fn get_veilid_table_store_path() -> Result<String, Error> {
    Ok(get_app_directory()? + "/table_store")
}

/// get_veilid_protected_store_path() returns the path to the veilid protected store.
pub fn get_veilid_protected_store_path() -> Result<String, Error> {
    Ok(get_app_directory()? + "/protected_store")
}

/// get_veilid_block_store_path() returns the path to the veilid block store.
pub fn get_veilid_block_store_path() -> Result<String, Error> {
    Ok(get_app_directory()? + "/block_store")
}

/// get_veilid_certfile_path() returns the path to the veilid certificate.
pub fn get_veilid_certfile_path() -> Result<String, Error> {
    Ok(get_app_directory()? + "/certfile")
}

/// get_veilid_keyfile_path() returns the path to the veilid keyfile.
pub fn get_veilid_keyfile_path() -> Result<String, Error> {
    Ok(get_app_directory()? + "/keyfile")
}

/// get_cache_directory() returns the path to the cache directory.
pub fn get_cache_directory() -> Result<String, Error> {
    Ok(get_app_directory()? + "/.cache")
}
