use super::rouille;
use super::mysql;
use common::ServerSetting;
use common::{error, to_sha3};
use common::ResponseContentType;
use login::Login;
use users::view::*;
use user::User;

pub fn process(request:&rouille::Request, conn:&mut mysql::PooledConn, setting:&ServerSetting,ctype:ResponseContentType)->Option<rouille::Response>{
    eprintln!("{:?}",request);
    router!(request,
    (GET)(/signup)=>{
        return Some(signup(ctype));
    },
    (POST) (/users)=>{
        let input = post_input!(request, {
            email: String,
            nickname: String,
            password:String
        });
        let input = match input{
            Ok(v)=>v,
            Err( _ )=>{
                return Some(error("파라메터가 부정확합니다.",400));
            }
        };
        let user = User::new()
        .nickname(input.nickname)
        .email(input.email)
        .password(to_sha3(input.password.as_str()));
        let response = 
        match user.sign_up(conn){
            Ok( _ )=>{
                return Some(signup_ok(ctype));
            },
            Err( _ )=>{
                return Some(error("이미 가입된 이메일이 있습니다.",403));
            }
        };
    },
    (POST)(/login)=>{
        let post = match post_input!(request, {email: String,password: String,}){
            Ok(v)=>v,
            Err( _ )=>{
                return Some(error("파라메터가 부정확합니다.",403));
            }
        };
        let password = to_sha3(&post.password);
        let email = post.email;
        let mut user = User::new().email(email);
        if let Err( _ ) = user.find_by_email(conn){
            return Some(error("계정이 존재하지 않습니다.",403));
        }
        let login = match Login::try_login(user, &password){
            Some(v)=>v,
            _=>return Some(error("비밀번호가 틀렸습니다.",403))
        };
        let token = login.get_token(setting);
        return Some(signin_ok(ctype, &login.user,token));
    },
    _=>return None)
}