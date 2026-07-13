//! Cached DBC message lookup used by the frame filters.

use std::collections::HashMap;

/// Cached message info for filtering.
/// Public for use by pro crate's multi-DBC filter.
pub struct DbcMessageInfo {
    pub name: String,
    pub signal_names: Vec<String>,
}

/// Message info cache type alias for convenience.
pub type DbcMessageCache = HashMap<(u32, bool), DbcMessageInfo>;

/// Build message info cache from a DBC for fast lookups.
/// This is the core function that both base and pro versions use.
pub fn build_message_cache_from_dbc(dbc: &dbc_rs::Dbc) -> DbcMessageCache {
    let mut cache = HashMap::new();

    for msg in dbc.messages().iter() {
        let is_extended = msg.id() > 0x7FF;
        let signal_names: Vec<String> = msg
            .signals()
            .iter()
            .map(|s| s.name().to_lowercase())
            .collect();

        cache.insert(
            (msg.id(), is_extended),
            DbcMessageInfo {
                name: msg.name().to_lowercase(),
                signal_names,
            },
        );
    }

    cache
}
