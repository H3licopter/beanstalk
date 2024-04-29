// Functions for Updating DOM
function uInnerHTML(id,update){
    const es = document.getElementsByClassName('c'+id);
    for (let i = 0;i<es.length;i++) {
      es[i].innerHTML=update
    }
}