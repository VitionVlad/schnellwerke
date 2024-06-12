var px = 0;
var py = 0;
var mlc = false;
var mrc = false;
var mmc = false;

document.addEventListener("mousemove", function(event){
    px = event.movementX;
    py = event.movementY;
}, false);     

export function jgetx(){
    var t = px;
    px = 0;
    return Number(t); 
}

export function jgety(){
    var t = py;
    py = 0;
    return Number(t); 
}

document.onclick = function(e){
    switch (e.button) {
      case 0:
        mlc = true;
        break;
      case 1:
        mmc = true;
        break;
      case 2:
        mrc = true;
        break;
      default:
        console.log(`Unknown button code: ${e.button}`);
    }
}

export function getmlc(){
    var t = mlc;
    mlc = 0;
    return Boolean(t);
}

export function getmmc(){
    var t = mmc;
    mmc = 0;
    return Boolean(t);
}

export function getmrc(){
    var t = mrc;
    mrc = 0;
    return Boolean(t);
}