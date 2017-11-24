use super::rouille;
use super::serde_json;
use super::templates;
use common::{ResponseContentType};
use model::Thread;
pub fn thread_list_view(content_type:ResponseContentType, threads:Vec<Thread>)->rouille::Response{
    let s = match content_type{
        ResponseContentType::Json=>{
            let v = try_or_400!(serde_json::to_vec(&threads));
            rouille::Response::from_data("application/json", v)
        },
        _=>{
            let mut s = Vec::new();
            templates::default(&mut s,threads).unwrap();
            rouille::Response::from_data("text/html;charset=utf-8", s)
        }
    };
    return s;
}
pub fn thread_view(content_type:ResponseContentType, thread:Thread)->rouille::Response{
    let s = match content_type{
        ResponseContentType::Json=>{
            let v = try_or_400!(serde_json::to_vec(&thread));
            rouille::Response::from_data("application/json", v)
        }
        _=>{
            let mut s = Vec::with_capacity(1024 * 1024);
            templates::thread_view(&mut s,&thread).unwrap();
            rouille::Response::from_data("text/html;charset=utf-8", s)
        }
    };
    return s;
}
pub fn thread_create_response(content_type:ResponseContentType, thread:Thread)->rouille::Response{
    let s = match content_type{
        ResponseContentType::Json=>{
            let mut res = Vec::new();
            use std::io::Write;
            write!(&mut res,"{{\"redirectURL\":\"/threads/{}\"}}",thread.get_uid());
            rouille::Response::from_data("application/json", res)
        }
        _=>{
            let s =format!("<!doctype html><html><head><script>document.location='/threads/{}'</script></head></html>", thread.get_uid());
            rouille::Response::html(s)
        }
    };
    return s;
}