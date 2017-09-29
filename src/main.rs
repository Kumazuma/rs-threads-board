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
use templates::*;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::Read;
use rouille::Server;
use std::time::Duration;
use std::io;
use std::io::prelude::*;
mod model;
mod db_conn;
use db_conn::*;
use model::Model;
#[derive(Serialize, Deserialize, Debug)]
struct ServerSetting{
	host:String,
	db:String,
	user:String,
	password:String,
    aes_iv:String,
    aes_key:String
}
enum ResponseContentType{
    Html,
    Json,
    Xml
}
#[derive(Serialize, Deserialize, Debug)]
struct SignCheckResponse{
    is_signin:bool,
    sign:Option<SignInfomation>
}
#[derive(Serialize, Deserialize, Debug)]
struct SignInfomation{
    email:String,
    nickname:String
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse{
    pub code:i32,
    pub msg:String
}
fn to_sha3(text:&str)->String{
    use crypto::digest::Digest;
    use crypto::sha3::Sha3;

    // create a SHA3-512 object
    let mut hasher = Sha3::sha3_512();

    // write input message
    hasher.input_str(text);

    // read hash digest
    hasher.result_str()
}
fn check_accept_type(request:&rouille::Request)->ResponseContentType{
    let accept:&str = request.header("Accept").unwrap_or("text/html");
    let accept_types = accept.split(",");
    let select_accept_type = accept_types.max_by(|one, two|{
        let mut s:Vec< _ > = one.split("q=").collect();
        let v1:i32 = if s.len() == 1{
            10
        }
        else{
            eprintln!("{}",s[1]);
            let t:f32 = s[1].parse().unwrap();
            (t * 10f32) as i32
        };
        s = two.split("q=").collect();
        let v2:i32 = if s.len() == 1{
            10
        }
        else{
            eprintln!("{}",s[1]);
            let t:f32 = s[1].parse().unwrap();
            (t * 10f32) as i32
        };
        match v1.cmp(&v2){
            std::cmp::Ordering::Equal=>std::cmp::Ordering::Greater,
            t@_=>t
        }
    }).unwrap();
    let v:Vec<&str> = select_accept_type.split("/").collect();
    eprintln!("{:?}",v);
    return match v[1].split(";").next().unwrap(){
        "html"|"xhtml"=>ResponseContentType::Html,
        "json"=>ResponseContentType::Json,
        "xml"=>ResponseContentType::Xml,
        _=>ResponseContentType::Html
    };
}
fn sign_in(setting:&ServerSetting, request:&rouille::Request)->Result<SignInfomation, ()>{
    
    return Err(());
}
fn check_sign(setting:&ServerSetting,request:&rouille::Request)->Result<SignInfomation, ()>{
    if let Some((_, val)) = rouille::input::cookies(&request).find(|&(n, _)| n == "sign-signiture") {
        println!("Value of cookie = {:?}", val);
        use crypto::aes::*;
        use crypto::blockmodes::*;
        use crypto::buffer::*;
        let mut decryptor = cbc_decryptor(KeySize::KeySize128, setting.aes_key.as_bytes(),setting.aes_iv.as_bytes(), PkcsPadding);
        let mut reader = RefReadBuffer::new(val.as_bytes());
        let mut buffer:[u8;1024 * 4] = [0;1024*4];
        let mut len = 0usize;
        {
            let mut writer = RefWriteBuffer::new(buffer.as_mut());
            match decryptor.decrypt(&mut reader,&mut writer,false){
                Ok(v)=>{
                    
                },
                Err(e)=>{
                    return Err(());
                }
            }
            len = writer.position();
        }
        
        let f:SignInfomation =match serde_json::from_slice(&buffer[0..len]){
            Ok(v)=>v,
            Err(e)=>{
                eprintln!("{}",e);
                return Err(());
            }
        };
        return Err(());
    }
    else{
        return Err(());
    }
}
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
	let server = Server::new("127.0.0.1:9999", move |request| {
        let setting:*const _ = &setting;
        let setting:&ServerSetting = unsafe{
            std::mem::transmute::<_, _>(setting)
        };
        //eprintln!("{}",setting.db);
        let mut model = try_or_400!(pool.get_conn());
		router!(request,
            (GET) (/)=>{
                /*
                if let Some(res_type) = check_accept_type(request){
                    let mut s = Vec::new();
                    templates::default(&mut s).unwrap();
                    rouille::Response::from_data("text/html;charset=utf-8", s)
                }
                else{
                    let mut s = Vec::new();
                    templates::default(&mut s).unwrap();
                    rouille::Response::from_data("text/html;charset=utf-8", s)
                }
                */
                let offset:usize = match request.get_param("offset").unwrap_or(String::from("0")).parse(){
                    Ok(v)=>v,
                    Err( _ )=>0usize
                };
                let count:usize = match request.get_param("offset").unwrap_or(String::from("25")).parse(){
                    Ok(v)=>v,
                    Err( _ )=>25usize
                };
                let list = model.get_threads_list(offset,count);
                let mut s = Vec::new();
                
                templates::default(&mut s,list).unwrap();
                rouille::Response::from_data("text/html;charset=utf-8", s)
            },
            (GET) (/threads)=>{
                let offset:usize = match request.get_param("offset").unwrap_or(String::from("0")).parse(){
                    Ok(v)=>v,
                    Err( _ )=>0usize
                };
                let count:usize = match request.get_param("offset").unwrap_or(String::from("25")).parse(){
                    Ok(v)=>v,
                    Err( _ )=>25usize
                };
                let list = model.get_threads_list(offset,count);
                return match check_accept_type(request){
                    ResponseContentType::Json=>{
                        let v = try_or_400!(serde_json::to_vec(&list));
                        rouille::Response::from_data("application/json", v)
                    },
                    ResponseContentType::Html=>{
                        let mut s = Vec::new();
                        templates::default(&mut s,list).unwrap();
                        rouille::Response::from_data("text/html;charset=utf-8", s)
                    },
                    ResponseContentType::Xml=>{
                        let mut s = Vec::new();
                        templates::xml_threads_list(&mut s,list).unwrap();
                        rouille::Response::from_data("application/xml", s)
                    }
                };
                
            },
            (GET) (/threads/)=>{
                let offset:usize = match request.get_param("offset").unwrap_or(String::from("0")).parse(){
                    Ok(v)=>v,
                    Err( _ )=>0usize
                };
                let count:usize = match request.get_param("offset").unwrap_or(String::from("25")).parse(){
                    Ok(v)=>v,
                    Err( _ )=>25usize
                };
                let list = model.get_threads_list(offset,count);
                let v = try_or_400!(serde_json::to_vec(&list));
                rouille::Response::from_data("application/json", v)
            },
            (POST) (/threads)=>{
                rouille::Response::text("스레드 생성")
            },
            (GET) (/threads/{id:String})=>{
                eprint!("{}",id);
                rouille::Response::text("스레드 뷰")
            },
            (DELETE) (/threads/{id:String})=>{
                eprint!("{}",id);
                rouille::Response::text("스레드 삭제")
            },
            (GET) (/threads/{id:String}/comments)=>{
                eprint!("{}",id);
                rouille::Response::text("스레드 코멘트 로드")
            },
            (POST) (/threads/{id:String}/comments)=>{
                eprint!("{}",id);
                rouille::Response::text("스레드 코멘트 추가")
            },
            (GET) (/threads/{id:String}/comments/{c_id:String})=>{
                eprint!("{}, {}",id, c_id);
                rouille::Response::text("코멘트 정보 뷰")
            },
            (PUT) (/threads/{id:String}/comments/{c_id:String})=>{
                eprint!("{}, {}",id, c_id);
                rouille::Response::text("코멘트 수정")
            },
            (DELETE) (/threads/{id:String}/comments/{c_id:String})=>{
                eprint!("{}, {}",id, c_id);
                rouille::Response::text("코멘트 삭제")
            },
            (POST) (/threads/{id:String}/comments/{c_id:String}/thumbsup)=>{
                eprint!("{}, {}",id, c_id);
                rouille::Response::text("코멘트 추천")
            },
            (POST) (/threads/{id:String}/comments/{c_id:String}/thumbsdown)=>{
                eprint!("{}, {}",id, c_id);
                rouille::Response::text("코멘트 추천")
            },
            (GET) (/tags)=>{
                rouille::Response::text("태그 리스트")
            },
            (GET) (/tags/{tag:String}/threads)=>{
                eprint!("{}",tag);
                rouille::Response::text("태그가 붙여진 스레드 리스트")
            },
            (GET) (/signup)=>{
                let mut s = Vec::new();
                templates::signup(&mut s).unwrap();
                return rouille::Response::from_data("text/html;charset=utf-8", s);
            },
            (POST) (/users)=>{
                let input = try_or_400!(post_input!(request, {
                    email: String,
                    nickname: String,
                    password:String
                }));
                let user = model::User::new(0, input.nickname, input.email, Some(to_sha3(input.password.as_str())));
                match model.add_new_user(user){
                    Ok( _ )=>{
                        match check_accept_type(request){
                            ResponseContentType::Html=>{
                                //let mut s = Vec::new();
                                //templates::redirection(&mut s,"/","회원가입이 완료되었습니다.").unwrap();
                                //return rouille::Response::from_data("text/html;charset=utf-8", s);
                            },
                            ResponseContentType::Json=>{
                                let res = ApiResponse{
                                    code:0i32,
                                    msg:String::from("가입이 완료되었습니다.")
                                };
                                let res = try_or_400!(serde_json::to_vec(&res));
                                return rouille::Response::from_data("application/json", res);
                            },
                            ResponseContentType::Xml=>{

                            }
                        }
                    },
                    Err( e )=>match e{
                        model::ModelError::CollapseInsertData(f)=>{
                            
                            match check_accept_type(request){
                                ResponseContentType::Html=>{
                                    
                                },
                                ResponseContentType::Json=>{
                                    let res = ApiResponse{
                                        code:-1i32,
                                        msg:String::from("이미 가입된 이메일과 중복됩니다.")
                                    };
                                    let res = try_or_400!(serde_json::to_vec(&res));
                                    return rouille::Response::from_data("application/json", res).with_status_code(400);
                                },
                                ResponseContentType::Xml=>{

                                }
                            }
                        },
                        _=>{}
                    }
                }
                rouille::Response::text("회원 가입")
            },
            (GET) (/users/{user_name:String})=>{
                eprint!("{}",user_name);
                rouille::Response::text("회원 정보")
            },
            (GET) (/signin/check)=>{
                match check_sign(setting,request){
                    Ok(v)=>{

                    },
                    Err(())=>{

                    }
                }
                rouille::Response::text("로그아웃")
                
            },
            (PUT) (/users/{user_name:String})=>{
                eprint!("{}",user_name);
                rouille::Response::text("회원정보 수정")
            },
            (GET) (/login)=>{
                rouille::Response::text("로그인 폼")
            },
			(POST) (/login)=>{
                //eprint!("{}",user_name);
                let status_code;
                let post = try_or_400!(post_input!(request, {
                    email: String,
                    password: String,
                }));
                eprintln!("{:?}",post);
                let password = to_sha3(&post.password);
                let email = post.email;
                let response = match model.get_user(model::ConditionUserFind::ByEMail(email)){
                    Some(ref u) if u.get_password() == password=>{
                        status_code= 200;
                        ApiResponse{
                            code:0,
                            msg:String::from("로그인되었습니다.")
                        }
                    },
                    _=>{
                        status_code= 400;
                        ApiResponse{
                            code:1,
                            msg:String::from("계정이 존재하지 않거나 비밀번호가 틀립니다.")
                        }
                    }
                };
                return match check_accept_type(request){
                    ResponseContentType::Json=>{
                        let v = try_or_400!(serde_json::to_vec(&response));
                        rouille::Response::from_data("application/json", v).with_status_code(status_code)
                    },
                    ResponseContentType::Xml=>{
                        let mut s = Vec::new();
                        templates::xml_api_response(&mut s,response).unwrap();
                        rouille::Response::from_data("application/xml", s).with_status_code(status_code)
                    },
                    ResponseContentType::Html=>rouille::Response::empty_404(),
                };
			},
            (POST) (/logout)=>{
                //eprint!("{}",user_name);
                rouille::Response::text("로그아웃")
			},
			(GET) (/css/{css:String}) =>{
				let css_path = Path::new("./css").join(css);
				//println!("{:?}",css_path.as_path());
				if let Ok(file) = File::open(css_path){
					rouille::Response::from_file("text/css",file)
				}
				else{
					rouille::Response::empty_404()
				}
			},
			(GET) (/font/{font:String}) =>{
				let font_path = Path::new("./font").join(font);
				//println!("{:?}",font_path.as_path());
				if let Ok(file) = File::open(font_path){
					rouille::Response::from_file("application/font",file)
				}
				else{
					rouille::Response::empty_404()
				}
			},
			(GET) (/js/{js:String}) =>{
				let js_path = Path::new("./js").join(js);
				//println!("{:?}",js_path.as_path());
				if let Ok(file) = File::open(js_path){
					rouille::Response::from_file("script/javascript",file)
				}
				else{
					rouille::Response::empty_404()
				}
			},
			// The code block is called if none of the other blocks matches the request.
			// We return an empty response with a 404 status code.
			_ => rouille::Response::empty_404()
		)
	}).unwrap();
	println!("Listening on {:?}", server.server_addr());
	server.run();
}
include!(concat!(env!("OUT_DIR"), "/templates.rs"));

