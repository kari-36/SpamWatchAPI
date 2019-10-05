use actix_web::{HttpRequest, HttpResponse, Result, web};
use serde::Deserialize;
use serde_json::Value;

use crate::database::{Database, Token};
use crate::errors::UserError;
use crate::guards::{Permission, PermissionGuard};
use crate::guards::Permission::User;
use crate::utils;

#[derive(Debug, Deserialize)]
pub struct CreateBan {
    id: i32,
    reason: String,
}

pub fn get_bans(req: HttpRequest) -> Result<HttpResponse, UserError> {
    let guard = PermissionGuard::new(utils::get_auth_token(&req)?)?;
    if guard.admin() {
        let mut db = Database::new()?;
        let bans = db.get_bans()?;
        let bans_json = serde_json::to_value(bans)
            .map_err(|e| {
                error!(utils::LOGGER, "{}", e);
                UserError::Internal
            })?;

        Ok(HttpResponse::Ok().json(bans_json))
    } else {
        Err(UserError::Forbidden)
    }
}


pub fn post_bans(req: HttpRequest, data: web::Json<Vec<CreateBan>>) -> Result<HttpResponse, UserError> {
    let guard = PermissionGuard::new(utils::get_auth_token(&req)?)?;
    if guard.admin() {
        let mut db = Database::new()?;
        for ban in data.iter() {
            db.add_ban(ban.id, &ban.reason)?;
        }
        Ok(HttpResponse::NoContent().body(""))
    } else {
        Err(UserError::Forbidden)
    }
}

pub fn get_ban(req: HttpRequest) -> Result<HttpResponse, UserError> {
    let guard = PermissionGuard::new(utils::get_auth_token(&req)?)?;
    let user_id: i32 = req.match_info().get("id").unwrap().parse().map_err(|e| {
        error!(utils::LOGGER, "{}", e);
        UserError::BadRequest
    })?;
    let mut db = Database::new()?;
    match db.get_ban(user_id)? {
        Some(ban) => Ok(HttpResponse::Ok().json(serde_json::to_value(ban)?)),
        None => Err(UserError::NotFound)
    }
}

pub fn delete_ban(req: HttpRequest) -> Result<HttpResponse, UserError> {
    let guard = PermissionGuard::new(utils::get_auth_token(&req)?)?;
    if guard.admin() {
        let user_id: i32 = req.match_info().get("id").unwrap().parse().map_err(|e| {
            error!(utils::LOGGER, "{}", e);
            UserError::BadRequest
        })?;

        let mut db = Database::new()?;

        match db.get_ban(user_id)? {
            Some(ban) => {
                db.delete_ban(user_id)?;
                Ok(HttpResponse::NoContent().body(""))
            }
            None => Err(UserError::NotFound)
        }
    } else {
        Err(UserError::Forbidden)
    }
}
