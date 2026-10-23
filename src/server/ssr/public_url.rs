/// URL publique du site (variable `PUBLIC_URL`), pour les liens partagés ou envoyés par email.
///
/// En dev, `http://localhost:3000` par défaut. En release elle est obligatoire :
/// un lien `localhost` dans un email ne mènerait nulle part.
#[derive(Clone)]
pub struct PublicUrl(String);

impl PublicUrl {
    pub fn from_env() -> Result<Self, String> {
        let url = match std::env::var("PUBLIC_URL") {
            Ok(url) => url,
            Err(_) if cfg!(debug_assertions) => "http://localhost:3000".to_string(),
            Err(_) => return Err("PUBLIC_URL manquant (ex : https://pioche.fr)".to_string()),
        };
        if !url.starts_with("http://") && !url.starts_with("https://") {
            return Err(format!("PUBLIC_URL doit commencer par http(s):// : {url}"));
        }
        Ok(Self(url.trim_end_matches('/').to_string()))
    }

    /// Lien absolu vers `path`, qui commence par `/`.
    pub fn link(&self, path: &str) -> String {
        format!("{}{path}", self.0)
    }
}
