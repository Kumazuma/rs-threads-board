#[macro_use]
extern crate rouille;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate chrono;

#[macro_use]
extern crate mysql;
extern crate crypto;
use std::path::{Path};
use std::fs::File;
use rouille::Server;
use std::time::Duration;
mod model;
mod markdown;
mod db_conn;
mod thread_n_tag;
mod user;
mod login;
mod common;
mod threads;
mod profile;
mod comments;
mod users;
mod tags;
use user::User;
use common::*;
mod preview;
type ControllerReturnType = Option<rouille::Response>;
type ContorllerType = Fn(&rouille::Request, &mut mysql::PooledConn, &ServerSetting,ResponseContentType)->ControllerReturnType;

pub fn process(request:&rouille::Request, _:&mut mysql::PooledConn, setting:&ServerSetting,ctype:ResponseContentType)->ControllerReturnType{
    router!(request,
    (GET) (/write)=>{
        let mut s = Vec::new();
            templates::thread_create(&mut s).unwrap();
            
        return Some(rouille::Response::from_data("text/html;charset=utf-8", s));
    },
    (GET) (/css/{css:String}) =>{
        let css_path = Path::new("./css").join(css);
        //println!("{:?}",css_path.as_path());
        if let Ok(file) = File::open(css_path){
            return Some(rouille::Response::from_file("text/css",file));
        }
        else{
            return Some(rouille::Response::empty_404());
        }
    },
    (GET) (/fonts/{font:String}) =>{
        let font_path = Path::new("./font").join(font);
        if let Ok(file) = File::open(font_path){
            return Some(rouille::Response::from_file("application/font",file));
        }
        else{
            return Some(rouille::Response::empty_404());
        }
    },
    (GET) (/js/{js:String}) =>{
        let js_path = Path::new("./js").join(js);
        if let Ok(file) = File::open(js_path){
            return Some(rouille::Response::from_file("script/javascript",file));
        }
        else{
            return Some(rouille::Response::empty_404());
        }
    },
    _=>{
        return None;
    });
    return None;
}


const controllers:&[&ContorllerType] = &[
    &comments::controller::process,
    &threads::controller::process,
    &profile::controller::process,
    &users::controller::process,
    &tags::controller::process,
    &preview::controller::process,
    &process
]; 

fn main() {
    
    let setting:ServerSetting;
    {
        let f =std::fs::File::open("./setting.json").unwrap();
        setting =serde_json::from_reader(f).unwrap();
    } 
    eprintln!("{:?}",setting);
    let mut builder = mysql::OptsBuilder::default();
    builder.ip_or_hostname(Some(setting.host.as_str()))
		.db_name(Some(setting.db.as_str()))
		.user(Some(setting.user.as_str()))
		.pass(Some(setting.password.as_str()))
		.tcp_keepalive_time_ms(Some(1000))
		.read_timeout(Some(Duration::new(2,0)))
		.write_timeout(Some(Duration::new(2,0)))
        .prefer_socket(false)
        .tcp_port(3306);
    let pool = mysql::Pool::new(mysql::Opts::from(builder)).unwrap();
    
	println!("Now listening on localhost:9999");
	// The `start_server` starts listening forever on the given address.
	let server = Server::new("0.0.0.0:9999", move |request| {
        //eprintln!("{:?}", request);
        let setting:*const _ = &setting;
        let setting:&ServerSetting = unsafe{
            std::mem::transmute(setting)
        };
        //eprintln!("{}",setting.db);
        let mut model = try_or_400!(pool.get_conn());
        let t = common::check_accept_type(request);
        for controller in controllers{
            if let Some(v) = controller(request, &mut model, setting,t){
                return v;
            }
        }
        return rouille::Response::empty_404();
    }).unwrap();
	println!("Listening on {:?}", server.server_addr());
	server.run();
}
include!(concat!(env!("OUT_DIR"), "/templates.rs"));

