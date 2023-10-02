# andotp-import

This is a simple crate to read encrypted backups created by the [andOTP](https://github.com/andOTP/andOTP/) Android app.

## Usage

Create an encrypted backup in the app and copy it to where you need it.
The code below will prompt for a password, open the backup file and print all current codes.

```
fn main() {
	let pw = rpassword::prompt_password("password: ").unwrap();
	let accts = andotp_import::read_from_file("./otp_accounts_2023-10-02_18-58-25.json.aes", &pw).unwrap();

	for (acct, totp) in accts {
		println!("{} {}", acct.label, totp.generate_current().unwrap());
	}
}
```

The full API documentation is available at [docs.rs](https://docs.rs/andotp-import/).
See [totp-rs](https://docs.rs/totp-rs/) for details on the provided TOTP values.

## License

Copyright © 2023 FliegendeWurst

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.