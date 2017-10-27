$(document).ready(()=>{
    $.ajax(document.URL + "/comments", {
        dataType:"html",
    }).done((e)=>{
        $("#comments-view").html(e);
    });
});