use serde::{Deserialize, Serialize};

// structure for parsing data from JSON file.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Data {
    #[serde(rename(deserialize = "development"))]
    pub development: Development,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Development {
    #[serde(rename(deserialize = "address"))]
    pub address: String,
    #[serde(rename(deserialize = "port"))]
    pub port: String,
    #[serde(rename(deserialize = "workers"))]
    pub workers: i32,
    #[serde(rename(deserialize = "database"))]
    pub database: Database,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Database {
    #[serde(rename(deserialize = "adapter"))]
    pub adapter: String,
    #[serde(rename(deserialize = "db_name"))]
    pub db_name: String,
    #[serde(rename(deserialize = "pool"))]
    pub pool: i32,
}

// let _json = r#"
//     {
//         "development":
//         {
//             "address": "localhost",
//             "port": "8000",
//             "workers": 4,
//             "database": {
//                 "adapter": "postgresql",
//                 "db_name": "blog_development",
//                 "pool": 5
//             }
//         }
//     }"#;

// let _toml_str = r#"
//         [development]
//         address = "localhost"
//         port = "8000"
//         workers = 4
//
//         [development.database]
//         adapter = "postgresql"
//         db_name = "blog_development"
//         pool = 5
//     "#;

pub fn read_config() -> String {
    // Convert the JSON/TOML string back to a Data.
    let data: Data = toml::from_str(include_str!("../../src/data/database.toml")).unwrap();
    // let data: Data = serde_json::from_str(include_str!("../../src/data/database.json")).unwrap();
    // println!("{:#?}", data);

    // Convert the Data to a JSON string.
    let serialized = serde_json::to_string_pretty(&data).unwrap();
    // println!("serialized = {}", serialized);

    serialized
}

#[cfg(test)]
mod tests {
    use crate::db_config::Data;

    #[test]
    fn it1() {
        let data: Data = serde_json::from_str(include_str!("../../src/data/database.json")).unwrap();
        assert_eq!(data.development.port, "8000");
    }

    #[test]
    fn it2() {
        let data: Data = toml::from_str(include_str!("../../src/data/database.toml")).unwrap();
        assert_eq!(data.development.port, "8000");
    }
}
