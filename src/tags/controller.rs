use super::rouille;
use super::mysql;
use super::templates;
use common::ServerSetting;
use common::error;
use super::serde_json;
use common::ResponseContentType;
use model::Thread;
use tags::view::*;
use model::Tag;
use thread_n_tag;
pub fn process(request:&rouille::Request, conn:&mut mysql::PooledConn, setting:&ServerSetting,ctype:ResponseContentType)->Option<rouille::Response>{
    router!(request, 


    (GET) (/threads/{uid:u32}/tags)=>{
        let thread = match Thread::get(conn, uid){
            Some(v)=>v,
            None=>return Some(rouille::Response::empty_404())
        };
        let tags = thread_n_tag::get_tags_in_thread( conn, &thread);
        let (content_type, data) = match ctype{
            ResponseContentType::Html|ResponseContentType::Xml=>{
                let mut buffer = Vec::new();
                templates::format_thread_tags(&mut buffer, &tags);
                ("text/html;charset=utf-8", buffer)
            },
            ResponseContentType::Json=>{
                let buffer = 
                serde_json::to_vec(&tags).unwrap();
                ("application/json", buffer)
            }
        };
        return Some(rouille::Response::from_data(content_type, data));
    },
    (GET) (/tags)=>{
        //let tags = thread_n_tag::get_tags(&mut model);
        let mut buffer = Vec::new();
        let tag_list = match request.get_param("q"){
            Some(v)=>Tag::list(conn, &v),
            None=>Vec::new()
        };
        templates::tags(&mut buffer,tag_list);
        return Some(rouille::Response::from_data("text/html;charset=utf-8", buffer)) ;
    },
    (GET) (/tags/{tag:String})=>{
        //eprint!("{}",tag);
        let tag = Tag::get(conn, &tag);
        let (content_type, data) = match ctype{
            ResponseContentType::Html|ResponseContentType::Xml=>{
                let mut buffer = Vec::new();
                if let &Some(ref v) = tag.get_threads(){
                    templates::tag_thread_list(&mut buffer, v);
                }
                ("text/html;charset=utf-8", buffer)
            },
            ResponseContentType::Json=>{
                let buffer = 
                serde_json::to_vec(&tag).unwrap();
                ("application/json", buffer)
            }
        };
        return Some(rouille::Response::from_data(content_type, data)) ;
    },
    _=>return None)
}