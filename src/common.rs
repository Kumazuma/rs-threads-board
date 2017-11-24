extern crate serde;
extern crate rouille;
extern crate crypto;

pub fn error(msg:&str, code:u16)->rouille::Response{
    rouille::Response::text(msg).with_status_code(code)
}
pub fn to_sha3(text:&str)->String{
    use crypto::digest::Digest;
    use crypto::sha3::Sha3;

    // create a SHA3-512 object
    let mut hasher = Sha3::sha3_512();

    // write input message
    hasher.input_str(text);

    // read hash digest
    hasher.result_str()
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ServerSetting{
	pub host:String,
	pub db:String,
	pub user:String,
	pub password:String,
    pub aes_iv:String,
    pub aes_key:String
}
#[derive(Clone, Copy)]
pub enum ResponseContentType{
    Html,
    Json,
    Xml
}
pub fn check_accept_type(request:&rouille::Request)->ResponseContentType{
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
        use std::cmp::Ordering;
        match v1.cmp(&v2){
            Ordering::Equal=>Ordering::Greater,
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