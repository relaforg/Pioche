use crate::server::error::AppError;

pub struct Exclusion((String, String));

impl Exclusion {
    pub fn new(couple: (String, String)) -> Result<Self, AppError> {
        if couple.0 == couple.1 {
            return Err(AppError::Invalid(
                "Une exclusion doit concerner deux participants différents".into(),
            ));
        }
        Ok(Exclusion(couple))
    }
}
