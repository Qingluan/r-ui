var rpc = {
    invoke : function(arg) { window.external.invoke(JSON.stringify(arg)); },
}

Ins = document.getElementsByTagName("input");
for(i=0; i< Ins.length ; i++ ){
    Ins[i].addEventListener("input", function (evt) {
        rpc.invoke({
            text:{
                content: this.value,
                id: this.id
            }
        });
    })
}
Btns = document.getElementsByClassName("btn");
for(i=0; i< Btns.length ; i++ ){
    Btns[i].addEventListener("click", function () {
        rpc.invoke({
            btn:{
                content: this.value,
                id: this.id
            }
        });
    })
}



function click(id) {
    let ele = document.getElementById(id);
    rpc.invoke({
        btn:{
            content:ele.value,
            id:ele.id
        }
    })
}
// Bts = document.getElementsByClassName("btn");
// for(i=0; i< Ins.length ; i++ ){
//     Ins[i].addEventListener(Ins[i].id, function (evt) {
//         rpc.invoke({
//             btn:{
//                 content: this.value,
//                 id: this.id
//             }
//         });
//     })
// }
