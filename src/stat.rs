use actix_web::{web, HttpResponse, Responder, HttpRequest};
use std::sync::Mutex;
use std::collections::{HashSet, HashMap};
use crate::settings::Settings;
use std::time::Duration;
use actix::{Actor, AsyncContext};
use std::rc::Rc;

const USER_IDENTIFIER_LENGTH: usize = 36;

struct StatRegisteredData {
    ips: HashMap<String, u32>,
    users: HashSet<String>,
    last_count: usize
}
struct StatServiceState {
    registered_data: Mutex<StatRegisteredData>
}

async fn show_online(data: web::Data<Rc<StatServiceState>>) -> impl Responder {
    let reg_data = data.registered_data.lock().unwrap();
    HttpResponse::Ok().body(reg_data.users.len().to_string())
}

async fn report_online(req: HttpRequest, path: web::Path<String>, data: web::Data<Rc<StatServiceState>>) -> impl Responder {
    let mut reg_data = data.registered_data.lock().unwrap();
    if path.len() == USER_IDENTIFIER_LENGTH && !reg_data.users.contains(path.as_str()) {
        let ip_counter = reg_data.ips.entry(req.peer_addr().unwrap().to_string()).or_insert(0);
        if *ip_counter < Settings::get().stat.max_users_per_ip {
            *ip_counter += 1;

            reg_data.users.insert(path.into_inner());
        }
    }
    HttpResponse::Ok().body("OK")
}

struct StatSumUpActor (Rc<StatServiceState>);

impl Actor for StatSumUpActor {
    type Context = actix::Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let interval = Duration::from_secs(Settings::get().stat.check_in_interval);
        ctx.run_interval(interval, move |act, _ctx| {
            let mut reg_data = act.0.registered_data.lock().unwrap();
            reg_data.last_count = reg_data.users.len();
            reg_data.users.clear();
            reg_data.ips.clear();
        });
    }
}

pub fn configure_stat(cfg: &mut web::ServiceConfig) {
    let state = Rc::new(StatServiceState {
        registered_data: Mutex::new(StatRegisteredData {
            ips: HashMap::new(), users: HashSet::new(), last_count: 0
        })
    });
    cfg
        .data(Rc::clone(&state))
        .route("/online", web::get().to(show_online))
        .route("/online/{userid}", web::put().to(report_online));
    StatSumUpActor(state).start();
}