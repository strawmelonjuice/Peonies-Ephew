/*
 * Copyright (c) 2024, MLC 'Strawmelonjuice' Bloeiman
 *
 * Licensed under the BSD 3-Clause License. See the LICENSE file for more info.
 */

use actix_session::Session;
use actix_web::http::StatusCode;
use actix_web::web::Data;
use actix_web::{HttpRequest, HttpResponse};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use tokio::sync::{Mutex, MutexGuard};

use crate::storage::users::auth::check;
use crate::storage::{fetch, BasicUserInfo};
use crate::{Config, ServerVars};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct JSClientData {
    pub instance: JSClientInstance,
    pub user: JSClientUser,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct JSClientInstance {
    pub config: JSClientConfig,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct JSClientConfig {
    pub interinstance: JSClientInterinstance,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct JSClientInterinstance {
    pub iid: String,
    pub lastpoll: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct JSClientUser {
    pub username: String,
    pub id: i64,
}

pub(crate) async fn update(
    server_z: Data<Mutex<ServerVars>>,
    session: Session,
    req: HttpRequest,
) -> HttpResponse {
    let server_y: MutexGuard<ServerVars> = server_z.lock().await;
    let config: Config = server_y.clone().config;
    let coninfo = req.connection_info();
    let ip = coninfo
        .realip_remote_addr()
        .clone()
        .unwrap_or("<unknown IP>");
    let username_ = session.get::<String>("username").unwrap_or(None);
    let username_a = session.get::<String>("username");
    let username_b = username_a.unwrap_or(None).unwrap_or(String::from(""));
    let username_c = if username_b != String::from("") {
        format!("/{}", username_b.green())
    } else {
        String::from("")
    };
    info!(
        "{}\t{:>45.47}\t\t{}{:<26}",
        "/api/fe/update".magenta(),
        ip.yellow(),
        "Request/200".bright_green(),
        username_c
    );
    let json = serde_json::to_string(&JSClientData {
        instance: JSClientInstance {
            config: JSClientConfig {
                interinstance: JSClientInterinstance {
                    iid: "".to_string(),
                    lastpoll: 0,
                },
            },
        },
        user: JSClientUser {
            username: match username_.clone() {
                Some(username) => username.to_string(),
                None => "unset".to_string(),
            },
            id: fetch(
                &config.clone(),
                String::from("Users"),
                "username",
                match username_ {
                    Some(username) => username.to_string(),
                    None => "unset".to_string(),
                },
            )
            .unwrap_or(None)
            .unwrap_or(String::from("0"))
            .parse()
            .unwrap_or(0),
        },
    })
    .unwrap();
    HttpResponse::build(StatusCode::OK)
        .content_type("text/json; charset=utf-8")
        .body(json)
}
#[derive(Deserialize)]
pub(super) struct AuthReqData {
    username: String,
    password: String,
}

pub(crate) async fn auth(
    server_z: Data<Mutex<ServerVars>>,
    session: Session,
    req: HttpRequest,
    data: actix_web::web::Json<AuthReqData>,
) -> HttpResponse {
    let server_y: MutexGuard<ServerVars> = server_z.lock().await;
    let config = server_y.clone().config;
    (server_y.tell)("Auth request received.".to_string());
    let result = check(
        data.username.clone(),
        data.password.clone(),
        &server_y.clone(),
    );
    let coninfo = req.connection_info();
    let ip = coninfo
        .realip_remote_addr()
        .clone()
        .unwrap_or("<unknown IP>");
    if result.success && result.user_exists && result.password_correct {
        let user_id = result.user_id.unwrap();
        let user: BasicUserInfo = serde_json::from_str(
            fetch(&config, String::from("Users"), "id", user_id.to_string())
                .unwrap()
                .unwrap()
                .as_str(),
        )
        .unwrap();
        let username = user.username;
        info!("User '{}' logged in succesfully from {}", username, ip);
        session.insert("userid", user.id).unwrap();
        session.insert("username", username).unwrap();
        session
            .insert("validity", config.clone().run.session_valid)
            .unwrap();
        HttpResponse::build(StatusCode::OK)
            .content_type("text/json; charset=utf-8")
            .body(r#"{"Ok": true}"#)
    } else {
        HttpResponse::build(StatusCode::UNAUTHORIZED)
            .content_type("text/json; charset=utf-8")
            .body(r#"{"Ok": false}"#)
    }
}