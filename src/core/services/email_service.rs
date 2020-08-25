use crate::core::models::users::*;
use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::smtp::error::SmtpResult;
use lettre::smtp::ConnectionReuseParameters;
use lettre::smtp::SmtpTransport;
use lettre::{ClientSecurity, ClientTlsParameters, SmtpClient, Transport};
use lettre_email::{Email, EmailBuilder};
use native_tls::{Protocol, TlsConnector};
use std::env;

fn get_credentials() -> Credentials {
    let smtp_username = env::var("SMTP_USERNAME").expect("SMTP_USERNAME must be set");
    let smtp_password = env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be set");
    Credentials::new(smtp_username, smtp_password)
}
fn get_mailer() -> SmtpTransport {
    let smtp_host = &*env::var("SMTP_HOST").expect("SMTP_HOST must be set");
    let mut tls_builder = TlsConnector::builder();
    tls_builder.min_protocol_version(Some(Protocol::Tlsv10));
    let tls_parameters =
        ClientTlsParameters::new(smtp_host.to_string(), tls_builder.build().unwrap());
    let creds = get_credentials();
    SmtpClient::new((smtp_host, 587), ClientSecurity::Required(tls_parameters))
        .unwrap()
        .authentication_mechanism(Mechanism::Login)
        .credentials(creds)
        .connection_reuse(ConnectionReuseParameters::ReuseUnlimited)
        .transport()
}
pub fn send_email(email: EmailBuilder) -> Result<SmtpResult, ()> {
    // let email = Email::builder()
    //     .to("ndthong144@gmail.com")
    //     .from("ndthong144.email@gmail.com")
    //     .subject("subject")
    //     .html("<h1>Hi there</h1>")
    //     .text("message")
    //     //.attachment_from_file(Path::new("myAttachement.pdf"), None, &mime::APPLICATION_PDF).unwrap()
    //     .build()
    //     .unwrap();

    let mut mailer = get_mailer();
    // Send the email
    let result = mailer.send(email.into());

    if result.is_ok() {
        println!("Email sent");
        Ok(result)
    } else {
        println!("Could not send email: {:?}", result);
        Ok(result)
    }
}
