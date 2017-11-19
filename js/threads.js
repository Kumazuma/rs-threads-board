$(document).ready(()=>{
    $.ajax(document.URL + "/comments", {
        dataType:"html",
    }).done((e)=>{
        $("#comments-view").html(e);
    });
    $.ajax(`${document.URL}/tags`,{
        dataType:"html"
    }).done((e)=>{
        $("#tag-list-view").html(e);
    });
    
    $(document).on("click", ".btn-delete-comment",(e)=>{
        if(confirm("정말 삭제하시겠습니까?") == false){
            return;
        }
        let ajax = {
            url:`/comments/${e.currentTarget.dataset.commentId}`,
            method:"DELETE",
            data:{
                token:getToken()
            }
        };
        $.ajax(ajax).done((e)=>{
            $.ajax(document.URL + "/comments", {
                dataType:"html",
            }).done((e)=>{
                $("#comments-view").html(e);
            });
        });
    });
    $("#btn-delete-thread").on("click",(e)=>{
        if(confirm("정말 삭제하시겠습니까?") == false){
            return;
        }
        let ajax = {
            url:`/threads/${e.currentTarget.dataset.threadUid}`,
            method:"DELETE",
            data:{
                token:getToken()
            }
        };
        $.ajax(ajax).done((e)=>{
            alert("삭제되었습니다.");
            document.location = "/";
        });
    });
});