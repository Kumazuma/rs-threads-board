function checkSignin(){
    let nickname = document.cookie.replace(/(?:(?:^|.*;\s*)nickname\s*\=\s*([^;]*).*$)|^.*$/, "$1");
    let token = document.cookie.replace(/(?:(?:^|.*;\s*)token\s*\=\s*([^;]*).*$)|^.*$/, "$1");
    let gravatar = document.cookie.replace(/(?:(?:^|.*;\s*)gravatar\s*\=\s*([^;]*).*$)|^.*$/, "$1");
    if(token != ""){
        $("#sign-in-form").css("display","none");
        $("#user-info").css("display","");
        $("#user-gravatar").attr("src",gravatar.replace("s=24","s=34"));
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
            document.cookie = "nickname=" + e.nickname + ";path=/";
            document.cookie = "token=" + e.token + ";path=/";
            document.cookie = "gravatar=" + e.gravatar + ";path=/";
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
});
