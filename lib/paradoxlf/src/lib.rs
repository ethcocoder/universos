//! ParadoxLF - Latent Form Compression Library
//!
//! Compression library for ParadoxOS state vectors.
//! Implements memory as potential energy (LAW 8).

#![warn(missing_docs, rust_2018_idioms)]

use flate2::write::{GzEncoder, GzDecoder};
use flate2::Compression;
use std::io::Write;

/// Compress data using ParadoxLF algorithm
///
/// # Arguments
///
/// * `data` - Raw data to compress
///
/// # Returns
///
/// Compressed data
pub fn compress(data: &[u8]) -> Vec<u8> {
    // TODO: Implement actual ParadoxLF compression algorithm
    // For now, use gzip as placeholder
    
    let mut encoder = GzEncoder::new(Vec::new(), Compression::best());
    encoder.write_all(data).expect("Compression failed");
    encoder.finish().expect("Compression failed")
}

/// Decompress ParadoxLF data
///
/// # Arguments
///
/// * `compressed` - Compressed data
/// * `original_size` - Original uncompressed size (if known)
///
/// # Returns
///
/// Decompressed data
pub fn decompress(compressed: &[u8], _original_size: Option<usize>) -> Result<Vec<u8>, String> {
    // TODO: Implement actual ParadoxLF decompression
    
    let mut decoder = GzDecoder::new(Vec::new());
    decoder.write_all(compressed)
        .map_err(|e| format!("Decompression failed: {:?}", e))?;
    decoder.finish()
        .map_err(|e| format!("Decompression failed: {:?}", e))
}

/// Calculate compression ratio
pub fn compression_ratio(original_size: usize, compressed_size: usize) -> f64 {
    original_size as f64 / compressed_size as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compress_decompress() {
        let data = vec![42u8; 1000];
        let original_size = data.len();
        
        let compressed = compress(&data);
        
        // Compression should reduce size for repetitive data
        assert!(compressed.len() < data.len(), 
                "Compressed size {} should be less than original {}", 
                compressed.len(), original_size);
        
        let decompressed = decompress(&compressed, Some(original_size)).unwrap();
        assert_eq!(data, decompressed);
    }

    #[test]
    fn test_compression_ratio() {
        let ratio = compression_ratio(1000, 100);
        assert_eq!(ratio, 10.0);
    }
}
