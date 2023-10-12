fn main() {
	let pw = rpassword::prompt_password("password: ").unwrap();
	let accts = andotp_import::read_from_file("./otp_accounts.json.aes", &pw).unwrap();

	for (acct, totp) in accts {
		println!("{} {} {}", acct.issuer, acct.label, totp.generate_current().unwrap());
	}
}
