# andotp-import

This is a simple crate to read encrypted backups created by the [andOTP](https://github.com/andOTP/andOTP/) Android app.

## Usage

Create an encrypted backup in the app and copy it to where you need it.
The code below will prompt for a password, open the backup file and print all current TOTP codes.

```rust
fn main() {
	let pw = rpassword::prompt_password("password: ").unwrap();
	let accts = andotp_import::read_from_file("./otp_accounts_2023-10-02_18-58-25.json.aes", &pw).unwrap();

	for (acct, totp) in accts {
		println!("{} {}", acct.label, totp.generate_current().unwrap());
	}
}
```

The full API documentation is available at [docs.rs](https://docs.rs/andotp-import/).

## License

MIT License, see [`LICENSE`](./LICENSE)
