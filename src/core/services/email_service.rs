use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::smtp::ConnectionReuseParameters;
use lettre::{ClientSecurity, ClientTlsParameters, SmtpClient, Transport};
use lettre_email::Email;
use native_tls::{Protocol, TlsConnector};
use std::env;

pub fn send_email() -> Result<(), ()> {
    let email = Email::builder()
        .to("ndthong144@gmail.com")
        .from("ndthong144.email@gmail.com")
        .subject("subject")
        .html("<h1>Hi there</h1>")
        .text("message")
        //.attachment_from_file(Path::new("myAttachement.pdf"), None, &mime::APPLICATION_PDF).unwrap()
        .build()
        .unwrap();
    let smtp_port = env::var("SMTP_PORT").expect("SMTP_PORT must be set");
    let smtp_host = &*env::var("SMTP_HOST").expect("SMTP_HOST must be set");
    let smtp_username = env::var("SMTP_USERNAME").expect("SMTP_USERNAME must be set");
    let smtp_password = env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be set");
    let creds = Credentials::new(smtp_username, smtp_password);

    let mut tls_builder = TlsConnector::builder();
    tls_builder.min_protocol_version(Some(Protocol::Tlsv10));
    let tls_parameters =
        ClientTlsParameters::new(smtp_host.to_string(), tls_builder.build().unwrap());

    // Open connection to gmail
    let mut mailer = SmtpClient::new(
        (smtp_host, 587),
        ClientSecurity::Required(tls_parameters),
    )
    .unwrap()
    .authentication_mechanism(Mechanism::Login)
    .credentials(creds)
    .connection_reuse(ConnectionReuseParameters::ReuseUnlimited)
    .transport();

    // Send the email
    let result = mailer.send(email.into());

    if result.is_ok() {
        println!("Email sent");
        Ok(())
    } else {
        println!("Could not send email: {:?}", result);
        Ok(())
    }
}
// pub fn send_confirmation_mail(confirmation: &Confirmation) -> Result<(), AuthError> {
//   let domain_url = vars::domain_url();
//   let expires = confirmation.expires_at.format("%I:%M %p %A, %-d %B, %C%y").to_string();
//   let html_text = format!(
//       "Please click on the link below to complete registration. <br/>
//        <a href=\"{domain}/register/{id}\">Complete registration</a> <br/>
//       This link expires on <strong>{expires}</strong>",
//       domain=domain_url,
//       id=confirmation.id,
//       expires=expires
//   );
//   let plain_text = format!(
//       "Please visit the link below to complete registration:\n
//       {domain}/register/{id}\n
//       This link expires on {expires}.",
//       domain=domain_url,
//       id=confirmation.id,
//       expires=expires
//   );

//   let email = Email::builder()
//                     .to(confirmation.email.clone())
//                     .from(("noreply@auth-service.com", vars::smtp_sender_name()))
//                     .subject("Complete your registration on our one-of-a-kind Auth Service")
//                     .text(plain_text)
//                     .html(html_text)
//                     .build()
//                     .unwrap();

//   let smtp_host = vars::smtp_host();
//   let mut tls_builder = TlsConnector::builder();
//   tls_builder.min_protocol_version(Some(Protocol::Tlsv10));
//   let tls_parameters = ClientTlsParameters::new(smtp_host.clone(), tls_builder.build().unwrap());

//   let mut mailer = SmtpClient::new((smtp_host.as_str(), vars::smtp_port()), ClientSecurity::Required(tls_parameters))
//                               .unwrap()
//                               .authentication_mechanism(Mechanism::Login)
//                               .credentials(Credentials::new(vars::smtp_username(), vars::smtp_password()))
//                               .connection_reuse(ConnectionReuseParameters::ReuseUnlimited)
//                               .transport();

//   let result = mailer.send(email);

//   if result.is_ok() {
//       println!("Email sent");

//       Ok(())
//   } else {
//       println!("Could not send email: {:?}", result);

//       Err(AuthError::ProcessError(String::from("Could not send confirmation email")))
//   }
// }
