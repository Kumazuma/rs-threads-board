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
});