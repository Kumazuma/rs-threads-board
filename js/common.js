let offset = 0;
function getToken(){
    return document.cookie.replace(/(?:(?:^|.*;\s*)token\s*\=\s*([^;]*).*$)|^.*$/, "$1") ;
}
function getNickname(){
    return decodeURIComponent(document.cookie.replace(/(?:(?:^|.*;\s*)nickname\s*\=\s*([^;]*).*$)|^.*$/, "$1"));
}
function getGravatar(){
    return  decodeURIComponent(document.cookie.replace(/(?:(?:^|.*;\s*)gravatar\s*\=\s*([^;]*).*$)|^.*$/, "$1"));
}
function checkSignin(){
    let nickname =decodeURIComponent(document.cookie.replace(/(?:(?:^|.*;\s*)nickname\s*\=\s*([^;]*).*$)|^.*$/, "$1"));
    let token =document.cookie.replace(/(?:(?:^|.*;\s*)token\s*\=\s*([^;]*).*$)|^.*$/, "$1");
    let gravatar = decodeURIComponent(document.cookie.replace(/(?:(?:^|.*;\s*)gravatar\s*\=\s*([^;]*).*$)|^.*$/, "$1"));
    if(token != ""){
        $("#sign-in-form").css("display","none");
        $("#user-info").css("display","");
        $("#user-gravatar").attr("src",gravatar.replace("s=24","s=34"));
        $("#user-nickname").text(nickname);
        
    }
}
function signout(){
    document.cookie = "nickname=;path=/";
    document.cookie = "token=;path=/";
    document.cookie = "gravatar=;path=/";
    $("#sign-in-form").css("display","");
    $("#user-info").css("display","none");
}
$(document).ready(()=>{

    checkSignin();
    $("#btn-signout").on("click",(e)=>{
        signout();
    });
    $("#sign-in-form").on("submit",(e)=>{
        let ajax_args = {
            url:e.currentTarget.action,
            method:"POST",
            Accept:"application/json",
            dataType:"json",
            data:$("#sign-in-form").serialize()
        };
        $.ajax(ajax_args).done((e)=>{
            document.cookie =`nickname=${encodeURIComponent(e.nickname)};path=/`; 
            document.cookie = "token=" + e.token + ";path=/";
            document.cookie =`gravatar=${encodeURIComponent(e.gravatar)};path=/`;
            checkSignin();
        }).fail(( jqXHR, textStatus )=>{   
            let json_res = jqXHR.responseJSON;
            switch(json_res.code){
                case "ThereIsNotAccount":
                alert("계정이 존재하지 않습니다.");
                break;
                case "IncorrectPassword":
                alert("암호가 틀렸습니다.");
                break;
                case "InvalidParameter":
                alert("파라메터가 잘못되었습니다.");
                break;
            }
        });
        e.preventDefault();
    });
    $("#comment-write-form").on("submit",(e)=>{
        let nickname = document.cookie.replace(/(?:(?:^|.*;\s*)nickname\s*\=\s*([^;]*).*$)|^.*$/, "$1");
        let token = document.cookie.replace(/(?:(?:^|.*;\s*)token\s*\=\s*([^;]*).*$)|^.*$/, "$1");
        let gravatar = document.cookie.replace(/(?:(?:^|.*;\s*)gravatar\s*\=\s*([^;]*).*$)|^.*$/, "$1");
        if(token == ""){
            alert("로그인하지 않았습니다. 로그인을 하시기 바랍니다.");
        }
        else{
            let parameter = {
                content:$("#comment-write-content").val(),
                token:token
            };
            let ajax_args = {
                url:e.currentTarget.action,
                method:"POST",
                Accept:"application/json",
                dataType:"json",
                data:parameter
            };
            $.ajax(ajax_args).done((e)=>{

                $("#comment-write-content").val("");
                let thread_uid = $("#comment-write-thread-uid").val();
                $.ajax(document.URL + "/comments", {
                    dataType:"html",
                }).done((e)=>{
                    $("#comments-view").html(e);
                });
            });
        }
        e.preventDefault();
    });
    $("#more-threads").on("click",(e)=>{
        offset += 25;
        let a = /q=([^&]*)/;
        
        let q= a.exec(location.search);
        if(q != undefined)
        {
            q = q[1];
        }
        //let q = location.search.replace(, "$1");
        //document.location.search.substr
        let data = {offset:offset};
        if(q != undefined)
        {
            data["q"] = q;
        }
        $.ajax(document.location.pathname,{
            data:data,
            dataType:"json"
        }).done((e)=>{
            let res = 
            e.map((it)=>
                `<tr>
                <td><a href="/thrads/${it.uid}">${it.subject}</a></td>
                <td>
                    <span class="user">
                        <img src="https://www.gravatar.com/avatar/${ md5(it.opener.email)}?s=24" class="user-gravta">
                        <span>${it.opener.nickname}#${it.opener.uid}</span>
                    </span>
                </td>
                <td>${it.recent_update_datetime}</td>
            </tr>`
            );
            $("#thread-list tbody").append(res.join(""));
        });
        
    });
    $("#tag-query").on("submit",(e)=>{
        let tag = $("#tag-query input[name='q']").val();
        document.location = `/tags/${tag}`;
        e.preventDefault();
    });

});
