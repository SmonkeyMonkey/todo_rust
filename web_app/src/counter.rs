use std::env;

use redis::RedisError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Counter {
    pub count: i32,
}

impl Counter {
    fn get_redis_url() -> String {
        let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
        redis_url   
    }
    
    pub fn save(self) -> Result<(), RedisError> {
        let serialized = serde_yaml::to_vec(&self).unwrap();

        let client = match redis::Client::open(Counter::get_redis_url()) {
            Ok(client) => client,
            Err(error) => return Err(error),
        };
        let mut conn = match client.get_connection() {
            Ok(client) => client,
            Err(error) => return Err(error),
        };
        match redis::cmd("SET")
            .arg("COUNTER")
            .arg(serialized)
            .query::<Vec<u8>>(&mut conn)
        {
            Ok(_) => Ok(()),
            Err(error) => Err(error),
        }
    }
    pub fn load() -> Result<Counter, RedisError> {
        let client = match redis::Client::open(Counter::get_redis_url()) {
            Ok(client) => client,
            Err(error) => return Err(error),
        };
        let mut conn = match client.get_connection() {
            Ok(conn) => conn,
            Err(error) => return Err(error),
        };
        let byte_data: Vec<u8> = match redis::cmd("GET").arg("COUNTER").query(&mut conn) {
            Ok(data) => data,
            Err(error) => return Err(error),
        };
        Ok(serde_yaml::from_slice(&byte_data).unwrap())
    }
}
