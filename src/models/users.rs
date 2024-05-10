use async_trait::async_trait;
use chrono::offset::Local;
use loco_rs::{auth::jwt, hash, prelude::*};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub use super::_entities::sea_orm_active_enums::Role;
pub use super::_entities::users::{self, ActiveModel, Entity, Model};
use super::{courses, is_unique_constraint_violation_err};

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginParams {
    /// login identity, with try to use this field as username or email to find the user model
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterParams {
    pub email: String,
    pub password: String,
    pub username: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct BatchSignupItem {
    pub username: String,
    pub password: String,
    pub email: String,
    pub displayed_name: Option<String>,
    pub role: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BatchSignupParams {
    /// Also register these users to course, if specified
    pub course: Option<courses::Model>,
    pub users: Vec<BatchSignupItem>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct EditParams {
    pub displayed_name: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error("got invalid role in batch signup")]
    BatchSignupInvalidRole(BatchSignupItem),
}

#[must_use]
pub const fn int_to_role(i: i32) -> Option<Role> {
    match i {
        0 => Some(Role::Admin),
        1 => Some(Role::Teacher),
        2 => Some(Role::Student),
        _ => None,
    }
}

#[must_use]
pub const fn role_to_int(r: &Role) -> i32 {
    match r {
        Role::Admin => 0,
        Role::Student => 1,
        Role::Teacher => 2,
    }
}

#[derive(Debug, Validate, Deserialize)]
pub struct Validator {
    #[validate(length(min = 2, message = "Name must be at least 2 characters long."))]
    pub name: String,
    #[validate(custom = "validation::is_valid_email")]
    pub email: String,
}

impl Validatable for super::_entities::users::ActiveModel {
    fn validator(&self) -> Box<dyn Validate> {
        Box::new(Validator {
            name: self.name.as_ref().to_owned(),
            email: self.email.as_ref().to_owned(),
        })
    }
}

#[async_trait::async_trait]
impl ActiveModelBehavior for super::_entities::users::ActiveModel {
    async fn before_save<C>(self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        self.validate()?;
        if insert {
            let mut this = self;
            this.pid = ActiveValue::Set(Uuid::new_v4());
            this.api_key = ActiveValue::Set(format!("noral-oj-{}", Uuid::new_v4()));
            Ok(this)
        } else {
            Ok(self)
        }
    }
}

#[async_trait]
impl Authenticable for super::_entities::users::Model {
    async fn find_by_api_key(db: &DatabaseConnection, api_key: &str) -> ModelResult<Self> {
        let user = users::Entity::find()
            .filter(users::Column::ApiKey.eq(api_key))
            .one(db)
            .await?;
        user.ok_or_else(|| ModelError::EntityNotFound)
    }

    async fn find_by_claims_key(db: &DatabaseConnection, claims_key: &str) -> ModelResult<Self> {
        Self::find_by_pid(db, claims_key).await
    }
}

impl super::_entities::users::Model {
    /// finds a user by the provided email
    ///
    /// # Errors
    ///
    /// When could not find user by the given token or DB query error
    pub async fn find_by_email<C: ConnectionTrait>(db: &C, email: &str) -> ModelResult<Self> {
        Self::find_by_column(db, users::Column::Email, email).await
    }

    /// finds a user by the provided verification token
    ///
    /// # Errors
    ///
    /// When could not find user by the given token or DB query error
    pub async fn find_by_verification_token(
        db: &DatabaseConnection,
        token: &str,
    ) -> ModelResult<Self> {
        Self::find_by_column(db, users::Column::EmailVerificationToken, token).await
    }

    /// /// finds a user by the provided reset token
    ///
    /// # Errors
    ///
    /// When could not find user by the given token or DB query error
    pub async fn find_by_reset_token(db: &DatabaseConnection, token: &str) -> ModelResult<Self> {
        Self::find_by_column(db, users::Column::ResetToken, token).await
    }

    /// finds a user by the provided pid
    ///
    /// # Errors
    ///
    /// When could not find user  or DB query error
    pub async fn find_by_pid(db: &DatabaseConnection, pid: &str) -> ModelResult<Self> {
        let parse_uuid = Uuid::parse_str(pid).map_err(|e| ModelError::Any(e.into()))?;
        Self::find_by_column(db, users::Column::Pid, parse_uuid).await
    }

    /// finds a user by the provided api key
    ///
    /// # Errors
    ///
    /// When could not find user by the given token or DB query error
    pub async fn find_by_api_key(db: &DatabaseConnection, api_key: &str) -> ModelResult<Self> {
        Self::find_by_column(db, users::Column::ApiKey, api_key).await
    }

    /// finds a user by the provided username
    ///
    /// # Errors
    ///
    /// When could not find user by the given username or DB query error
    pub async fn find_by_username<C: ConnectionTrait>(db: &C, username: &str) -> ModelResult<Self> {
        Self::find_by_column(db, users::Column::Name, username).await
    }

    /// finds a model by given column and value
    ///
    /// # Errors
    ///
    /// When cloud not find user by the given column or DB query error
    async fn find_by_column<C: ConnectionTrait>(
        db: &C,
        column: impl sea_orm::ColumnTrait,
        value: impl Into<sea_orm::Value> + Send,
    ) -> ModelResult<Self> {
        let user = users::Entity::find()
            .filter(column.eq(value))
            .one(db)
            .await?;
        user.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// Verifies whether the provided plain password matches the hashed password
    ///
    /// # Errors
    ///
    /// when could not verify password
    #[must_use]
    pub fn verify_password(&self, password: &str) -> bool {
        hash::verify_password(password, &self.password)
    }

    /// Asynchronously creates a user with a password and saves it to the
    /// database.
    ///
    /// # Errors
    ///
    /// When could not save the user into the DB
    pub async fn create_with_password<C: ConnectionTrait + TransactionTrait>(
        db: &C,
        params: &RegisterParams,
    ) -> ModelResult<Self> {
        let txn = db.begin().await?;

        let password_hash =
            hash::hash_password(&params.password).map_err(|e| ModelError::Any(e.into()))?;
        let user = users::ActiveModel {
            email: ActiveValue::set(params.email.to_string()),
            password: ActiveValue::set(password_hash),
            name: ActiveValue::set(params.username.to_string()),
            ..Default::default()
        }
        .insert(&txn)
        .await
        .map_err(|e| {
            if is_unique_constraint_violation_err(&e) {
                ModelError::EntityAlreadyExists {}
            } else {
                ModelError::Any(e.into())
            }
        })?;

        txn.commit().await.map_err(|e| {
            if is_unique_constraint_violation_err(&e) {
                ModelError::EntityAlreadyExists {}
            } else {
                ModelError::Any(e.into())
            }
        })?;

        Ok(user)
    }

    /// Batch signup multiple users at once.
    ///
    /// # Errors
    ///
    /// - If the input role id cannot be mapped to a [`Role`] variant
    /// - DB error
    pub async fn batch_signup(
        db: &DatabaseConnection,
        params: &BatchSignupParams,
    ) -> ModelResult<Vec<Self>> {
        let tx = db.begin().await?;
        if let Some(r) = params
            .users
            .iter()
            .find(|u| u.role.map_or(false, |r| int_to_role(r).is_none()))
        {
            return Err(ModelError::Any(Box::new(Error::BatchSignupInvalidRole(
                r.clone(),
            ))));
        }

        let mut new_users = Vec::with_capacity(params.users.len());
        for u in &params.users {
            let register_result = Self::create_with_password(
                &tx,
                &RegisterParams {
                    username: u.username.clone(),
                    email: u.email.clone(),
                    password: u.password.clone(),
                },
            )
            .await;

            let new_user = match register_result {
                Err(ModelError::EntityAlreadyExists {}) => {
                    match Self::find_by_username(&tx, &u.username).await {
                        Ok(u) => Ok(u),
                        Err(_) => Self::find_by_email(&tx, &u.email).await,
                    }
                }
                // update info of new registered user
                Ok(m) => {
                    let mut am = m.into_active_model();
                    am.displayed_name = ActiveValue::set(u.displayed_name.clone());
                    Ok(am.verified(&tx).await?)
                }
                r => r,
            }?;

            // TODO: update course info

            new_users.push(new_user);
        }
        tx.commit().await?;

        Ok(new_users)
    }

    /// Creates a JWT
    ///
    /// # Errors
    ///
    /// when could not convert user claims to jwt token
    pub fn generate_jwt(&self, secret: &str, expiration: &u64) -> ModelResult<String> {
        Ok(jwt::JWT::new(secret).generate_token(expiration, self.pid.to_string())?)
    }
}

impl super::_entities::users::ActiveModel {
    /// Sets the email verification information for the user and
    /// updates it in the database.
    ///
    /// This method is used to record the timestamp when the email verification
    /// was sent and generate a unique verification token for the user.
    ///
    /// # Errors
    ///
    /// when has DB query error
    pub async fn set_email_verification_sent(
        mut self,
        db: &DatabaseConnection,
    ) -> ModelResult<Model> {
        self.email_verification_sent_at = ActiveValue::set(Some(Local::now().naive_local()));
        self.email_verification_token = ActiveValue::Set(Some(Uuid::new_v4().to_string()));
        Ok(self.update(db).await?)
    }

    /// Sets the information for a reset password request,
    /// generates a unique reset password token, and updates it in the
    /// database.
    ///
    /// This method records the timestamp when the reset password token is sent
    /// and generates a unique token for the user.
    ///
    /// # Arguments
    ///
    /// # Errors
    ///
    /// when has DB query error
    pub async fn set_forgot_password_sent(mut self, db: &DatabaseConnection) -> ModelResult<Model> {
        self.reset_sent_at = ActiveValue::set(Some(Local::now().naive_local()));
        self.reset_token = ActiveValue::Set(Some(Uuid::new_v4().to_string()));
        Ok(self.update(db).await?)
    }

    /// Records the verification time when a user verifies their
    /// email and updates it in the database.
    ///
    /// This method sets the timestamp when the user successfully verifies their
    /// email.
    ///
    /// # Errors
    ///
    /// when has DB query error
    pub async fn verified<C: ConnectionTrait>(mut self, db: &C) -> ModelResult<Model> {
        self.email_verified_at = ActiveValue::set(Some(Local::now().naive_local()));
        Ok(self.update(db).await?)
    }

    /// Resets the current user password with a new password and
    /// updates it in the database.
    ///
    /// This method hashes the provided password and sets it as the new password
    /// for the user.
    ///
    /// # Errors
    ///
    /// when has DB query error or could not hashed the given password
    pub async fn reset_password(
        mut self,
        db: &DatabaseConnection,
        password: &str,
    ) -> ModelResult<Model> {
        self.password =
            ActiveValue::set(hash::hash_password(password).map_err(|e| ModelError::Any(e.into()))?);
        Ok(self.update(db).await?)
    }

    /// Edit an user's info, generally this shoud only be done by admin.
    ///
    /// # Errors
    ///
    /// When has DB query error or could not hash the given password.
    pub async fn edit(
        mut self,
        db: &impl ConnectionTrait,
        params: EditParams,
    ) -> ModelResult<Model> {
        let password = params
            .password
            .as_ref()
            .map(|pass| hash::hash_password(pass).map_err(|e| ModelError::Any(e.into())))
            .transpose()?;

        self.password = password
            .map(|p| ActiveValue::set(p))
            .unwrap_or(ActiveValue::not_set());
        self.displayed_name = ActiveValue::set(params.displayed_name);

        Ok(self.update(db).await?)
    }
}
