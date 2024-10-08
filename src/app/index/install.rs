use std::collections::BTreeMap;

use serde_json::json;
use tiny_web::sys::{
    action::{Action, Answer},
    data::Data,
    init::DBConfig,
    request::RawData,
    response::Redirect,
};
use tiny_web_macro::fnv1a_64 as hash;

pub async fn index(this: &mut Action) -> Answer {
    this.set_lang_arr(&[hash!("welcome_text"), hash!("welcome"), hash!("next")]);

    this.set(hash!("title"), this.lang(hash!("welcome")));
    this.load(hash!("head"), hash!("index"), hash!("install"), hash!("head"), None).await;
    this.load(hash!("foot"), hash!("index"), hash!("install"), hash!("foot"), None).await;

    this.render("index")
}

pub async fn license(this: &mut Action) -> Answer {
    this.set_lang_arr(&[hash!("license_text"), hash!("license"), hash!("next"), hash!("license_agree")]);
    this.set(hash!("title"), this.lang(hash!("license")));

    this.load(hash!("head"), hash!("index"), hash!("install"), hash!("head"), None).await;
    this.load(hash!("foot"), hash!("index"), hash!("install"), hash!("foot"), None).await;
    this.render("license")
}

pub async fn not_accept(this: &mut Action) -> Answer {
    this.set_lang_arr(&[hash!("license_not_accept"), hash!("license"), hash!("home")]);
    this.set(hash!("title"), this.lang(hash!("license")));

    this.load(hash!("head"), hash!("index"), hash!("install"), hash!("head"), None).await;
    this.load(hash!("foot"), hash!("index"), hash!("install"), hash!("foot"), None).await;
    this.render("not_accept")
}

pub async fn config_log(this: &mut Action) -> Answer {
    if !this.internal {
        this.response.redirect = Some(Redirect {
            url: this.not_found().await,
            permanently: true,
        });
        return Answer::None;
    }
    this.set_lang_arr(&[hash!("config_log_title"), hash!("config_log_text")]);
    this.set(hash!("config_log_file"), format!("{}/tiny.log", this.tool.get_root()));
    this.render("config_log")
}

pub async fn config_db(this: &mut Action) -> Answer {
    if !this.internal {
        this.response.redirect = Some(Redirect {
            url: this.not_found().await,
            permanently: true,
        });
        return Answer::None;
    }
    this.set_lang_arr(&[
        hash!("config_db_title"),
        hash!("config_db_host"),
        hash!("config_db_port"),
        hash!("config_db_name"),
        hash!("config_db_user"),
        hash!("config_db_pwd"),
        hash!("db_ssl"),
        hash!("db_max_text"),
        hash!("db_max_auto"),
        hash!("db_max_manual"),
        hash!("config_db_check"),
        hash!("config_db_checking"),
        hash!("config_db_check_err_network"),
        hash!("config_db_check_timeout"),
        hash!("config_db_check_err_server"),
        hash!("config_db_check_err_answer"),
        hash!("config_db_check_err_json"),
        hash!("config_db_type"),
    ]);

    this.set(hash!("db_max_value"), 2 * this.tool.get_cpu());
    this.set(hash!("config_db_type_value"), this.tool.get_db_type().to_owned());

    this.render("config_db")
}

pub async fn config_cpu(this: &mut Action) -> Answer {
    if !this.internal {
        this.response.redirect = Some(Redirect {
            url: this.not_found().await,
            permanently: true,
        });
        return Answer::None;
    }
    this.set_lang_arr(&[hash!("config_cpu_title"), hash!("config_cpu_text"), hash!("cpu_max_auto"), hash!("cpu_max_manual")]);
    this.set(hash!("cpu_max_value"), 2 + this.tool.get_cpu());
    this.render("config_cpu")
}

pub async fn config_salt(this: &mut Action) -> Answer {
    if !this.internal {
        this.response.redirect = Some(Redirect {
            url: this.not_found().await,
            permanently: true,
        });
        return Answer::None;
    }
    this.set_lang_arr(&[hash!("config_salt_title"), hash!("config_salt_text")]);
    this.set(hash!("config_salt_value"), this.session.generate_salt());
    this.render("config_salt")
}

pub async fn config_user(this: &mut Action) -> Answer {
    if !this.internal {
        this.response.redirect = Some(Redirect {
            url: this.not_found().await,
            permanently: true,
        });
        return Answer::None;
    }
    this.render("config_user")
}

pub async fn config_session(this: &mut Action) -> Answer {
    if !this.internal {
        this.response.redirect = Some(Redirect {
            url: this.not_found().await,
            permanently: true,
        });
        return Answer::None;
    }
    this.set_lang_arr(&[hash!("config_session_title"), hash!("config_session_text")]);
    this.set(hash!("config_session_value"), this.session.session_key.as_str().to_owned());
    this.render("config_session")
}

pub async fn config_lang(this: &mut Action) -> Answer {
    if !this.internal {
        this.response.redirect = Some(Redirect {
            url: this.not_found().await,
            permanently: true,
        });
        return Answer::None;
    }
    let all = this.all_lang_list().await;
    let list = this.lang_list().await;
    let mut vec = Vec::with_capacity(list.len());
    let mut set;

    for lang in all {
        let mut item = BTreeMap::new();
        item.insert(hash!("code"), Data::String(lang.code));
        item.insert(hash!("name"), Data::String(lang.name));
        set = false;
        for l in &*list {
            if l.id == lang.id {
                set = true;
                break;
            }
        }
        item.insert(hash!("set"), Data::Bool(set));

        vec.push(Data::Map(item));
    }
    this.set_lang_arr(&[hash!("config_lang_title"), hash!("config_lang_name"), hash!("config_lang_actions")]);
    this.set(hash!("lang_list"), vec);
    this.render("config_lang")
}

pub async fn config(this: &mut Action) -> Answer {
    let accept = if let Some(Data::Bool(accept)) = this.session.get(hash!("accept")) { *accept } else { false };
    if !accept {
        match this.request.input.get.get("license") {
            Some(license) => {
                if license != "accept" {
                    this.response.redirect = Some(Redirect {
                        url: "/index/install/not_accept".to_owned(),
                        permanently: false,
                    });
                    return Answer::None;
                } else {
                    this.session.set(hash!("accept"), true);
                }
            }
            None => {
                this.response.redirect = Some(Redirect {
                    url: "/index/install/not_accept".to_owned(),
                    permanently: false,
                });
                return Answer::None;
            }
        }
    }
    this.set_lang_arr(&[
        hash!("config"),
        hash!("config_save"),
        hash!("config_saving"),
        hash!("config_err_server"),
        hash!("config_err_network"),
        hash!("config_timeout"),
        hash!("config_err_answer"),
        hash!("config_err_json"),
    ]);

    this.set(hash!("title"), this.lang(hash!("config")));

    this.load(hash!("config_lang"), hash!("index"), hash!("install"), hash!("config_lang"), None).await;
    this.load(hash!("config_log"), hash!("index"), hash!("install"), hash!("config_log"), None).await;
    this.load(hash!("config_db"), hash!("index"), hash!("install"), hash!("config_db"), None).await;
    this.load(hash!("config_session"), hash!("index"), hash!("install"), hash!("config_session"), None).await;
    this.load(hash!("config_salt"), hash!("index"), hash!("install"), hash!("config_salt"), None).await;
    this.load(hash!("config_cpu"), hash!("index"), hash!("install"), hash!("config_cpu"), None).await;
    this.load(hash!("config_user"), hash!("index"), hash!("install"), hash!("config_user"), None).await;

    this.load(hash!("head"), hash!("index"), hash!("install"), hash!("head"), None).await;
    this.load(hash!("foot"), hash!("index"), hash!("install"), hash!("foot"), None).await;
    this.render("config")
}

pub async fn success(this: &mut Action) -> Answer {
    this.set_lang_arr(&[hash!("success_text"), hash!("success"), hash!("home")]);
    this.set(hash!("title"), this.lang(hash!("success")));

    this.load(hash!("head"), hash!("index"), hash!("install"), hash!("head"), None).await;
    this.load(hash!("foot"), hash!("index"), hash!("install"), hash!("foot"), None).await;

    this.tool.install_end();
    this.render("success")
}

pub async fn config_save(this: &mut Action) -> Answer {
    if !this.request.ajax {
        this.response.redirect = Some(Redirect {
            url: this.not_found().await,
            permanently: true,
        });
        return Answer::None;
    }
    this.response.content_type = Some("application/json".to_owned());
    if let RawData::Json(json) = &this.request.input.raw {
        // Validate data
        let cpu = json.get("cpu").and_then(|v| v.as_str()).unwrap_or("auto");
        if cpu != "auto" && cpu.parse::<usize>().is_err() {
            let answ = json!({
                "status": "err",
                "text": this.lang(hash!("config_save_cpu_err"))
            });
            return Answer::String(answ.to_string());
        }
        let log = json.get("log").and_then(|v| v.as_str()).unwrap_or("");
        if log.is_empty() {
            let answ = json!({
                "status": "err",
                "text": this.lang(hash!("config_save_log_err"))
            });
            return Answer::String(answ.to_string());
        }
        let salt = json.get("salt").and_then(|v| v.as_str()).unwrap_or("");
        if salt.is_empty() {
            let answ = json!({
                "status": "err",
                "text": this.lang(hash!("config_save_salt_err"))
            });
            return Answer::String(answ.to_string());
        }
        let session = json.get("session").and_then(|v| v.as_str()).unwrap_or("");
        if session.is_empty() {
            let answ = json!({
                "status": "err",
                "text": this.lang(hash!("config_save_session_err"))
            });
            return Answer::String(answ.to_string());
        }
        let db = match json.get("db") {
            Some(db) => {
                if db.is_object() {
                    db
                } else {
                    let answ = json!({
                        "status": "err",
                        "text": this.lang(hash!("config_save_err"))
                    });
                    return Answer::String(answ.to_string());
                }
            }
            None => {
                let answ = json!({
                    "status": "err",
                    "text": this.lang(hash!("config_save_err"))
                });
                return Answer::String(answ.to_string());
            }
        };

        let port = db.get("port").and_then(|v| v.as_str()).unwrap_or("");
        let port = if port.is_empty() {
            0
        } else {
            match port.parse::<u16>() {
                Ok(port) => port,
                Err(_) => {
                    let answ = json!({
                        "status": "err",
                        "text": this.lang(hash!("config_db_err"))
                    });
                    return Answer::String(answ.to_string());
                }
            }
        };
        let max = db.get("max").and_then(|v| v.as_str()).unwrap_or("auto");
        if max != "auto" && max.parse::<usize>().is_err() {
            let answ = json!({
                "status": "err",
                "text": this.lang(hash!("config_db_err"))
            });
            return Answer::String(answ.to_string());
        }
        let host = db.get("host").and_then(|v| v.as_str()).unwrap_or("");
        let name = db.get("name").and_then(|v| v.as_str()).unwrap_or("");
        let user = db.get("user").and_then(|v| v.as_str()).unwrap_or("");
        let pwd = db.get("pwd").and_then(|v| v.as_str()).unwrap_or("");
        let ssl = db.get("ssl").and_then(|v| v.as_bool()).unwrap_or(false);

        let lang = match json.get("lang") {
            Some(lang) => match lang.as_array() {
                Some(lang) => lang,
                None => {
                    let answ = json!({
                        "status": "err",
                        "text": this.lang(hash!("config_save_err"))
                    });
                    return Answer::String(answ.to_string());
                }
            },
            None => {
                let answ = json!({
                    "status": "err",
                    "text": this.lang(hash!("config_save_err"))
                });
                return Answer::String(answ.to_string());
            }
        };
        let all = this.all_lang_list().await;
        let mut langs = Vec::with_capacity(lang.len());

        for l in lang {
            let code = match l.as_str() {
                Some(code) => {
                    if code.len() == 2 {
                        code
                    } else {
                        continue;
                    }
                }
                None => continue,
            };
            for i in &all {
                if i.code == code {
                    langs.push((i.id, code));
                }
            }
        }
        if langs.is_empty() {
            let answ = json!({
                "status": "err",
                "text": this.lang(hash!("config_save_lang_err"))
            });
            return Answer::String(answ.to_string());
        }

        // Get install
        let mut sql: Vec<String> = match this.tool.get_install_sql().await {
            Ok(sql) => sql.split_terminator(";-- \\n").map(|s| s.to_string()).collect(),
            Err(e) => {
                let answ = json!({
                    "status": "err",
                    "text": format!("{}: {}", this.lang(hash!("config_save_install_sql_err")), e),
                });
                return Answer::String(answ.to_string());
            }
        };
        for (id, _) in &langs {
            sql.push(format!("UPDATE lang SET enable=true, index=0 WHERE lang_id={id};"));
        }
        let (_, def) = unsafe { *langs.get_unchecked(0) };
        let ssl_mode = if ssl { "true" } else { "false" };
        let port_mode = if port > 0 { "" } else { "#" };

        // Prepare tiny.toml
        let data = format!(
            r#"# Default language.
# Must consist of two characters according to ISO 639-1.
lang = "{def}"

# Path to log file.
# If set empty, log file will be created automatically.
log = "{log}"

# Max of work threads in async.
# Usually a little more than CPUs.
# Set "auto" to detect automatically.
max = "{cpu}"

# IP address from which to accept connections.
# Set "any" to use any IPs or empty if "bind" is Unix domain sockets.
bind_from = "127.0.0.1"

# IP address and port to work this server.
# On Unix systems, a "bind" starting with a "/" is interpreted as a path to a directory containing Unix domain sockets.
bind = "127.0.0.1:12500"

# IP address from which to accept connections for managing the server.
# Set "any" to use any IPs or empty if "rpc_ip" is Unix domain sockets.
rpc_from = "127.0.0.1"

# IP address and port to manage this server.
# On Unix systems, a "rpc" starting with a "/" is interpreted as a path to a directory containing Unix domain sockets.
rpc = "127.0.0.1:12501"

# Session key
session = "{session}"

# salt for a crypto functions
salt = "{salt}"

# Database host.
# On Unix systems, a "db_host" starting with a "/" is interpreted as a path to a directory containing Unix domain sockets.
db_host = "{host}"

# Database port.
# Can be empty.
{port_mode}db_port={port}

# Database name.
db_name = "{name}"

# Database username.
# Can be empty.
db_user = "{user}"

# Database password.
# Can be empty.
db_pwd = "{pwd}"

# Database sslmode mode.
# true is require
sslmode = {ssl_mode}

# Number of connections to the database for all work threads in async.
# Usually set from 2 to 4 on one work thread.
# Set "auto" to detect automatically.
db_max = "{max}"

# Used net protocol
# Maybe: FastCGI, SCGI, uWSGI (modifier1=0), gRPC, HTTP or WebSocket.
protocol = "FastCGI"

# Default controller for request "/" or default class or default action
action_index="/index/index/index"

# Default controller for 404 Not Found
action_not_found="/index/index/not_found"

# Default controller for error_route
action_err="/index/index/err"

"#
        );

        // Check db
        let config = DBConfig {
            host: host.to_owned(),
            port: if port == 0 { None } else { Some(port) },
            name: name.to_owned(),
            user: if user.is_empty() { None } else { Some(user.to_owned()) },
            pwd: if pwd.is_empty() { None } else { Some(pwd.to_owned()) },
            sslmode: ssl,
            max: 0,
        };
        if let Err(e) = this.tool.check_db(config, Some(sql)).await {
            let answ = json!({
                "status": "err",
                "text": format!("{}: {}", this.lang(hash!("config_db_error")), e),
            });
            return Answer::String(answ.to_string());
        }

        // Write config
        if let Err(e) = this.tool.save_config_file(&data) {
            let answ = json!({
                "status": "err",
                "text": format!("{}: {}", this.lang(hash!("config_save_file_err")), e),
            });
            return Answer::String(answ.to_string());
        }

        let answer = json!({
            "status": "ok",
        });
        Answer::String(answer.to_string())
    } else {
        let answ = json!({
            "status": "err",
            "text": this.lang(hash!("config_save_err"))
        });
        Answer::String(answ.to_string())
    }
}

pub async fn db_check(this: &mut Action) -> Answer {
    if !this.request.ajax {
        this.response.redirect = Some(Redirect {
            url: this.not_found().await,
            permanently: true,
        });
        return Answer::None;
    }
    this.response.content_type = Some("application/json".to_owned());
    if let RawData::Json(json) = &this.request.input.raw {
        let port = json.get("db_port").and_then(|v| v.as_str()).unwrap_or("");
        let port = if port.is_empty() {
            0
        } else {
            match port.parse::<u16>() {
                Ok(port) => port,
                Err(_) => {
                    let answ = json!({
                        "status": "err",
                        "text": this.lang(hash!("config_db_err"))
                    });
                    return Answer::String(answ.to_string());
                }
            }
        };

        let max = json.get("db_max").and_then(|v| v.as_str()).unwrap_or("auto");
        if max != "auto" && max.parse::<usize>().is_err() {
            let answ = json!({
                "status": "err",
                "text": this.lang(hash!("config_db_err"))
            });
            return Answer::String(answ.to_string());
        }

        let host = json.get("db_host").and_then(|v| v.as_str()).unwrap_or("");
        let name = json.get("db_name").and_then(|v| v.as_str()).unwrap_or("");
        let user = json.get("db_user").and_then(|v| v.as_str()).unwrap_or("");
        let pwd = json.get("db_pwd").and_then(|v| v.as_str()).unwrap_or("");
        let ssl = json.get("db_ssl").and_then(|v| v.as_bool()).unwrap_or(false);

        let config = DBConfig {
            host: host.to_owned(),
            port: if port == 0 { None } else { Some(port) },
            name: name.to_owned(),
            user: if user.is_empty() { None } else { Some(user.to_owned()) },
            pwd: if pwd.is_empty() { None } else { Some(pwd.to_owned()) },
            sslmode: ssl,
            max: 0,
        };

        let answer = match this.tool.check_db(config, None).await {
            Ok(ok) => json!({
                "status": "ok",
                "text": format!("{}: {}", this.lang(hash!("config_db_ok")), ok),
            }),
            Err(e) => json!({
                "status": "err",
                "text": format!("{}: {}", this.lang(hash!("config_db_error")), e),
            }),
        };
        Answer::String(answer.to_string())
    } else {
        let answ = json!({
            "status": "err",
            "text": this.lang(hash!("config_db_err"))
        });
        Answer::String(answ.to_string())
    }
}

pub async fn head(this: &mut Action) -> Answer {
    if !this.internal {
        this.response.redirect = Some(Redirect {
            url: this.not_found().await,
            permanently: true,
        });
        return Answer::None;
    }

    let list = this.lang_list().await;
    let mut code = "en".to_owned();

    for lang in &*list {
        if lang.id == this.session.get_lang_id() {
            code = lang.code.clone();
        }
    }

    this.set(hash!("lang_code"), code);

    this.set_lang_arr(&[hash!("settings"), hash!("toggle_dark")]);
    this.load(hash!("lang"), hash!("index"), hash!("install"), hash!("lang"), None).await;
    this.load(hash!("menu"), hash!("index"), hash!("install"), hash!("menu"), None).await;
    this.render("head")
}

pub async fn foot(this: &mut Action) -> Answer {
    if !this.internal {
        this.response.redirect = Some(Redirect {
            url: this.not_found().await,
            permanently: true,
        });
        return Answer::None;
    }

    this.set_lang_arr(&[hash!("terms"), hash!("policy"), hash!("cookie"), hash!("contact"), hash!("brand"), hash!("copyrignt")]);
    this.render("foot")
}

pub async fn lang(this: &mut Action) -> Answer {
    if !this.internal {
        this.response.redirect = Some(Redirect {
            url: this.not_found().await,
            permanently: true,
        });
        return Answer::None;
    }

    let list = this.lang_list().await;
    let mut vec: Vec<Data> = Vec::with_capacity(list.len());
    for lang in &*list {
        let mut l: BTreeMap<i64, Data> = BTreeMap::new();
        l.insert(hash!("id"), lang.id.into());
        l.insert(hash!("name"), lang.name.clone().into());
        vec.push(l.into())
    }
    this.set(hash!("langs"), vec);
    this.render("lang")
}

pub async fn menu(this: &mut Action) -> Answer {
    if !this.internal {
        this.response.redirect = Some(Redirect {
            url: this.not_found().await,
            permanently: true,
        });
        return Answer::None;
    }

    this.set_lang(hash!("home"));
    this.render("menu")
}

pub async fn not_found(this: &mut Action) -> Answer {
    this.set_lang_arr(&[hash!("not_found"), hash!("not_found_text"), hash!("go_back_home")]);
    this.set(hash!("title"), this.lang(hash!("not_found")));

    this.load(hash!("head"), hash!("index"), hash!("install"), hash!("head"), None).await;
    this.load(hash!("foot"), hash!("index"), hash!("install"), hash!("foot"), None).await;

    this.render("not_found")
}

pub async fn lang_set(this: &mut Action) -> Answer {
    let id = match this.request.input.get.get("id") {
        Some(id) => {
            if let Ok(id) = id.parse::<i64>() {
                id
            } else {
                this.response.redirect = Some(Redirect {
                    url: this.not_found().await,
                    permanently: false,
                });
                return Answer::None;
            }
        }
        None => {
            this.response.redirect = Some(Redirect {
                url: this.not_found().await,
                permanently: false,
            });
            return Answer::None;
        }
    };
    let list = this.lang_list().await;
    for lang in &*list {
        if lang.id == id {
            this.session.set_lang_id(id);
            let url = match this.request.input.get.get("location") {
                Some(url) => {
                    if let Ok(url) = this.percent_decode(url) {
                        url.into_owned()
                    } else {
                        "/".to_owned()
                    }
                }
                None => "/".to_owned(),
            };
            this.response.redirect = Some(Redirect { url, permanently: false });
            return Answer::None;
        }
    }
    this.response.redirect = Some(Redirect {
        url: this.not_found().await,
        permanently: false,
    });
    Answer::None
}
