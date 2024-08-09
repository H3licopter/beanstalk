// For loading WASM
WebAssembly.instantiateStreaming(fetch("./page-dist-url/pkg/bs.wasm")).then((obj)=>{
let wsx = obj.instance.exports;//js
;});

// WASM will need a JS function to update the DOM
// 


// Functions that shoulden't be passed to WASM directly, so may need glue code
function uInnerHTML(id,update){
    const es = document.getElementsByClassName(id);
    if(Array.isArray(update)){update=update.join(' ')}
    for(let i = 0;i<es.length;i++){
        es[i].innerHTML=update
    }
}
function uAppendChild(id,update){
    const es = document.getElementsByClassName(id);
    if(Array.isArray(update)){update=update.join(' ')}
    for(let i = 0;i<es.length;i++){
        es[i].appendChild(update)
    }
}
function uRemoveChild(id,update){
    const es = document.getElementsByClassName(id);
    if(Array.isArray(update)){update=update.join(' ')}
    for(let i = 0;i<es.length;i++){
        es[i].removeChild(update)
    }
}
function uReplaceChild(id,update){
    const es = document.getElementsByClassName(id);
    if(Array.isArray(update)){update=update.join(' ')}
    for(let i = 0;i<es.length;i++){
        es[i].replaceChild(update)
    }
}

