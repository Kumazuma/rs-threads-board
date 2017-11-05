extern crate ructe;
extern crate core;
use self::ructe::templates::ToHtml;
use self::core::fmt::Display;
use std::io::{Write,Error};
use std::mem::transmute;

use std::collections::HashMap as Dict;
fn parse_linkmark(text:&[u8])->(&[u8], Vec<u8>){
    
    let mut contents:Option<(usize, usize)> = None;
    let mut link :(Option<usize>, Option<usize>) = (None, None);
    for (i,ch) in text.iter().enumerate(){
        if let None = contents{
            if *ch == b']'{
                contents = Some((1,i));
            }
        }
        else{
            if *ch == b'('{
                link.0 = Some(i + 1);
            }
            if let Some( _ ) = link.0{
                if *ch == b')'{
                    link.1 = Some(i);
                    break;
                }
            }
        }
    }
    let r = match (link.0, link.1, contents){
        (Some(l1), Some(l2), Some(c))=>(l1,l2,c.0, c.1),
        _=>{
            return (&text[1..],vec![b'[']);
        }
    };
    let mut link = Vec::<u8>::new();
    let mut contents = Vec::<u8>::new();
    for ch in &text[r.0..r.1]{
        if *ch == b'"'{
            link.extend(b"%22");
        }
        else{
            link.push(*ch);
        }
    }
    for ch in &text[r.2 .. r.3]{
        match *ch{
            b'<'=>{
                contents.extend(b"&lt;");
            },
            b'>'=>{
                contents.extend(b"&gt;");
            }
            _=>{
                contents.push(*ch);
            }
        }
    }
    let mut msg = Vec::<u8>::new();
    msg.extend(b"<a href=\"");
    msg.append(&mut link);
    msg.extend(b"\">");
    msg.append(&mut contents);
    msg.extend(b"</a>");
    return (&text[(r.1)+1..], msg);
}
fn parse_nomark(text:&[u8])->(&[u8], Vec<u8>){
    let mut res = Vec::<u8>::new();
    let mut t = &text[2..];

    while t.len() >= 2{
        if &t[..2] == b"}}"{
            return (&t[2..], res); //(&t[2..], format!("<a href=\"{}\">{}</a>",link, res));
        }
        let s = match &t[0..1]{
            b"<"=>b"&lt;",
            b">"=>b"&gt;",
            c@_=>c
        } ;
        res.extend(s);
        t = &t[1..];
    }
    
    return (&text[1..],vec![b'{']);
}
fn parse_strikethrough<'a>(text:&'a [u8], tag:&'a [u8])->(&'a[u8], Vec<u8>){
    let mut res = Vec::<u8>::new();
    let mut t = &text[2..];

    while t.len() > 0{
        if t.len() >= 2{
            if &t[..2] == tag{
                let mut msg = Vec::<u8>::new();
                msg.extend(b"<del>");
                msg.append(&mut res);
                msg.extend(b"</del>");
                return (&t[2..], msg); //(&t[2..], format!("<a href=\"{}\">{}</a>",link, res));
            }
            if &t[..2] == b"__"{
                let mut r = parse_bold(t, &t[..2]);
                res.append(&mut r.1);
                t = r.0;
                continue;
            }
            if &t[..2] == b"**"{
                let mut r = parse_bold(t, &t[..2]);
                res.append(&mut r.1);
                t = r.0;
                continue;
            }
        }
        if &t[..1] == b"["{
            let mut r = parse_linkmark(t);
            res.append(&mut r.1);
            t = r.0;
            continue;
        }

        res.push(t[0]);
        t = &t[1..];
    }
    
    return (&text[1..],Vec::from(&tag[..1]));
}
fn parse_bold<'a>(text:&'a [u8], tag:&'a [u8])->(&'a[u8], Vec<u8>){
    let mut res = Vec::<u8>::new();
    let mut t = &text[2..];

    while t.len() > 0{
        if t.len() >= 2{
            if &t[..2] == tag{
                let mut msg = Vec::<u8>::new();
                msg.extend(b"<strong>");
                msg.append(&mut res);
                msg.extend(b"</strong>");
                return (&t[2..], msg); //(&t[2..], format!("<a href=\"{}\">{}</a>",link, res));
            }
            if &t[..2] == b"~~"{
                let mut r = parse_strikethrough(t,&t[..2]);
                res.append(&mut r.1);
                t = r.0;
                continue;
            }
            if &t[..2] == b"--"{
                let mut r = parse_strikethrough(t, &t[..2]);
                res.append(&mut r.1);
                t = r.0;
                continue;
            }
        }
        if &t[..1] == b"["{
            let mut r = parse_linkmark(t);
            res.append(&mut r.1);
            t = r.0;
            continue;
        }

        res.push(t[0]);
        t = &t[1..];
    }
    
    return (&text[1..],Vec::from(&tag[..1]));
}
fn parse_blackqoute(text:&[u8])->(&[u8], Vec<u8>){
    let mut t = text;
    let mut html = Vec::<u8>::new();
    return (t, html);
}
fn parse(text:&str)->Vec<u8>{
    let mut res = Vec::<u8>::new();
    let mut t:&[u8] = text.as_bytes();
    res.extend(b"<p>");
    while t.len() != 0{
        if t.len() > 2{
            if (t.len() == text.as_bytes().len() && &t[..1] == b">") ||
            &t[..2] == b"\n>"{

            }
            if &t[..2] == b"\n\n"{
                res.extend(b"</p><p>");
                t = &t[2..];
                continue;
            }
            if &t[..2] == b"  "{
                res.extend(b"<br>");
                t = &t[2..];
                continue;
            }
            if &t[..2] == b"~~"{
                let mut r = parse_strikethrough(t, &t[..2]);
                res.append(&mut r.1);
                t = r.0;
                continue;
            }
            if &t[..2] == b"--"{
                let mut r = parse_strikethrough(t, &t[..2]);
                res.append(&mut r.1);
                t = r.0;
                continue;
            }
            if &t[..2] == b"{{"{
                let mut r = parse_nomark(t);
                res.append(&mut r.1);
                t = r.0;
                continue;
            }
            if &t[..2] == b"__"{
                let mut r = parse_bold(t,&t[..2]);
                res.append(&mut r.1);
                t = r.0;
                continue;
            }
            if &t[..2] == b"**"{
                let mut r = parse_bold(t,&t[..2]);
                res.append(&mut r.1);
                t = r.0;
                continue;
            }
        }
        if &t[..1] == b"["{
            let mut r = parse_linkmark(t);
            res.append(&mut r.1);
            t = r.0;
            continue;
        }
        
        let s = match &t[0..1]{
            b"<"=>b"&lt;",
            b">"=>b"&gt;",
            c@_=>c
        } ;
        res.extend(s);
        t = &t[1..];
    }
    res.extend(b"</p>");
    return res;
    //return String::from_utf8(res).unwrap();
}

pub struct Markdown{
    obj:*const str
}
impl Markdown{
    pub fn to_html(&self, out: &mut Write) -> Result<(),Error>{
        let text:&str =
        unsafe{transmute::<_, &str>(self.obj)};
        out.write_all(&parse(text));
        Ok(())
    }
}
pub fn render(obj:&str)->Markdown{
    Markdown{obj:obj}
}