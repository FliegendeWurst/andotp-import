use std::{
	fs,
	io::{BufRead, Cursor, Read},
	num::NonZeroU32,
	path::Path,
};

use byteorder::{BigEndian, ReadBytesExt};
use ring::{
	aead::{Aad, LessSafeKey, Nonce, UnboundKey, AES_256_GCM},
	digest, pbkdf2,
};
use serde_derive::Deserialize;
use serde_json::Value;
use totp_rs::{Algorithm, Secret, SecretParseError, TotpUrlError, TOTP};

static PBKDF2_ALG: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA1;

pub fn read_from_file<P: AsRef<Path>>(path: P, password: &str) -> Result<Vec<(Account, TOTP)>, Error> {
	let data = fs::read(path)?;
	read_from_bytes(&data, password)
}

pub fn read_from_bytes(bytes: &[u8], password: &str) -> Result<Vec<(Account, TOTP)>, Error> {
	let mut file_content = Cursor::new(bytes);

	let iterations = file_content.read_u32::<BigEndian>()?;
	let mut iv = [0; 12];
	file_content.read_exact(&mut iv)?;

	let mut key = [0; digest::SHA256_OUTPUT_LEN];
	pbkdf2::derive(
		PBKDF2_ALG,
		NonZeroU32::new(iterations).ok_or(Error::ZeroIterations)?,
		&iv,
		password.as_bytes(),
		&mut key,
	);

	let mut encrypt_iv = [0; 12];
	file_content.read_exact(&mut encrypt_iv)?;

	// TODO: replace with remaining_slice() once the cursor_remaining feature is stable
	let mut encrypted: Vec<_> = file_content.fill_buf().unwrap().to_vec();

	let key = UnboundKey::new(&AES_256_GCM, &key)?;
	let key = LessSafeKey::new(key);
	let decrypted = key.open_in_place(Nonce::assume_unique_for_key(encrypt_iv), Aad::empty(), &mut encrypted)?;
	let data = String::from_utf8_lossy(decrypted);
	let value: Vec<Account> = serde_json::from_str(&data)?;
	let mut totps = vec![];
	for acct in value {
		if acct.totp_type != "TOTP" {
			// TODO: implement other types
			continue;
		}
		let totp = TOTP::new(
			match acct.algorithm.as_str() {
				"SHA1" => Algorithm::SHA1,
				"SHA256" => Algorithm::SHA256,
				"SHA512" => Algorithm::SHA512,
				_ => return Err(Error::UnknownAlgorithm(acct.algorithm)),
			},
			acct.digits,
			1,
			acct.period,
			Secret::Encoded(acct.secret.clone()).to_bytes()?,
		)?;
		totps.push((acct, totp));
	}
	Ok(totps)
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("I/O error")]
	Io(#[from] std::io::Error),
	#[error("invalid iterations count of zero")]
	ZeroIterations,
	#[error("parse error `{0}`")]
	SecretParseError(SecretParseError),
	#[error("JSON parse error")]
	JsonParseError(#[from] serde_json::Error),
	#[error("cryptography error")]
	Cryptography(ring::error::Unspecified),
	#[error("TOTP import error")]
	TotpError(#[from] TotpUrlError),
	#[error("unknown algorithm `{0}`")]
	UnknownAlgorithm(String),
}

impl From<ring::error::Unspecified> for Error {
	fn from(value: ring::error::Unspecified) -> Self {
		Error::Cryptography(value)
	}
}

impl From<SecretParseError> for Error {
	fn from(value: SecretParseError) -> Self {
		Error::SecretParseError(value)
	}
}

#[derive(Deserialize, Debug)]
pub struct Account {
	pub secret: String,
	pub issuer: String,
	/// Label for this entry. This is displayed underneath the current code in the andOTP app.
	pub label: String,
	pub digits: usize,
	#[serde(rename = "type")]
	/// Type of account. Currently only TOTP is supported, other types are silently ignored.
	pub totp_type: String,
	pub algorithm: String,
	/// Thumbnail ID. Either "Default" or a name. Full list of possible values: https://github.com/andOTP/andOTP/tree/master/app/src/main/res/drawable
	pub thumbnail: String,
	pub last_used: i64,
	pub used_frequency: f64,
	period: u64,
	pub tags: Value,
}
