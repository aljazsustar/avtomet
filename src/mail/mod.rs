use mail_send::{SmtpClientBuilder, mail_builder::MessageBuilder};

use crate::parser::oglas::Oglas;

pub async fn send_mail(oglas: Oglas) {
    let message = MessageBuilder::new()
    .from(("AvtoBot", "yo@mama.com"))
    .to(vec![
        ("Aljaž Šuštar", "aljazsustar99@gmail.com"),
    ])
    .subject(format!("Nova ponudba: {} {}!", oglas.znamka, oglas.model))
    .html_body(format!(r#"<h1>Na voljo je novo vozilo:</h1> <p> {} {}, {} prevoženih km. </p> Oglas je na voljo na <a href="{}"">povezavi</a>"#, oglas.znamka, oglas.model, oglas.kilometri, oglas.naslov));

// Connect to the SMTP submissions port, upgrade to TLS and
// authenticate using the provided credentials.
SmtpClientBuilder::new("smtp.gmail.com", 587)
    .implicit_tls(false)
    .credentials((dotenv!("GMAIL_USER"), dotenv!("GMAIL_PASS")))
    .connect()
    .await
    .unwrap()
    .send(message)
    .await
    .unwrap();
}