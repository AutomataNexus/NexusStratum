use std::fmt;
use std::time::{Duration, SystemTime};

use crate::sri::sha256_digest;

/// Default maximum age for a CSRF token before it is considered expired.
///
/// Two hours is long enough for a user to fill out and submit a form while
/// still bounding the window in which a leaked token remains useful.
pub const DEFAULT_MAX_AGE: Duration = Duration::from_secs(2 * 60 * 60);

const NONCE_LEN: usize = 16;
const TS_LEN: usize = 8;
const MAC_LEN: usize = 32;
const PAYLOAD_LEN: usize = NONCE_LEN + TS_LEN;
const TOKEN_BYTES: usize = PAYLOAD_LEN + MAC_LEN;
const PREFIX: &str = "csrf_";

/// Why a CSRF token failed verification.
///
/// Distinguishing these lets the server log a forged/tampered token
/// (`BadSignature`) differently from an honestly stale one (`Expired`), while
/// still rejecting both.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CsrfError {
    /// The token was not the expected shape (bad prefix, length, or hex).
    Malformed,
    /// The HMAC did not match — the token was forged or tampered with.
    BadSignature,
    /// The signature was valid but the token is older than the max age.
    Expired,
}

impl fmt::Display for CsrfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            Self::Malformed => "CSRF token is malformed",
            Self::BadSignature => "CSRF token signature is invalid",
            Self::Expired => "CSRF token has expired",
        };
        f.write_str(msg)
    }
}

impl std::error::Error for CsrfError {}

/// A server-held secret key for issuing and verifying CSRF tokens.
///
/// This is the proper, stateless way to do **time-based** CSRF protection
/// (OWASP "HMAC-based Token Pattern"). Each token is self-contained:
///
/// ```text
/// payload = random_nonce(16) || issued_at(u64 BE seconds)
/// token   = "csrf_" + hex(payload || HMAC-SHA256(secret, payload))
/// ```
///
/// The issue time lives **inside** the HMAC-signed payload, so a client cannot
/// extend a token's lifetime without invalidating its signature. Verification
/// checks the signature in constant time *before* trusting the timestamp, then
/// rejects tokens older than a caller-chosen max age.
///
/// # Key management
///
/// The secret must be generated once (e.g. [`CsrfKey::generate`]), persisted
/// out-of-band (environment variable, secret manager), and loaded at startup
/// via [`CsrfKey::from_secret`]. It is **per-application and long-lived**, not
/// per-request. Rotating the secret invalidates all outstanding tokens.
#[derive(Clone)]
pub struct CsrfKey {
    secret: Vec<u8>,
}

impl CsrfKey {
    /// Build a key from an existing secret (loaded from config/secret manager).
    ///
    /// Any length is accepted; HMAC handles key sizing internally. Use at least
    /// 32 bytes of high-entropy secret in production.
    pub fn from_secret(secret: impl Into<Vec<u8>>) -> Self {
        Self {
            secret: secret.into(),
        }
    }

    /// Generate a fresh 32-byte signing secret from the OS CSPRNG.
    ///
    /// Only do this once per application and persist the result — generating a
    /// new key on every boot would invalidate every previously issued token.
    pub fn generate() -> Self {
        let mut secret = vec![0u8; 32];
        getrandom::fill(&mut secret).expect("OS CSPRNG should be available");
        Self { secret }
    }

    /// Issue a new signed CSRF token stamped with the current time, **not**
    /// bound to any session.
    ///
    /// Suitable for forms with no authenticated session (e.g. a public contact
    /// or login form). For authenticated users prefer [`issue_for`](Self::issue_for),
    /// which binds the token to the session so it cannot be replayed by another
    /// user.
    pub fn issue(&self) -> CsrfToken {
        self.mint(SystemTime::now(), &[])
    }

    /// Issue a signed CSRF token bound to a session/user identifier.
    ///
    /// `session_id` is mixed into the signature (it is **not** stored in the
    /// token). At verification time the server supplies the current session's
    /// id via [`verify_for`](Self::verify_for); a token minted for one session
    /// fails verification for any other. Use a stable, per-session value such
    /// as the session cookie id or a per-session secret.
    pub fn issue_for(&self, session_id: &[u8]) -> CsrfToken {
        self.mint(SystemTime::now(), session_id)
    }

    /// Issue a token with an explicit issue time (and optional binding).
    ///
    /// Primarily useful in tests; production code should use [`issue`](Self::issue)
    /// or [`issue_for`](Self::issue_for).
    pub fn issue_at(&self, issued_at: SystemTime) -> CsrfToken {
        self.mint(issued_at, &[])
    }

    fn mint(&self, issued_at: SystemTime, session_id: &[u8]) -> CsrfToken {
        let mut nonce = [0u8; NONCE_LEN];
        getrandom::fill(&mut nonce).expect("OS CSPRNG should be available");

        // The session id is signed but not embedded: the server re-supplies it
        // at verify time, so it never travels in the token.
        let mut payload = Vec::with_capacity(PAYLOAD_LEN);
        payload.extend_from_slice(&nonce);
        payload.extend_from_slice(&unix_secs(issued_at).to_be_bytes());

        let mac = self.mac(&payload, session_id);

        let mut raw = Vec::with_capacity(TOKEN_BYTES);
        raw.extend_from_slice(&payload);
        raw.extend_from_slice(&mac);

        CsrfToken {
            value: format!("{PREFIX}{}", hex_encode(&raw)),
        }
    }

    /// HMAC over the public payload plus the (server-side) session binding.
    fn mac(&self, payload: &[u8], session_id: &[u8]) -> [u8; MAC_LEN] {
        let mut message = Vec::with_capacity(payload.len() + session_id.len());
        message.extend_from_slice(payload);
        message.extend_from_slice(session_id);
        hmac_sha256(&self.secret, &message)
    }

    /// Verify a token that was issued **without** session binding.
    ///
    /// Returns `Ok(())` only when the token is well-formed, its HMAC matches
    /// this key (checked in constant time), **and** it was issued within
    /// `max_age`. The timestamp is read only after the signature is confirmed,
    /// so expiry is always evaluated on trusted data.
    pub fn verify(&self, submitted: &str, max_age: Duration) -> Result<(), CsrfError> {
        self.verify_for(submitted, &[], max_age)
    }

    /// Verify a token against a session/user binding.
    ///
    /// `session_id` must be the same value passed to [`issue_for`](Self::issue_for);
    /// a token minted for a different session yields [`CsrfError::BadSignature`].
    pub fn verify_for(
        &self,
        submitted: &str,
        session_id: &[u8],
        max_age: Duration,
    ) -> Result<(), CsrfError> {
        let hex = submitted.strip_prefix(PREFIX).ok_or(CsrfError::Malformed)?;
        let raw = hex_decode(hex).ok_or(CsrfError::Malformed)?;
        if raw.len() != TOKEN_BYTES {
            return Err(CsrfError::Malformed);
        }

        let (payload, mac) = raw.split_at(PAYLOAD_LEN);
        let expected = self.mac(payload, session_id);
        if !constant_time_eq(&expected, mac) {
            return Err(CsrfError::BadSignature);
        }

        // Signature verified — the timestamp in `payload` is now trustworthy.
        let mut ts_bytes = [0u8; TS_LEN];
        ts_bytes.copy_from_slice(&payload[NONCE_LEN..]);
        let issued_at = u64::from_be_bytes(ts_bytes);
        let now = unix_secs(SystemTime::now());
        // saturating_sub treats a future issue time (clock skew) as age 0.
        if now.saturating_sub(issued_at) > max_age.as_secs() {
            return Err(CsrfError::Expired);
        }

        Ok(())
    }

    /// Convenience boolean wrapper around [`verify`](Self::verify).
    pub fn is_valid(&self, submitted: &str, max_age: Duration) -> bool {
        self.verify(submitted, max_age).is_ok()
    }
}

impl fmt::Debug for CsrfKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Never print the secret.
        f.debug_struct("CsrfKey").finish_non_exhaustive()
    }
}

/// A CSRF token value, ready to embed in a form.
///
/// For time-based protection, mint these with [`CsrfKey::issue`] and verify
/// them with [`CsrfKey::verify`]. The legacy [`new`](Self::new) /
/// [`generate`](Self::generate) / [`validate`](Self::validate) methods support
/// the stateful synchronizer-token pattern (server stores the token in the
/// session and compares against it) and perform **no** expiry checking.
#[derive(Debug, Clone)]
pub struct CsrfToken {
    value: String,
}

impl CsrfToken {
    /// Create a token from a known value (e.g. one stored in a server session).
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }

    /// Generate a random (unsigned) token for the synchronizer-token pattern.
    ///
    /// This token carries no issue time and must be stored server-side and
    /// compared via [`validate`](Self::validate). For stateless, time-based
    /// verification use [`CsrfKey::issue`] instead.
    pub fn generate() -> Self {
        let token = format!("{PREFIX}{}", crate::csp::generate_secure_token(32));
        Self { value: token }
    }

    /// Get the token value.
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Generate a hidden input HTML string for embedding in forms.
    pub fn to_hidden_input(&self) -> String {
        format!(
            "<input type=\"hidden\" name=\"_csrf\" value=\"{}\" />",
            stratum_core::security::escape_attr(&self.value)
        )
    }

    /// Validate that a submitted token matches this token (constant-time).
    ///
    /// Value comparison only — no expiry. For time-based verification, issue
    /// and verify with [`CsrfKey`].
    pub fn validate(&self, submitted: &str) -> bool {
        constant_time_eq(self.value.as_bytes(), submitted.as_bytes())
    }
}

impl fmt::Display for CsrfToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl PartialEq for CsrfToken {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for CsrfToken {}

/// Seconds since the Unix epoch; 0 if the time predates the epoch.
fn unix_secs(t: SystemTime) -> u64 {
    t.duration_since(SystemTime::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

/// HMAC-SHA256 (RFC 2104) built on the crate's SHA-256.
fn hmac_sha256(key: &[u8], msg: &[u8]) -> [u8; MAC_LEN] {
    const BLOCK: usize = 64;

    // Keys longer than the block size are hashed down first.
    let mut k = if key.len() > BLOCK {
        sha256_digest(key)
    } else {
        key.to_vec()
    };
    k.resize(BLOCK, 0);

    let mut ipad = [0x36u8; BLOCK];
    let mut opad = [0x5cu8; BLOCK];
    for i in 0..BLOCK {
        ipad[i] ^= k[i];
        opad[i] ^= k[i];
    }

    let mut inner = Vec::with_capacity(BLOCK + msg.len());
    inner.extend_from_slice(&ipad);
    inner.extend_from_slice(msg);
    let inner_hash = sha256_digest(&inner);

    let mut outer = Vec::with_capacity(BLOCK + MAC_LEN);
    outer.extend_from_slice(&opad);
    outer.extend_from_slice(&inner_hash);
    let digest = sha256_digest(&outer);

    let mut out = [0u8; MAC_LEN];
    out.copy_from_slice(&digest);
    out
}

fn hex_encode(data: &[u8]) -> String {
    let mut s = String::with_capacity(data.len() * 2);
    for b in data {
        s.push(char::from_digit((b >> 4) as u32, 16).unwrap());
        s.push(char::from_digit((b & 0x0f) as u32, 16).unwrap());
    }
    s
}

fn hex_decode(s: &str) -> Option<Vec<u8>> {
    if s.len() % 2 != 0 {
        return None;
    }
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(bytes.len() / 2);
    let mut i = 0;
    while i < bytes.len() {
        let hi = (bytes[i] as char).to_digit(16)?;
        let lo = (bytes[i + 1] as char).to_digit(16)?;
        out.push(((hi << 4) | lo) as u8);
        i += 2;
    }
    Some(out)
}

/// Constant-time byte comparison to prevent timing attacks.
///
/// Both the content and the length comparison are constant-time.
/// The longer slice is always fully iterated to prevent length leakage.
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    let len_eq = a.len() == b.len();
    // Always iterate the longer of the two to prevent length-based timing
    let max_len = a.len().max(b.len());
    let mut result = 0u8;
    for i in 0..max_len {
        let x = if i < a.len() { a[i] } else { 0 };
        let y = if i < b.len() { b[i] } else { 0 };
        result |= x ^ y;
    }
    result == 0 && len_eq
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Synchronizer-token (legacy) pattern -------------------------------

    #[test]
    fn csrf_token_validation() {
        let token = CsrfToken::new("secret-token-123");
        assert!(token.validate("secret-token-123"));
        assert!(!token.validate("wrong-token"));
        assert!(!token.validate("secret-token-12"));
    }

    #[test]
    fn csrf_token_hidden_input() {
        let token = CsrfToken::new("abc123");
        let html = token.to_hidden_input();
        assert!(html.contains("type=\"hidden\""));
        assert!(html.contains("name=\"_csrf\""));
        assert!(html.contains("value=\"abc123\""));
    }

    #[test]
    fn csrf_token_generate_unique() {
        let a = CsrfToken::generate();
        let b = CsrfToken::generate();
        assert_ne!(a, b);
        assert!(a.value().starts_with("csrf_"));
    }

    #[test]
    fn csrf_token_generate_sufficient_entropy() {
        let token = CsrfToken::generate();
        // "csrf_" prefix + 64 hex chars (32 bytes)
        assert!(token.value().len() >= 69);
    }

    #[test]
    fn csrf_token_xss_prevention() {
        let token = CsrfToken::new("test\" onclick=\"alert(1)");
        let html = token.to_hidden_input();
        // Quotes are escaped so the injected attribute can't break out of the value
        assert!(html.contains("&quot;"));
        assert!(!html.contains("\" onclick=\""));
    }

    #[test]
    fn constant_time_eq_works() {
        assert!(constant_time_eq(b"hello", b"hello"));
        assert!(!constant_time_eq(b"hello", b"world"));
        assert!(!constant_time_eq(b"hello", b"hell"));
    }

    // --- HMAC primitive ----------------------------------------------------

    #[test]
    fn hmac_sha256_rfc4231_case1() {
        // RFC 4231 Test Case 1: key = 0x0b * 20, data = "Hi There".
        let key = [0x0bu8; 20];
        let mac = hmac_sha256(&key, b"Hi There");
        let hex = hex_encode(&mac);
        assert_eq!(
            hex,
            "b0344c61d8db38535ca8afceaf0bf12b881dc200c9833da726e9376c2e32cff7"
        );
    }

    #[test]
    fn hmac_sha256_long_key_is_hashed() {
        // Key longer than the 64-byte block must still produce a 32-byte MAC.
        let key = vec![0xaau8; 131];
        let mac = hmac_sha256(&key, b"Test Using Larger Than Block-Size Key");
        assert_eq!(mac.len(), MAC_LEN);
    }

    #[test]
    fn hex_round_trip() {
        let data = [0x00u8, 0x0f, 0xa5, 0xff, 0x10];
        assert_eq!(hex_decode(&hex_encode(&data)).unwrap(), data);
        assert!(hex_decode("xyz").is_none());
        assert!(hex_decode("abc").is_none()); // odd length
    }

    // --- Time-based signed tokens (the proper pattern) ---------------------

    #[test]
    fn signed_token_round_trips() {
        let key = CsrfKey::generate();
        let token = key.issue();
        assert!(token.value().starts_with("csrf_"));
        assert_eq!(key.verify(token.value(), DEFAULT_MAX_AGE), Ok(()));
        assert!(key.is_valid(token.value(), DEFAULT_MAX_AGE));
    }

    #[test]
    fn signed_token_rejected_by_different_key() {
        let issuer = CsrfKey::generate();
        let attacker = CsrfKey::generate();
        let token = issuer.issue();
        assert_eq!(
            attacker.verify(token.value(), DEFAULT_MAX_AGE),
            Err(CsrfError::BadSignature)
        );
    }

    #[test]
    fn tampered_token_fails_signature() {
        let key = CsrfKey::generate();
        let token = key.issue();
        // Flip the last hex nibble of the token.
        let mut v = token.value().to_string();
        let last = v.pop().unwrap();
        let flipped = if last == 'a' { 'b' } else { 'a' };
        v.push(flipped);
        assert_eq!(key.verify(&v, DEFAULT_MAX_AGE), Err(CsrfError::BadSignature));
    }

    #[test]
    fn timestamp_tampering_is_caught() {
        // An attacker who rewrites the embedded timestamp invalidates the MAC,
        // so they cannot extend an expired token's life.
        let key = CsrfKey::generate();
        let old = SystemTime::now()
            .checked_sub(Duration::from_secs(10 * 60 * 60))
            .unwrap();
        let token = key.issue_at(old);
        // Expired as-is...
        assert_eq!(key.verify(token.value(), DEFAULT_MAX_AGE), Err(CsrfError::Expired));

        // ...and rebuilding the payload with a "now" timestamp under a key the
        // attacker doesn't hold cannot produce a valid MAC.
        let forged = CsrfKey::generate().issue(); // attacker's own key
        assert_eq!(
            key.verify(forged.value(), DEFAULT_MAX_AGE),
            Err(CsrfError::BadSignature)
        );
    }

    #[test]
    fn expired_signed_token_is_rejected() {
        let key = CsrfKey::generate();
        let issued = SystemTime::now()
            .checked_sub(Duration::from_secs(3 * 60 * 60))
            .unwrap();
        let token = key.issue_at(issued);
        assert_eq!(
            key.verify(token.value(), DEFAULT_MAX_AGE),
            Err(CsrfError::Expired)
        );
        // A longer window accepts the same token.
        assert_eq!(
            key.verify(token.value(), Duration::from_secs(4 * 60 * 60)),
            Ok(())
        );
    }

    #[test]
    fn future_issue_time_is_not_expired() {
        // Clock skew: token issued slightly in the future is still fresh.
        let key = CsrfKey::generate();
        let future = SystemTime::now() + Duration::from_secs(120);
        let token = key.issue_at(future);
        assert_eq!(key.verify(token.value(), DEFAULT_MAX_AGE), Ok(()));
    }

    #[test]
    fn malformed_tokens_are_rejected() {
        let key = CsrfKey::generate();
        assert_eq!(key.verify("", DEFAULT_MAX_AGE), Err(CsrfError::Malformed));
        assert_eq!(key.verify("nope", DEFAULT_MAX_AGE), Err(CsrfError::Malformed));
        assert_eq!(
            key.verify("csrf_zz", DEFAULT_MAX_AGE),
            Err(CsrfError::Malformed)
        );
        // Right prefix and hex, wrong length.
        assert_eq!(
            key.verify("csrf_abcd", DEFAULT_MAX_AGE),
            Err(CsrfError::Malformed)
        );
    }

    #[test]
    fn session_bound_token_accepts_same_session() {
        let key = CsrfKey::generate();
        let token = key.issue_for(b"session-alice");
        assert_eq!(
            key.verify_for(token.value(), b"session-alice", DEFAULT_MAX_AGE),
            Ok(())
        );
    }

    #[test]
    fn session_bound_token_rejects_other_session() {
        // Token transplant: Alice's token replayed under Bob's session fails.
        let key = CsrfKey::generate();
        let token = key.issue_for(b"session-alice");
        assert_eq!(
            key.verify_for(token.value(), b"session-bob", DEFAULT_MAX_AGE),
            Err(CsrfError::BadSignature)
        );
        // ...and the unbound verify path rejects a bound token too.
        assert_eq!(
            key.verify(token.value(), DEFAULT_MAX_AGE),
            Err(CsrfError::BadSignature)
        );
    }

    #[test]
    fn unbound_token_does_not_verify_as_bound() {
        let key = CsrfKey::generate();
        let token = key.issue(); // no binding
        assert_eq!(key.verify(token.value(), DEFAULT_MAX_AGE), Ok(()));
        assert_eq!(
            key.verify_for(token.value(), b"some-session", DEFAULT_MAX_AGE),
            Err(CsrfError::BadSignature)
        );
    }

    #[test]
    fn from_secret_is_deterministic_across_instances() {
        // Two keys built from the same secret must verify each other's tokens
        // (e.g. across a load-balanced fleet sharing one secret).
        let a = CsrfKey::from_secret(b"shared-application-secret-bytes!".to_vec());
        let b = CsrfKey::from_secret(b"shared-application-secret-bytes!".to_vec());
        let token = a.issue();
        assert_eq!(b.verify(token.value(), DEFAULT_MAX_AGE), Ok(()));
    }

    #[test]
    fn debug_does_not_leak_secret() {
        let key = CsrfKey::from_secret(b"super-secret-value".to_vec());
        let dbg = format!("{key:?}");
        assert!(!dbg.contains("super-secret-value"));
    }
}
