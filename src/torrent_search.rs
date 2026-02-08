use magneto::{Magneto, SearchRequest, Category, OrderBy};
use magneto::search_providers::{PirateBay, SearchProvider};
use crate::logger::{log_info, log_debug, log_error};
use std::error::Error;

/// Torrent search result with formatted display fields
#[derive(Debug, Clone)]
pub struct TorrentResult {
    pub name: String,
    pub uploaded: String,
    pub size: String,
    pub seeders: u32,
    pub leechers: u32,
    pub magnet_link: String,
}

/// Search The Pirate Bay for movies matching the query
pub async fn search_torrents(query: &str) -> Result<Vec<TorrentResult>, Box<dyn Error>> {
    log_info(&format!("Torrent search initiated: query=\"{}\", provider=PirateBay", query));
    
    // Initialize magneto with only PirateBay provider to avoid YTS connection issues
    let providers: Vec<Box<dyn SearchProvider>> = vec![Box::new(PirateBay::new())];
    let magneto = Magneto::with_providers(providers);
    
    // Create search request with Movies category filter
    let request = SearchRequest {
        query,
        categories: vec![Category::Movies],
        number_of_results: 5,
        order_by: OrderBy::Seeders,
    };
    
    // Execute search with better error context
    log_debug("Executing torrent search request");
    let results = magneto.search(request).await.map_err(|e| {
        let error_msg = format!("Torrent search failed: {}. This may be due to provider unavailability or network issues.", e);
        log_error(&error_msg);
        Box::<dyn Error>::from(error_msg)
    })?;
    
    log_debug(&format!("Raw search returned {} results", results.len()));
    
    // Sort by seeders (descending) and take top 5
    let mut sorted_results = results;
    sorted_results.sort_by(|a, b| b.seeders.cmp(&a.seeders));
    sorted_results.truncate(5);
    
    log_info(&format!("Torrent search returned {} results", sorted_results.len()));
    
    // Convert to TorrentResult format
    let torrent_results: Vec<TorrentResult> = sorted_results
        .into_iter()
        .enumerate()
        .map(|(idx, torrent)| {
            // Format size from bytes to human-readable
            let size = format_size(torrent.size_bytes);
            
            let result = TorrentResult {
                name: torrent.name.clone(),
                uploaded: "Unknown".to_string(), // magneto doesn't provide upload date
                size,
                seeders: torrent.seeders,
                leechers: torrent.peers, // peers field represents leechers
                magnet_link: torrent.magnet_link.clone(),
            };
            
            log_debug(&format!(
                "Result {}: \"{}\" seeds={} size={}",
                idx, result.name, result.seeders, result.size
            ));
            
            result
        })
        .collect();
    
    Ok(torrent_results)
}

/// Format bytes into human-readable size string
fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    
    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

/// Open a magnet link using the OS default handler
pub fn open_magnet_link(magnet_link: &str) -> Result<(), Box<dyn Error>> {
    log_info(&format!("Opening magnet link: {}", magnet_link));
    
    #[cfg(target_os = "linux")]
    let command = "xdg-open";
    
    #[cfg(target_os = "macos")]
    let command = "open";
    
    #[cfg(target_os = "windows")]
    let command = "start";
    
    std::process::Command::new(command)
        .arg(magnet_link)
        .spawn()
        .map_err(|e| {
            log_error(&format!("Failed to open magnet link: {}", e));
            e
        })?;
    
    log_info("Magnet link opened successfully");
    Ok(())
}
