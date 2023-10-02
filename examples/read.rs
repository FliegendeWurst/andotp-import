fn main() {
	let pw = rpassword::prompt_password("password: ").unwrap();
	let accts = andotp_import::read_from_file("./otp_accounts_2023-10-02_18-58-25.json.aes", &pw).unwrap();

	for (acct, totp) in accts {
		println!("{} {}", acct.label, totp.generate_current().unwrap());
	}
}
