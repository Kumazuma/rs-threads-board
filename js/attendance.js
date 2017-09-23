
$(document).ready(function(){
    $.ajax("/users",{dataType:"json"}).done(function(res){
        window.users = res;
        var list = users.filter(function(it){return it.is_in_home == false;});
        list.map(function(it){
            $("#3-" + it.idx + "")[0].checked = true;
        });
    });
    $("#btn-attendance-submit").click(function(){
        var date = $("input[type='date']").val();
        if(date.length == 0){
            alert("날짜를 입력해 주시길 바랍니다.");
            return;
        }
        var check = true;
        var chklength = window.users.filter((user)=>{
            return $("input[type='radio'][name='"+ user.idx +"']:checked").length == 0
        }).length;
        if(chklength != 0){
            alert("출결을 전부 입력하지 않았습니다.");
            return;
        }
        var data =JSON.stringify(window.users.map((it)=>{
            var value = $("input[type='radio'][name='"+ it.idx +"']:checked").val();
            return {
                user:it,
                status:value
            };
        }));
        $.ajax({url: "/attendance/" + date,method:"PUT" ,data:{list: data}}).done(function(){
            alert("완료되었습니다.");
        });
    });
});