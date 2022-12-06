use std::collections::HashMap;

#[derive(Clone)]
pub struct AppConfig {
    user_name: String,
    user_pem: String,
    base_url: String,
}

impl AppConfig {
    pub fn from(f: HashMap<String, String>) -> AppConfig {
        let mut cfg = AppConfig { user_name: String::from(""), user_pem: String::from(""), base_url: String::from("") };
        if f.contains_key("user_name") {
            cfg.user_name = f["user_name"].clone();
        }
        if f.contains_key("user_pem") {
            cfg.user_pem = f["user_pem"].clone();
        }
        if f.contains_key("base_url") {
            cfg.base_url = f["base_url"].clone();
        }

        return cfg;
    }

    pub fn user_name(&self) -> String {
        self.user_name.clone()
    }

    pub fn user_pem(&self) -> String {
        self.user_pem.clone()
    }
    
    pub fn base_url(&self) -> String {
        self.base_url.clone()
    }

    pub fn self_url(&self) -> String {
        return format!("https://{}/users/{}", self.base_url, self.user_name);
    }

    pub fn profile_url(&self) -> String {
        return format!("https://{}/~{}", self.base_url, self.user_name);
    }
}