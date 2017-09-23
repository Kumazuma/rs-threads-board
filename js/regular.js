$(document).ready(function(){
    var param = {
        url:"/regular/list",
        dataType:"json"
    };
    $.ajax(param).done(function(data){
        var list = document.querySelector("#user-list");
        list.innerHTML ="";
        data.forEach(function(it) {
            var t = document.querySelector('#tmpl-1');
            var td = t.content.querySelectorAll("td");
            td[0].textContent = it.user.name;
            td[1].textContent = it.location;
            var radio= t.content.querySelector("input[type='radio']");
            var label = t.content.querySelector("label");
            
            radio.id = it.idx;
            radio.value = it.idx;
            label.setAttribute("for", it.idx);
            var button= t.content.querySelector("button");
            button.setAttribute("data-idx", it.idx);
            //alert("1");
            var clone2 = document.importNode(t.content, true);
            //alert("2");
            //$(list).append(clone2);
            list.appendChild(clone2);
            //alert("3");
        });
    });
    $(document).on("click","#calender td",function(e){
        $("#calender td").removeClass("click-date");
        $(e.currentTarget).addClass("click-date");
    });
    $(document).on("click","table button",function(e){
        var button = e.currentTarget;
        if(confirm("정말로 고정청소구역을 해제하겠습니까?")){
            var param = {
                url:"/regular/" + button.getAttribute("data-idx"),
                method:"DELETE"
            }
            $.ajax(param).done(function(e){
                
            });
        }
    });
    $("#submit").click(function(){
        var date = document.querySelector("#calender td.click-date").id;
        var idx =Number($("input[type='radio']:checked").val());
        var data = {
            "shift_idx":idx,
            "date":date,
            "register":""
        };
        var param = {
            url:"/regular/log",
            method:"POST",
            data:{value:JSON.stringify(data)},
            dataType:"json"
        };
        $.ajax(param).done(function(e){
            alert("완료되었습니다.");
        }).fail(function(e){
            if(e.responseJSON.msg != undefined){
                alert(e.responseJSON.msg);
            }
        });

    });

});
