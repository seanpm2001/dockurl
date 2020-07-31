use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;

pub struct NetworkConfig {
    pub endpoints_config: EndpointsConfig,
}
impl NetworkConfig {
    pub fn consume(self) -> HashMap<String, Value> {
        let mut map = HashMap::new();
        map.insert(
            "EndpointsConfig".to_string(),
            json!(self.endpoints_config.consume()),
        );
        map
    }
}

pub struct EndpointsConfig {
    pub endpoint_settings: EndpointSettings,
}
impl EndpointsConfig {
    pub fn consume(self) -> HashMap<String, Value> {
        let mut map = HashMap::new();
        map.insert(
            "EndpointSettings".to_string(),
            json!(self.endpoint_settings.consume()),
        );
        map
    }
}

pub struct EndpointSettings {
    fields: HashMap<String, Value>,
}
impl EndpointSettings {
    pub fn new() -> Self {
        Self {
            fields: HashMap::new(),
        }
    }

    pub fn consume(self) -> HashMap<String, Value> {
        self.fields
    }

    pub fn ipam_config(&mut self) {
        // todo
    }

    pub fn links(&mut self) {
        // todo
    }

    pub fn alias(&mut self, alias: &str) {
        if let Some(aliases) = self.fields.get_mut("Aliases") {
            if let Some(aliases) = aliases.as_array_mut() {
                aliases.push(json!(alias));
            }
        } else {
            self.fields
                .insert("Aliases".to_string(), json!(vec![alias]));
        }
    }

    pub fn network_id(&mut self, network_id: &str) {
        self.fields
            .insert("NetworkID".to_string(), json!(network_id));
    }

    pub fn endpoint_id(&mut self) {
        // todo
    }

    pub fn gateway(&mut self) {
        // todo
    }

    pub fn ip_address(&mut self) {
        // todo
    }

    pub fn ip_prefix_len(&mut self) {
        // todo
    }

    pub fn ipv6_gateway(&mut self) {
        // todo
    }

    pub fn global_ipv6_address(&mut self) {
        // todo
    }

    pub fn global_ipv6_prefix_len(&mut self) {
        // todo
    }

    pub fn mac_address(&mut self) {
        // todo
    }

    pub fn driver_opts(&mut self) {
        // todo
    }
}
