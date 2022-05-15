use clap::Parser;
use lettre::{smtp::authentication::Credentials, SmtpClient, Transport};
use lettre_email::EmailBuilder;
use whoami::{hostname, username};

#[derive(Parser, Debug)]
#[clap(
    name = "Email on Login",
    version = "1.0.0",
    author = "Kawasaki Daichi <kyudai.kawasaki@gmail.com>",
    about = "Send email on login with task scheduler"
)]
struct Opts {
    /// Email to send login notification
    #[clap(short, long)]
    email: String,

    /// Application-specific password
    #[clap(short, long)]
    password: String,
}

fn main() {
    // Get login time (Task Scheduler triggered)
    let datetime = chrono::Local::now();
    let format_datetime = datetime.format("%Y年%m月%d日 %H:%M:%S");

    // Get login hostname and username
    let hostname = hostname();
    let username = username();

    // Set email parameters
    let opts = Opts::parse();
    let str_address = opts.email;
    let str_password = opts.password;
    let str_subject = format!("【セキュリティ通知】 {hostname} へのログイン検知");
    let str_body = format!(
        "【セキュリティ通知】 {hostname} へのログイン検知\n
        {format_datetime} に <{username}> がログインしました。\n
        身に覚えがない場合は直ちに PC の LAN ケーブルを取り外してください。\n
        すぐに対応できない場合は管理者に連絡し、パスワードを変更してください。"
    );

    let email = EmailBuilder::new()
        .to(str_address.clone())
        .from(str_address.clone())
        .subject(str_subject)
        .text(str_body)
        .build()
        .unwrap();

    let credentials = Credentials::new(str_address, str_password);
    let mut mailer = SmtpClient::new_simple("smtp.gmail.com")
        .unwrap()
        .credentials(credentials)
        .transport();

    let res = mailer.send(email.into());
    match res {
        Ok(_) => println!("sending email was succeeded."),
        Err(err) => println!("{err}"),
    }
}
