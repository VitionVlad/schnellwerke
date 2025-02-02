var keycodes = new Int8Array(256);

function getid(key){
    switch(key){
        case "q":
            return 0;
        case "w":
            return 1;
        case "e":
            return 2;
        case "r":
            return 3;
        case "t":
            return 4;
        case "y":
            return 5;
        case "u":
            return 6;
        case "i":
            return 7;
        case "o":
            return 8;
        case "p":
            return 9;
        case "a":
            return 10;
        case "s":
            return 11;
        case "d":
            return 12;
        case "f":
            return 13;
        case "g":
            return 14;
        case "h":
            return 15;
        case "j":
            return 16;
        case "k":
            return 17;
        case "l":
            return 18;
        case ";":
            return 19;
        case "z":
            return 20;
        case "x":
            return 21;
        case "c":
            return 22;
        case "v":
            return 23;
        case "b":
            return 24;
        case "n":
            return 25;
        case "m":
            return 26;
        case "Q":
            return 100;
        case "W":
            return 101;
        case "E":
            return 102;
        case "R":
            return 103;
        case "T":
            return 104;
        case "Y":
            return 105;
        case "U":
            return 106;
        case "I":
            return 107;
        case "O":
            return 108;
        case "P":
            return 109;
        case "A":
            return 110;
        case "S":
            return 111;
        case "D":
            return 112;
        case "F":
            return 113;
        case "G":
            return 114;
        case "H":
            return 115;
        case "J":
            return 116;
        case "K":
            return 117;
        case "L":
            return 118;
        case "Z":
            return 120;
        case "X":
            return 121;
        case "C":
            return 122;
        case "V":
            return 123;
        case "B":
            return 124;
        case "N":
            return 125;
        case "M":
            return 126;
        case ",":
            return 27;
        case ".":
            return 28;
        case "/":
            return 29;
        case "1":
            return 30;
        case "2":
            return 31;
        case "3":
            return 32;
        case "4":
            return 33;
        case "5":
            return 34;
        case "6":
            return 35;
        case "7":
            return 36;
        case "8":
            return 37;
        case "9":
            return 38;
        case "0":
            return 39;
        case "`":
            return 40;
        case "Escape":
            return 41;
        case " ":
            return 42;
        case "Shift":
            return 43;
        case "Control":
            return 44;
        case "Tab":
            return 45;
        case "CapsLock":
            return 46;
        case "Alt":
            return 47;
        case "ArrowUp":
            return 48;
        case "ArrowLeft":
            return 49;
        case "ArrowDown":
            return 50;
        case "ArrowRight":
            return 51;
        default:
            return -1;
    }
}

document.addEventListener('keydown', function(event) {
    var id = getid(event.key);
    if(id != -1){
        keycodes[id] = 1;
    }
}, true);

document.addEventListener('keyup', function(event) {
    var id = getid(event.key);
    if(id != -1){
        keycodes[id] = 0;
    }
}, true);

export function getkey(keyid){
    return keycodes[keyid];
}