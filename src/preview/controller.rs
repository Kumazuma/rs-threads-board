use super::rouille;
use super::mysql;
use common::ServerSetting;
use common::error;
use common::ResponseContentType;
use preview::view::*;
pub fn process(request:&rouille::Request, conn:&mut mysql::PooledConn, setting:&ServerSetting,ctype:ResponseContentType)->Option<rouille::Response>{
    let o = router!(request,
        (GET)(/preview/comment)=>{
            preview_render_view(request.get_param("text"))
        },
        _=>return None
    );
    return Some(o);
}