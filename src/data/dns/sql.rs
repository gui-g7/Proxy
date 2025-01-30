use lazy_static::lazy_static;
use rusqlite::{params, Connection, Result};
use std::num::NonZero;
use std::sync::{Arc, Mutex};
use lru::LruCache;
use crossbeam_channel::{Sender, bounded};
use serde_json;

#[derive(Debug)]
pub struct DnsRecord {
    pub ip: String,
    pub service: String,
}

lazy_static! {
    static ref DB_CONN: Arc<Mutex<Connection>> = {
        let conn = Connection::open("dns_registry.db")
            .expect("Falha ao abrir o banco de dados");
        
        conn.execute_batch(
            r#"
            PRAGMA journal_mode = WAL;
            PRAGMA synchronous = NORMAL;
            
            CREATE TABLE IF NOT EXISTS services (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL UNIQUE
            );
            
            CREATE TABLE IF NOT EXISTS ip_mappings (
                ip TEXT PRIMARY KEY,
                service_id INTEGER,
                timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY(service_id) REFERENCES services(id)
            );
            "#
        ).expect("Falha na criação das tabelas");
        
        Arc::new(Mutex::new(conn))
    };
    
    static ref CACHE: Mutex<LruCache<String, String>> = Mutex::new(LruCache::new(NonZero::new(10_000).unwrap()));
    
    static ref WRITE_QUEUE: Sender<DnsRecord> = {
        let (sender, receiver) = bounded(1000);
        
        std::thread::spawn(move || {
            let mut conn = DB_CONN.lock().unwrap();
            
            while let Ok(record) = receiver.recv() {
                if let Err(e) = process_write(&mut conn, &record) {
                    eprintln!("Erro na escrita: {}", e);
                }
            }
        });
        
        sender
    };
}

#[allow(unused)]
impl DnsRecord {
    pub fn new(ip: &str, service: &str) -> Self {
        Self {
            ip: ip.to_string(),
            service: service.to_string(),
        }
    }

    pub fn find_or_fetch(ip: &str) -> Option<String> {
        if let Some(service) = check_cache(ip) {
            return Some(service);
        }
        
        match get_from_database(ip) {
            Ok(Some(service)) => {
                update_cache(ip, &service);
                Some(service)
            },
            _ => {
                if let Ok(service) = fetch_from_external_api(ip) {
                    let record = DnsRecord::new(ip, &service);
                    let _ = WRITE_QUEUE.send(record);
                    update_cache(ip, &service);
                    Some(service)
                } else {
                    None
                }
            }
        }
    }
}

// Funções internas
fn check_cache(ip: &str) -> Option<String> {
    CACHE.lock().unwrap().get(ip).cloned()
}

fn update_cache(ip: &str, service: &str) {
    CACHE.lock().unwrap().put(ip.to_string(), service.to_string());
}

fn get_from_database(ip: &str) -> Result<Option<String>> {
    let conn = DB_CONN.lock().unwrap();
    
    let mut stmt = conn.prepare(
        "SELECT s.name 
        FROM ip_mappings i
        JOIN services s ON i.service_id = s.id
        WHERE i.ip = ?1"
    )?;
    
    let mut rows = stmt.query_map(params![ip], |row| row.get(0))?;
    
    if let Some(row) = rows.next() {
        Ok(Some(row?))
    } else {
        Ok(None)
    }
}

fn process_write(conn: &mut Connection, record: &DnsRecord) -> Result<()> {
    let tx = conn.transaction()?;
    
    tx.execute(
        "INSERT OR IGNORE INTO services (name) VALUES (?1)",
        params![record.service]
    )?;
    
    let service_id: i64 = tx.query_row(
        "SELECT id FROM services WHERE name = ?1",
        params![record.service],
        |row| row.get(0)
    )?;
    
    tx.execute(
        "INSERT OR REPLACE INTO ip_mappings (ip, service_id) 
        VALUES (?1, ?2)",
        params![record.ip, service_id]
    )?;
    
    tx.commit()
}

fn fetch_from_external_api(ip: &str) -> Result<String, reqwest::Error> {
    reqwest::blocking::get(&format!("https://ipapi.co/{}/json/", ip))?
        .json::<serde_json::Value>()
        .map(|json| {
            json["country"].as_str()
                .map(|s| s.to_string())
                .unwrap_or_else(|| "unknown".to_string())
        })
}
