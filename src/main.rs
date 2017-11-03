#[macro_use]
extern crate rouille;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate chrono;
extern crate base64;
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
use model::Model;
pub trait Response{
    fn get_response(&self, request:&rouille::Request)->rouille::Response;
}
#[derive(Serialize, Deserialize, Debug)]
enum LoginFailedReason{
    ThereIsNoAccount,
    IncorrectPassword,
    InvalidParameter
}
#[derive(Serialize, Deserialize, Debug)]
struct LoginFailed{
    code:LoginFailedReason
}
#[derive(Serialize, Deserialize, Debug)]
struct LoginSuccess{
    token:String,
    gravatar:String,
    nickname:String
}
impl Response for LoginFailed {
    // add code here
    fn get_response(&self, request:&rouille::Request)->rouille::Response{
        match check_accept_type(request){
            ResponseContentType::Html|ResponseContentType::Xml=>rouille::Response::html(""),
            ResponseContentType::Json=>{
                let v = try_or_400!(serde_json::to_vec(self));
                rouille::Response::from_data("application/json", v).with_status_code(400)
            }
        }
    }
}
impl Response for LoginSuccess {
    // add code here
    fn get_response(&self, request:&rouille::Request)->rouille::Response{
        match check_accept_type(request){
            ResponseContentType::Html|ResponseContentType::Xml=>rouille::Response::html(""),
            ResponseContentType::Json=>{
                let v = try_or_400!(serde_json::to_vec(self));
                rouille::Response::from_data("application/json", v)
            }
        }
    }
}
struct ThreadView{
    body:model::Thread
}
struct ThreadVuewError{

}
impl Response for ThreadView {
    // add code here
    fn get_response(&self, request:&rouille::Request)->rouille::Response{
        match check_accept_type(request){
            ResponseContentType::Html=>{
                let mut s = Vec::with_capacity(1024 * 1024);
                templates::thread_view(&mut s,&self.body).unwrap();
                rouille::Response::from_data("text/html;charset=utf-8", s)
            },
            ResponseContentType::Xml=>rouille::Response::html(""),
            ResponseContentType::Json=>{
                let v = try_or_400!(serde_json::to_vec(&self.body));
                rouille::Response::from_data("application/json", v)
            }
        }
    }
}
impl Response for ThreadVuewError{
    fn get_response(&self, _:&rouille::Request)->rouille::Response{
        return rouille::Response::html("");
    }
}
struct CommentView{
    body:Vec<model::Comment>
}
impl Response for CommentView {
    // add code here
    fn get_response(&self, request:&rouille::Request)->rouille::Response{
        match check_accept_type(request){
            ResponseContentType::Html=>{
                let mut s = Vec::with_capacity(1024 * 1024);
                templates::comments_view(&mut s,&self.body).unwrap();
                rouille::Response::from_data("text/html;charset=utf-8", s)
            },
            ResponseContentType::Xml=>rouille::Response::html(""),
            ResponseContentType::Json=>{
                let v = try_or_400!(serde_json::to_vec(&self.body));
                rouille::Response::from_data("application/json", v)
            }
        }
    }
}

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
pub struct SignInfomation{
    pub email:String,
    pub nickname:String,
    pub user_agent:String
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
            //eprintln!("{}",s[1]);
            let t:f32 = s[1].parse().unwrap();
            (t * 10f32) as i32
        };
        s = two.split("q=").collect();
        let v2:i32 = if s.len() == 1{
            10
        }
        else{
            //eprintln!("{}",s[1]);
            let t:f32 = s[1].parse().unwrap();
            (t * 10f32) as i32
        };
        match v1.cmp(&v2){
            std::cmp::Ordering::Equal=>std::cmp::Ordering::Greater,
            t@_=>t
        }
    }).unwrap();
    let v:Vec<&str> = select_accept_type.split("/").collect();
    //eprintln!("{:?}",v);
    return match v[1].split(";").next().unwrap(){
        "html"|"xhtml"=>ResponseContentType::Html,
        "json"=>ResponseContentType::Json,
        "xml"=>ResponseContentType::Xml,
        _=>ResponseContentType::Html
    };
}
fn sign_in(setting:&ServerSetting, request:&rouille::Request, model:&mut model::Model)->Box<Response>{
    let post = match post_input!(request, {email: String,password: String,}){
        Ok(v)=>v,
        Err( _ )=>{
            return Box::new(LoginFailed{
                code:LoginFailedReason::InvalidParameter
            });
        }
    };
    let password = to_sha3(&post.password);
    let email = post.email;
    let user = match  model.get_user(model::ConditionUserFind::ByEMail(email)){
        Some(v)=>v,
        None=>{
            return Box::new(LoginFailed{
                code:LoginFailedReason::ThereIsNoAccount
            });
        }
    };
    if user.get_password() != password{
        return Box::new(LoginFailed{
            code:LoginFailedReason::IncorrectPassword
        });
    }

    let s = SignInfomation{
        email:user.get_email().to_string(),
        nickname:user.get_nickname().to_string(),
        user_agent:request.header("User-Agent").unwrap_or("").to_string()
    };
    
    use crypto::aes::*;
    use crypto::blockmodes::*;
    use crypto::buffer::*;
    let mut encryptor = cbc_encryptor(KeySize::KeySize256, setting.aes_key.as_bytes(),setting.aes_iv.as_bytes(), PkcsPadding);
    let s = serde_json::to_vec(&s).unwrap();
    
    let mut reader = RefReadBuffer::new(&s);
    let mut buffer:[u8;1024 * 8] = [0;1024*8];
    let len;
    {
        let mut writer = RefWriteBuffer::new(buffer.as_mut());
        encryptor.encrypt(&mut reader,&mut writer,true).unwrap();
        len = writer.position();
        //println!("pos:{}, remain:{}",writer.position(), writer.remaining());
    }
    //eprintln!("{:?}",len);
    let r = base64::encode(&buffer[0..len]);
    //eprintln!("{:?}",r);
    return Box::new(LoginSuccess{
        nickname:String::from(user.get_nickname()),
        token:r,
        gravatar:user.get_gravatar_url(Some(34))
    });
}

fn check_sign(setting:&ServerSetting,token:&String)->Result<SignInfomation, ()>{
 
    use crypto::aes::*;
    use crypto::blockmodes::*;
    use crypto::buffer::*;
    let mut decryptor = cbc_decryptor(KeySize::KeySize256, setting.aes_key.as_bytes(),setting.aes_iv.as_bytes(), PkcsPadding);
    let val = match base64::decode(token){
        Ok(v)=>v,
        Err(e)=>{
            eprintln!("{}",e);
            return Err(());
        }
    };
    let mut reader = RefReadBuffer::new(&val);
    let mut buffer:[u8;1024 * 4] = [0;1024*4];
    let mut len = 0usize;
    {
        let mut writer = RefWriteBuffer::new(buffer.as_mut());
        if let Err(e) = decryptor.decrypt(&mut reader,&mut writer,true){
            return Err(());
        }
        len = writer.position();
    }
    if let Ok(v) = serde_json::from_slice(&buffer[0..len]){
        return Ok(v);
    }
    return Err(());
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
	let server = Server::new("0.0.0.0:9999", move |request| {
        //eprintln!("{:?}", request);
        let setting:*const _ = &setting;
        let setting:&ServerSetting = unsafe{
            std::mem::transmute::<_, _>(setting)
        };
        //eprintln!("{}",setting.db);
        let mut model = try_or_400!(pool.get_conn());
router!(request,
    (GET) (/)=>{
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
    (POST) (/threads)=>{
        let input = try_or_400!(post_input!(request, {
            token: String,
            subject: String,
            tags:String,
            comment:String
        }));
        if let Ok(v) = check_sign(setting, &input.token){
            eprintln!("{:?}",input);
            let user:model::User =match model.get_user(model::ConditionUserFind::ByEMail(v.email)){
                Some(v)=>v,
                None=>{
                    eprintln!("model.get_user");
                    return rouille::Response::empty_404()
                }
            };
            let thread = match model.add_thread(&input.subject,user,&input.comment){
                Ok(v)=>v,
                Err(_)=>{
                    eprintln!("match model.add_thread");
                    return rouille::Response::empty_404()
                }
            };
            let mut res = Vec::new();
            use std::io::Write;
            write!(&mut res,"{{\"redirectURL\":\"/threads/{}\"}}",thread.get_uid());
            return rouille::Response::from_data("application/json", res);
        }
        else{
            return rouille::Response::empty_404();
        }
    },
    (GET) (/write)=>{
        let mut s = Vec::new();
        templates::thread_create(&mut s).unwrap();
        rouille::Response::from_data("text/html;charset=utf-8", s)
    },
    (GET) (/threads/{id:i32})=>{
        let response:Box<Response>;
        if let Some(t) = model.get_thread(id){
            response = Box::new(ThreadView{body:t});
        }
        else{
            response = Box::new(ThreadVuewError{});
        }
        response.get_response(request)
    },
    (GET) (/threads/{id:i32}/)=>{let id = id;rouille::Response::empty_404()},
    (DELETE) (/threads/{id:String})=>{
        eprint!("{}",id);
        rouille::Response::text("스레드 삭제")
    },
    (GET) (/threads/{id:i32}/comments)=>{
        let response:Box<Response>;
        if let Some(t) = model.get_comments(id){
            response = Box::new(CommentView{body:t});
        }
        else{
            response = Box::new(ThreadVuewError{});
        }
        response.get_response(request)
    },
    (POST) (/threads/{id:i32}/comments)=>{
        let param =try_or_400!(post_input!(request,{
            content:String,
            token:String  
        }));
        let sign = match check_sign(setting, &param.token){
            Ok(v)=>v,
            Err( _ )=>return rouille::Response::text("application/json").with_status_code(400)
        };
        let user =match model.get_user(model::ConditionUserFind::ByEMail(sign.email)){
            Some(v)=>v,
            None=>return rouille::Response::text("application/json").with_status_code(400)
        };
        model.add_new_comment(id, user, param.content);
        let v:Vec<u8> =b"{}".to_vec();
        rouille::Response::from_data("application/json", v)
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
        let response = 
        match model.add_new_user(user){
            Ok( _ )=>ApiResponse{
                code:0i32,
                msg:String::from("가입이 완료되었습니다.")
            },
            Err( e )=>match e{
                model::ModelError::CollapseInsertData( _ )=>ApiResponse{
                    code:-1i32,
                    msg:String::from("이미 가입된 이메일과 중복됩니다.")
                },
                _=>ApiResponse{
                    code:-1i32,
                    msg:String::from("이미 가입된 이메일과 중복됩니다.")
                }
            }
        };
        let code = if response.code == 0{200}else{400};
        return match check_accept_type(request){
            ResponseContentType::Json=>{
                let v = try_or_400!(serde_json::to_vec(&response));
                rouille::Response::from_data("application/json", v).with_status_code(code)
            },
            ResponseContentType::Xml=>{
                let mut s = Vec::new();
                templates::xml_api_response(&mut s,response).unwrap();
                rouille::Response::from_data("application/xml", s).with_status_code(code)
            },
            ResponseContentType::Html=>rouille::Response::empty_404(),
        };
    },
    (GET) (/users/{user_name:String})=>{
        eprint!("{}",user_name);
        rouille::Response::text("회원 정보")
    },
    (PUT) (/users/{user_name:String})=>{
        eprint!("{}",user_name);
        rouille::Response::text("회원정보 수정")
    },
    (GET) (/login)=>{
        rouille::Response::text("로그인 폼")
    },
    (POST) (/login)=>{
        sign_in(setting, request,&mut model).get_response(&request)
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

