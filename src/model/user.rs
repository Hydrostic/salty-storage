use anyhow;
use bcrypt;
use serde::{Deserialize, Serialize};
use serde_repr::*;
use std::error::Error;

pub enum IdentityType {
    Email,
    Phone,
    Name,
}
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum UserStatus {
    Ok = 0,
    Banned = 1,
    ReadOnly = 2,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub status: UserStatus,
    pub avatar: Option<String>,
    pub family: Option<u64>,
    // 不公开避免意外暴露
    #[serde(skip_serializing_if = "Option::is_none")]
    passwd: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserInsert {}
use rbatis::Rbatis;

#[html_sql("src/sql/user.html")]
async fn create_one(rb: &Rbatis, args: &User) -> rbatis::Result<Vec<User>> {
    impled!()
}

#[html_sql("src/sql/user.html")]
async fn get_one(
    rb: &Rbatis,
    fields: Vec<&str>,
    condition: &str,
    condition_value: &str,
) -> rbatis::Result<Vec<User>> {
    impled!()
}
pub fn new() -> User {
    return User {
        id: 0,
        name: "".to_string(),
        email: None,
        phone: None,
        status: UserStatus::Ok,
        avatar: None,
        family: None,
        passwd: None,
    };
}
const BCRYPT_COST: u32 = 6;
impl User {
    pub fn get_password(&self) -> Option<String> {
        self.passwd.clone()
    }

    pub fn set_password(&mut self, passwd: &str) -> Result<(), Box<dyn Error>> {
        self.passwd = Some(bcrypt::hash(passwd, BCRYPT_COST)?);
        Ok(())
    }
    pub async fn create(&mut self, db: &Rbatis) -> Result<(), anyhow::Error> {
        let data = create_one(db, &self).await?;
        let data = data.get(0);
        if let Some(value) = data {
            self.id = value.id
        }
        Ok(())
    }

    pub async fn compare_password(
        &mut self,
        db: &Rbatis,
        identity: IdentityType,
        password: &str,
    ) -> anyhow::Result<bool> {
        let (condition, value) = match identity {
            IdentityType::Email => ("email", self.email.as_ref().unwrap().as_str()),
            IdentityType::Phone => ("phone", self.phone.as_ref().unwrap().as_str()),
            IdentityType::Name => ("name", self.name.as_str()),
        };
        let data = get_one(db, vec![], condition, value).await?;
        let data = data.get(0);
        if let None = data {
            return Ok(false);
        }
        let remote = data.unwrap().to_owned().passwd;
        if let None = remote {
            return Ok(false);
        }
        if bcrypt::verify(password, remote.unwrap().as_str())? {
            self.clone_from(data.unwrap());
            self.passwd = None;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
