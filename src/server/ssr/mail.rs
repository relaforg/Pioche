use lettre::{
    message::{header::ContentType, Mailbox},
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};

use crate::server::error::AppError;

/// Envoi d'emails par SMTP, configuré par variables d'environnement.
///
/// | Variable        | Défaut (dev, Mailpit)            |
/// |-----------------|----------------------------------|
/// | `SMTP_HOST`     | `localhost`                      |
/// | `SMTP_PORT`     | `1025`                           |
/// | `SMTP_TLS`      | `none` (`starttls`, `tls`)       |
/// | `SMTP_USERNAME` | aucun                            |
/// | `SMTP_PASSWORD` | aucun                            |
/// | `MAIL_FROM`     | `Pioche <noreply@pioche.local>`  |
#[derive(Clone)]
pub struct Mailer {
    transport: AsyncSmtpTransport<Tokio1Executor>,
    from: Mailbox,
}

impl Mailer {
    pub fn from_env() -> Result<Self, String> {
        let var = |key: &str, default: &str| {
            std::env::var(key).unwrap_or_else(|_| default.to_string())
        };

        let host = var("SMTP_HOST", "localhost");
        let port: u16 = var("SMTP_PORT", "1025")
            .parse()
            .map_err(|_| "SMTP_PORT invalide".to_string())?;

        let builder = match var("SMTP_TLS", "none").as_str() {
            "none" => AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&host),
            "starttls" => AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&host)
                .map_err(|e| e.to_string())?,
            "tls" => {
                AsyncSmtpTransport::<Tokio1Executor>::relay(&host).map_err(|e| e.to_string())?
            }
            other => {
                return Err(format!(
                    "SMTP_TLS inconnu : {other} (attendu : none, starttls ou tls)"
                ))
            }
        }
        .port(port);

        let builder = match (std::env::var("SMTP_USERNAME"), std::env::var("SMTP_PASSWORD")) {
            (Ok(user), Ok(password)) => builder.credentials(Credentials::new(user, password)),
            _ => builder,
        };

        let from = var("MAIL_FROM", "Pioche <noreply@pioche.local>")
            .parse()
            .map_err(|e| format!("MAIL_FROM invalide : {e}"))?;

        Ok(Self {
            transport: builder.build(),
            from,
        })
    }

    /// Envoie un email en texte brut.
    pub async fn send(&self, to: &str, subject: &str, body: String) -> Result<(), AppError> {
        let message = Message::builder()
            .from(self.from.clone())
            .to(to.parse::<Mailbox>()?)
            .subject(subject)
            .header(ContentType::TEXT_PLAIN)
            .body(body)?;

        self.transport.send(message).await?;
        Ok(())
    }
}
