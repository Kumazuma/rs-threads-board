$(document).ready(()=>{
    $("#thread-write-form").on("submit",(e)=>{
        let nickname = document.cookie.replace(/(?:(?:^|.*;\s*)nickname\s*\=\s*([^;]*).*$)|^.*$/, "$1");
        let token = document.cookie.replace(/(?:(?:^|.*;\s*)token\s*\=\s*([^;]*).*$)|^.*$/, "$1");
        let gravatar = document.cookie.replace(/(?:(?:^|.*;\s*)gravatar\s*\=\s*([^;]*).*$)|^.*$/, "$1");
        if(token == ""){
            alert("로그인하지 않았습니다. 로그인을 하시기 바랍니다.");
        }
        else{
            $("#thread-write-form input[name='token']").val(token);
            let parameter = $(e.currentTarget).serialize();
            let ajax_args = {
                url:e.currentTarget.action,
                method:"POST",
                Accept:"application/json",
                dataType:"json",
                data:parameter
            };
            $.ajax(ajax_args).done((e)=>{
                document.location = e.redirectURL;
            });
        }
        e.preventDefault();
    });
});
