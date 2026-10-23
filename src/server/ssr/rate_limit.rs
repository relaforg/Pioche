use std::{
    net::{IpAddr, Ipv6Addr, SocketAddr},
    num::NonZeroU32,
    sync::Arc,
    time::Duration,
};

use axum::{extract::ConnectInfo, http::request::Parts};
use governor::{DefaultDirectRateLimiter, DefaultKeyedRateLimiter, Quota, RateLimiter};
use leptos::prelude::use_context;

use crate::server::error::AppError;

/// Limites de débit des endpoints sensibles, partagées par tout le serveur.
///
/// Chaque limiteur est un « seau à jetons » : une rafale autorisée (`allow_burst`),
/// puis un jeton rendu à intervalle régulier.
#[derive(Clone)]
pub struct RateLimits(Arc<Limiters>);

struct Limiters {
    /// Connexion et inscription, par IP : freine le bruteforce de mots de passe.
    auth: DefaultKeyedRateLimiter<IpAddr>,
    /// Pages publiques à jeton (`/s/…`, `/e/…`), par IP.
    public_link: DefaultKeyedRateLimiter<IpAddr>,
    /// Emails envoyés, par organisateur.
    emails_per_user: DefaultKeyedRateLimiter<i32>,
    /// Emails envoyés, tous comptes confondus : protège la réputation du domaine
    /// même si quelqu'un multiplie les comptes.
    emails_total: DefaultDirectRateLimiter,
    /// Derrière un reverse proxy, l'IP du client est dans `X-Forwarded-For`.
    trust_proxy: bool,
}

const fn nz(n: u32) -> NonZeroU32 {
    match NonZeroU32::new(n) {
        Some(n) => n,
        None => panic!("quota nul"),
    }
}

impl RateLimits {
    /// `TRUST_PROXY=true` : lire l'IP client dans `X-Forwarded-For`. À n'activer
    /// que derrière un proxy qui écrit cet en-tête, sinon n'importe qui peut le falsifier.
    pub fn from_env() -> Self {
        let trust_proxy = std::env::var("TRUST_PROXY").is_ok_and(|v| v == "true");

        Self(Arc::new(Limiters {
            auth: RateLimiter::keyed(Quota::per_minute(nz(2)).allow_burst(nz(10))),
            public_link: RateLimiter::keyed(Quota::per_minute(nz(30)).allow_burst(nz(60))),
            emails_per_user: RateLimiter::keyed(
                Quota::per_hour(nz(10))
                    .allow_burst(nz(crate::server::participants::MAX_PARTICIPANTS as u32)),
            ),
            emails_total: RateLimiter::direct(Quota::per_hour(nz(200)).allow_burst(nz(1000))),
            trust_proxy,
        }))
    }

    /// Oublie régulièrement les clés dont le seau est plein : sans ça, la table
    /// grossirait d'une entrée par IP vue depuis le démarrage.
    pub fn spawn_cleanup(&self) {
        let limits = self.0.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(600));
            loop {
                interval.tick().await;
                limits.auth.retain_recent();
                limits.public_link.retain_recent();
                limits.emails_per_user.retain_recent();
            }
        });
    }

    pub fn check_auth(&self) -> Result<(), AppError> {
        self.0
            .auth
            .check_key(&self.client_ip()?)
            .map_err(|_| AppError::TooManyRequests)
    }

    pub fn check_public_link(&self) -> Result<(), AppError> {
        self.0
            .public_link
            .check_key(&self.client_ip()?)
            .map_err(|_| AppError::TooManyRequests)
    }

    /// Réserve `count` envois d'emails pour `user_id`.
    pub fn check_emails(&self, user_id: i32, count: u32) -> Result<(), AppError> {
        let Some(count) = NonZeroU32::new(count) else {
            return Ok(());
        };
        let too_many = || {
            AppError::Invalid(
                "Trop d'emails envoyés ces dernières heures : réessaie un peu plus tard.".into(),
            )
        };
        // `check_n` renvoie deux niveaux de `Result` : l'extérieur dit si la demande
        // dépasse la rafale maximale, l'intérieur si les jetons sont disponibles maintenant.
        self.0
            .emails_per_user
            .check_key_n(&user_id, count)
            .map_err(|_| too_many())?
            .map_err(|_| too_many())?;
        self.0
            .emails_total
            .check_n(count)
            .map_err(|_| too_many())?
            .map_err(|_| too_many())
    }

    fn client_ip(&self) -> Result<IpAddr, AppError> {
        let parts = use_context::<Parts>().ok_or(AppError::Internal)?;

        // Le proxy ajoute l'IP qu'il voit en fin de liste : c'est la seule
        // valeur fiable, les précédentes viennent du client.
        let forwarded = || {
            parts
                .headers
                .get("x-forwarded-for")?
                .to_str()
                .ok()?
                .rsplit(',')
                .next()?
                .trim()
                .parse::<IpAddr>()
                .ok()
        };
        let socket = || {
            parts
                .extensions
                .get::<ConnectInfo<SocketAddr>>()
                .map(|ConnectInfo(addr)| addr.ip())
        };

        let ip = if self.0.trust_proxy {
            forwarded().or_else(socket)
        } else {
            socket()
        };
        ip.map(bucket).ok_or(AppError::Internal)
    }
}

/// Une IPv6 « résidentielle » donne tout un /64 au client : on compte par /64,
/// sinon il suffirait de changer d'adresse pour repartir avec un seau plein.
fn bucket(ip: IpAddr) -> IpAddr {
    match ip.to_canonical() {
        IpAddr::V6(v6) => {
            let [a, b, c, d, ..] = v6.segments();
            IpAddr::V6(Ipv6Addr::new(a, b, c, d, 0, 0, 0, 0))
        }
        v4 => v4,
    }
}

/// Les limites du serveur, fournies par contexte dans `main`.
pub fn rate_limits() -> Result<RateLimits, AppError> {
    use_context::<RateLimits>().ok_or(AppError::Internal)
}
