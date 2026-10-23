use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::server::error::AppError;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SecretSantaOptions {
    pub budget_cents: Option<i32>,
    pub exchange_date: Option<NaiveDate>,
}

impl SecretSantaOptions {
    pub fn new(budget: Option<f64>, exchange_date: Option<NaiveDate>) -> Result<Self, AppError> {
        Ok(Self {
            budget_cents: budget.map(parse_budget).transpose()?,
            exchange_date: exchange_date.map(parse_date).transpose()?,
        })
    }
}

/// « 25 € », « 12,50 € ». Partagé entre la vue et les emails.
pub fn format_euros(cents: i32) -> String {
    match cents % 100 {
        0 => format!("{} €", cents / 100),
        rest => format!("{},{rest:02} €", cents / 100),
    }
}

/// « 20 décembre ».
pub fn format_day(date: NaiveDate) -> String {
    date.format_localized("%-d %B", chrono::Locale::fr_FR)
        .to_string()
}

fn parse_budget(budget: f64) -> Result<i32, AppError> {
    if budget <= 0.0 {
        return Err(AppError::Invalid(
            "Le budget doit être strictement supérieur a 0".into(),
        ));
    }

    if budget > 1000.0 {
        return Err(AppError::Invalid(
            "Le budget ne doit pas dépasser 1000".into(),
        ));
    }

    if budget.is_infinite() || budget.is_nan() {
        return Err(AppError::Invalid("Le budget invalide".into()));
    }

    Ok((budget * 100.0).round() as i32)
}

fn parse_date(date: NaiveDate) -> Result<NaiveDate, AppError> {
    let today = chrono::Local::now().date_naive();

    if date < today {
        return Err(AppError::Invalid("La date d'échange est passée".into()));
    }

    Ok(date)
}

#[cfg(feature = "ssr")]
mod convert {
    use crate::{entities::santa_settings, server::secret_santa_option::SecretSantaOptions};

    impl From<santa_settings::Model> for SecretSantaOptions {
        fn from(value: santa_settings::Model) -> Self {
            Self {
                budget_cents: value.budget_cents,
                exchange_date: value.exchange_date,
            }
        }
    }
}
