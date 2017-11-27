use super::rouille;
use super::mysql;
use common::ServerSetting;
use common::error;
use common::ResponseContentType;
use login::Login;
use model::Thread;
use threads::view::*;
pub fn process(request:&rouille::Request, conn:&mut mysql::PooledConn, setting:&ServerSetting,ctype:ResponseContentType)->Option<rouille::Response>{
    router!(request, 
    (GET)(/)=>{
        let q:Option<String> = request.get_param("q");
        
        let offset:usize = match request.get_param("offset"){
            Some(v)=>v.parse().unwrap_or(0usize),
            None=>0usize
        };

        let count:usize = match request.get_param("count"){
            Some(v)=>v.parse().unwrap_or(25usize),
            None=>25usize
        };
        let r_etag:Option<&str> = match request.header("If-None-Match"){
            Some(v) if v.starts_with("\"") => Some(&v[1..v.len()-1]),
            Some(v)=>Some(v),
            _=>None
        };
        let etag = Thread::e_tag(conn, offset, &q);
        
        match r_etag{
            Some(t) if t == &etag=>{
                
                return Some(rouille::Response::from_data("",vec![])
                .with_status_code(304)
                .with_etag_keep(etag));
            }
            _=>{
                let list = Thread::list( conn, q, offset,count);
                return Some(thread_list_view(ctype, list).with_etag_keep(etag));
            }
        }
    },
    (GET)(/threads)=>{
        let q:Option<String> = request.get_param("q");
        
        let offset:usize = match request.get_param("offset"){
            Some(v)=>v.parse().unwrap_or(0usize),
            None=>0usize
        };

        let count:usize = match request.get_param("count"){
            Some(v)=>v.parse().unwrap_or(25usize),
            None=>25usize
        };
        let r_etag:Option<&str> = match request.header("If-None-Match"){
            Some(v) if v.starts_with("\"") => Some(&v[1..v.len()-1]),
            Some(v)=>Some(v),
            _=>None
        };
        let etag = Thread::e_tag(conn, offset, &q);

        match r_etag{
            Some(t) if t == &etag=>{
                
                return Some(rouille::Response::from_data("",vec![])
                .with_status_code(304)
                .with_etag_keep(etag));
            }
            _=>{
                let list = Thread::list( conn, q, offset,count);
                return Some(thread_list_view(ctype, list).with_etag_keep(etag));
            }
        }
    },
    (POST)(/threads)=>{
        let input = post_input!(request, {
            token: String,
            subject: String,
            tags:String,
            comment:String
        });
        let input = match input{
            Ok(v)=>v,
            Err( _ )=>{
                return Some(error("파라메터가 부정확합니다.",404));
            }
        };
        let user = match Login::token(&input.token,setting){
            Some(v)=>v,
            None=>{
                return Some(error("권한이 없습니다.",403));
            }
        };
        let thread =match Thread::upload( conn, &input.subject, user, &input.comment){
            Ok(v)=>{
                for it in input.tags.split(',').filter(|it|it.trim().len() != 0){
                    use model::Tag;
                    let mut tag = Tag::new(String::from(it.trim()));
                    tag.put(conn, &v);
                }
                v
            }
            Err( _ )=>{
                return Some(error("스레드를 생성할 수 없습니다.",403));
            }
        };
        return Some(thread_create_response(ctype,thread));
    },
    (GET)(/threads/{uid:u32})=>{
        return match Thread::get( conn, uid){
            Some(v)=>Some(thread_view(ctype,v)),
            None=>Some(error("해당 스레드는 존재하지 않습니다",404))
        };
    },
    (DELETE)(/threads/{uid:u32})=>{
        let input = post_input!(request, {
            token: String
        });
        let input = match input{
            Ok(v)=>v,
            Err( _ )=>{
                return Some(error("파라메터가 부정확합니다.",404));
            }
        };
        let user = match Login::token(&input.token,setting){
            Some(v)=>v,
            None=>{
                return Some(error("권한이 없습니다.",403));
            }
        };
        use model::Thread;
        let thread =  match Thread::get( conn, uid){
            None=>return Some(error("해당 스레드는 존재하지 않습니다",404)),
            Some(v)=>v
        };
        if thread.get_opener().get_uid() != user.get_uid(){
            return Some(error("권한이 없습니다.",403));
        }
        thread.delete( conn);
        return Some(rouille::Response::text("{}").with_status_code(200).with_additional_header("Content-Type","application/json")); 
    },
    _=>return None
    );
    return None;
}