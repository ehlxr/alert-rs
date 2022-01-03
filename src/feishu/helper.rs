use crate::feishu::sdk::Sdk;

pub struct UserHelper {
    sdk: Sdk,
    // cache map[string]string
}

impl UserHelper {
    pub fn new(sdk: Sdk) -> Self {
        Self { sdk }
    }

    pub fn get_open_ids(&self) {}
}
