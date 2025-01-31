use redis::{Commands, Connection, RedisResult};
use std::sync::Mutex;

#[allow(unused)]
pub struct RedisDB {
    conn: Mutex<Connection>,
}

#[allow(unused)]
impl RedisDB {
    pub fn new(redis_url: &str) -> RedisResult<Self> {
        let client = redis::Client::open(redis_url)?;
        let conn = client.get_connection()?;
        Ok(RedisDB {
            conn: Mutex::new(conn),
        })
    }

    pub fn store_packet(&self, protocol: &str, data: &str) -> RedisResult<()> {
        let mut conn = self.conn.lock().unwrap();
        let key = format!("packet:{}", protocol);
        
        let _: i64 = conn.lpush(key, data)?; // Explicitamos o tipo esperado
        
        Ok(())
    }    
}

