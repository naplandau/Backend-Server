// use lettre::{
//     smtp::{
//         authentication::{Credentials, Mechanism},
//         ConnectionReuseParameters,
//     },
//     ClientSecurity, ClientTlsParameters, SmtpClient, Transport,
// };
// use native_tls::{Protocol, TlsConnector};

// pub async fn send_confirmation(Confir) -> Result<()> {
//     let domain_url = "localhost:3000";
//     let expires = confirmation
//         .expires_at
//         .format("%I:%M %p %A, %-d %B, %C%y")
//         .to_string();
//     let html_text = format!(
//         "Please click on the link below to complete registration. <br/>
//        <a href=\"{domain}/register?id={id}&email={email}\">Complete registration</a> <br/>
//       This link expires on <strong>{expires}</strong>",
//         domain = domain_url,
//         id = confirmation.id,
//         email = confirmation.email,
//         expires = expires
//     );
//     let plain_text = format!(
//         "Please visit the link below to complete registration:\n
//       {domain}/register.html?id={id}&email={email}\n
//       This link expires on {expires}.",
//         domain = domain_url,
//         id = confirmation.id,
//         email = confirmation.email,
//         expires = expires
//     );
//     let email = Email::builder()
//         .to(confirmation.email.clone())
//         .from(("noreply@auth-service.com", vars::smtp_sender_name()))
//         .subject("Complete your registration on our one-of-a-kind Auth Service")
//         .text(plain_text)
//         .html(html_text)
//         .build()
//         .unwrap();
//     let smtp_host = vars::smtp_host();
//     let mut tls_builder = TlsConnector::builder();
//     tls_builder.min_protocol_version(Some(Protocol::Tlsv10));
//     let tls_parameters = ClientTlsParameters::new(smtp_host.clone(), tls_builder.build().unwrap());
//     let mut mailer = SmtpClient::new(
//         (smtp_host.as_str(), vars::smtp_port()),
//         ClientSecurity::Required(tls_parameters),
//     )
//     .unwrap()
//     .authentication_mechanism(Mechanism::Login)
//     .credentials(Credentials::new(
//         vars::smtp_username(),
//         vars::smtp_password(),
//     ))
//     .connection_reuse(ConnectionReuseParameters::ReuseUnlimited)
//     .transport();
//     let result = mailer.send(email);
//     if result.is_ok() {
//         println!("Email sent");

//         Ok(())
//     } else {
//         println!("Could not send email: {:?}", result);
//         Err(AuthError::ProcessError(String::from(
//             "Could not send confirmation email",
//         )))
//     }
// }

// fn create_confirmation(email: String, pool: &web::Data<Pool>) -> Result<(), AuthError> {
//     let confirmation = insert_record(email, pool)?;
//     send_confirmation_mail(&confirmation)
// }
