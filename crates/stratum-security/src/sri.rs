/// Subresource Integrity (SRI) hash generation.
///
/// Generates SRI hashes for external resources (scripts, stylesheets)
/// to ensure they haven't been tampered with.
///
/// # Example
///
/// ```
/// use stratum_security::sri::SriHash;
///
/// let hash = SriHash::sha256(b"alert('hello')");
/// assert!(hash.to_integrity_attr().starts_with("sha256-"));
/// ```
/// An SRI hash for a resource.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SriHash {
    algorithm: SriAlgorithm,
    digest: Vec<u8>,
}

/// Supported SRI hash algorithms.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SriAlgorithm {
    Sha256,
    Sha384,
    Sha512,
}

impl SriAlgorithm {
    fn prefix(&self) -> &'static str {
        match self {
            Self::Sha256 => "sha256",
            Self::Sha384 => "sha384",
            Self::Sha512 => "sha512",
        }
    }
}

impl SriHash {
    /// Compute a SHA-256 SRI hash of the given content.
    ///
    /// Uses a minimal built-in SHA-256 to avoid pulling in a heavy crypto
    /// dependency. For production use with SHA-384/512, provide the digest
    /// directly via [`SriHash::from_digest`].
    pub fn sha256(content: &[u8]) -> Self {
        Self {
            algorithm: SriAlgorithm::Sha256,
            digest: sha256_digest(content),
        }
    }

    /// Create an SRI hash from a pre-computed digest.
    ///
    /// Use this when you have a digest from an external crypto library
    /// (e.g., `ring`, `sha2`).
    pub fn from_digest(algorithm: SriAlgorithm, digest: Vec<u8>) -> Self {
        Self { algorithm, digest }
    }

    /// Generate the `integrity` attribute value (e.g., `"sha256-base64hash"`).
    pub fn to_integrity_attr(&self) -> String {
        format!(
            "{}-{}",
            self.algorithm.prefix(),
            base64_encode(&self.digest)
        )
    }

    /// Generate a full HTML integrity attribute string.
    pub fn to_html_attr(&self) -> String {
        format!("integrity=\"{}\"", self.to_integrity_attr())
    }
}

/// Minimal Base64 encoder (no external dependency).
fn base64_encode(data: &[u8]) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();
    let chunks = data.chunks(3);
    for chunk in chunks {
        let b0 = chunk[0] as u32;
        let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };
        let triple = (b0 << 16) | (b1 << 8) | b2;

        result.push(CHARS[((triple >> 18) & 0x3F) as usize] as char);
        result.push(CHARS[((triple >> 12) & 0x3F) as usize] as char);
        if chunk.len() > 1 {
            result.push(CHARS[((triple >> 6) & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
        if chunk.len() > 2 {
            result.push(CHARS[(triple & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
    }
    result
}

/// Minimal SHA-256 implementation (FIPS 180-4).
/// Only used for SRI — not for general cryptographic purposes.
fn sha256_digest(data: &[u8]) -> Vec<u8> {
    let mut h: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
    ];

    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
        0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
        0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
        0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
        0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
        0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
        0xc67178f2,
    ];

    // Padding
    let bit_len = (data.len() as u64) * 8;
    let mut msg = data.to_vec();
    msg.push(0x80);
    while (msg.len() % 64) != 56 {
        msg.push(0);
    }
    msg.extend_from_slice(&bit_len.to_be_bytes());

    // Process 512-bit blocks
    for block in msg.chunks(64) {
        let mut w = [0u32; 64];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([
                block[i * 4],
                block[i * 4 + 1],
                block[i * 4 + 2],
                block[i * 4 + 3],
            ]);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16]
                .wrapping_add(s0)
                .wrapping_add(w[i - 7])
                .wrapping_add(s1);
        }

        let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut hh] = h;

        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let temp1 = hh
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(K[i])
                .wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);

            hh = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }

        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
        h[5] = h[5].wrapping_add(f);
        h[6] = h[6].wrapping_add(g);
        h[7] = h[7].wrapping_add(hh);
    }

    h.iter().flat_map(|v| v.to_be_bytes()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sha256_empty_string() {
        // SHA-256 of empty string is well-known
        let hash = SriHash::sha256(b"");
        let hex: String = hash.digest.iter().map(|b| format!("{:02x}", b)).collect();
        assert_eq!(
            hex,
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn sha256_hello() {
        let hash = SriHash::sha256(b"hello");
        let hex: String = hash.digest.iter().map(|b| format!("{:02x}", b)).collect();
        assert_eq!(
            hex,
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        );
    }

    #[test]
    fn integrity_attr_format() {
        let hash = SriHash::sha256(b"test");
        let attr = hash.to_integrity_attr();
        assert!(attr.starts_with("sha256-"));
    }

    #[test]
    fn html_attr_format() {
        let hash = SriHash::sha256(b"test");
        let attr = hash.to_html_attr();
        assert!(attr.starts_with("integrity=\"sha256-"));
        assert!(attr.ends_with("\""));
    }

    #[test]
    fn from_digest() {
        let digest = vec![0u8; 32];
        let hash = SriHash::from_digest(SriAlgorithm::Sha384, digest);
        assert!(hash.to_integrity_attr().starts_with("sha384-"));
    }

    #[test]
    fn base64_encode_basic() {
        assert_eq!(base64_encode(b"hello"), "aGVsbG8=");
        assert_eq!(base64_encode(b""), "");
        assert_eq!(base64_encode(b"ab"), "YWI=");
        assert_eq!(base64_encode(b"abc"), "YWJj");
    }
}
