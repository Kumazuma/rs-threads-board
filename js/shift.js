$(document).ready(function(){
    $(document).on("click","#calender td",function(){
        $("#calender td").removeClass("click-date");
        $(this).addClass("click-date");
        $.ajax("/shift/" + this.id,{dataType:"json"}).done(function(data){
            var list = $("#shift-list tbody").html("");
            for(it of data){
                var t = document.querySelector('#shift-list-row'),
                td = t.content.querySelectorAll("td");
                td[0].textContent = it.user.name;
                td[1].textContent = it.user.date_come_in;
                
                radios= t.content.querySelectorAll("input[type='radio']");
                
                for(radio of radios){
                    radio.name =  it.user.idx;
                    radio.id = radio.value +"/" +it.user.name + "/" +it.user.date_come_in;
                    radio.checked = radio.value == it.status;
                }
                labels = t.content.querySelectorAll("label");
                for(var i = 0 ; i < labels.length ; i++)
                {
                    labels[i].setAttribute("for",radios[i].id);
                }
                var clone2 = document.importNode(t.content, true);
                list.append(clone2);
            }
        });
    });
    $(document).on("change","#shift-list input[type='radio']",function(){
        var data = {
            status:this.value,
            user_idx:this.name
        };
        $.ajax("/shift/" +  $("#calender td.click-date")[0].id,{data:data,dataType:"json",method:"POST"});
    });
});