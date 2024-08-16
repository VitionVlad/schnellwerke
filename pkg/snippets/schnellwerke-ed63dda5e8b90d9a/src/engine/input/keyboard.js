var keycode = -1;

document.addEventListener('keydown', function(event) {
    switch(event.key){
        case "q":
            keycode = 0;
            break;
        case "w":
            keycode = 1;
            break;
        case "e":
            keycode = 2;
            break;
        case "r":
            keycode = 3;
            break;
        case "t":
            keycode = 4;
            break;
        case "y":
            keycode = 5;
            break;
        case "u":
            keycode = 6;
            break;
        case "i":
            keycode = 7;
            break;
        case "o":
            keycode = 8;
            break;
        case "p":
            keycode = 9;
            break;
        case "a":
            keycode = 10;
            break;
        case "s":
            keycode = 11;
            break;
        case "d":
            keycode = 12;
            break;
        case "f":
            keycode = 13;
            break;
        case "g":
            keycode = 14;
            break;
        case "h":
            keycode = 15;
            break;
        case "j":
            keycode = 16;
            break;
        case "k":
            keycode = 17;
            break;
        case "l":
            keycode = 18;
            break;
        case ";":
            keycode = 19;
            break;
        case "z":
            keycode = 20;
            break;
        case "x":
            keycode = 21;
            break;
        case "c":
            keycode = 22;
            break;
        case "v":
            keycode = 23;
            break;
        case "b":
            keycode = 24;
            break;
        case "n":
            keycode = 25;
            break;
        case "m":
            keycode = 26;
            break;
        case "Q":
            keycode = 100;
            break;
        case "W":
            keycode = 101;
            break;
        case "E":
            keycode = 102;
            break;
        case "R":
            keycode = 103;
            break;
        case "T":
            keycode = 104;
            break;
        case "Y":
            keycode = 105;
            break;
        case "U":
            keycode = 106;
            break;
        case "I":
            keycode = 107;
            break;
        case "O":
            keycode = 108;
            break;
        case "P":
            keycode = 109;
            break;
        case "A":
            keycode = 110;
            break;
        case "S":
            keycode = 111;
            break;
        case "D":
            keycode = 112;
            break;
        case "F":
            keycode = 113;
            break;
        case "G":
            keycode = 114;
            break;
        case "H":
            keycode = 115;
            break;
        case "J":
            keycode = 116;
            break;
        case "K":
            keycode = 117;
            break;
        case "L":
            keycode = 118;
            break;
        case "Z":
            keycode = 120;
            break;
        case "X":
            keycode = 121;
            break;
        case "C":
            keycode = 122;
            break;
        case "V":
            keycode = 123;
            break;
        case "B":
            keycode = 124;
            break;
        case "N":
            keycode = 125;
            break;
        case "M":
            keycode = 126;
            break;
        case ",":
            keycode = 27;
            break;
        case ".":
            keycode = 28;
            break;
        case "/":
            keycode = 29;
            break;
        case "1":
            keycode = 30;
            break;
        case "2":
            keycode = 31;
            break;
        case "3":
            keycode = 32;
            break;
        case "4":
            keycode = 33;
            break;
        case "5":
            keycode = 34;
            break;
        case "6":
            keycode = 35;
            break;
        case "7":
            keycode = 36;
            break;
        case "8":
            keycode = 37;
            break;
        case "9":
            keycode = 38;
            break;
        case "0":
            keycode = 39;
            break;
        case "`":
            keycode = 40;
            break;
        case "Escape":
            keycode = 41;
            break;
        case " ":
            keycode = 42;
            break;
        case "Shift":
            keycode = 43;
            break;
        case "Control":
            keycode = 44;
            break;
        case "Tab":
            keycode = 45;
            break;
        case "CapsLock":
            keycode = 46;
            break;
        case "Alt":
            keycode = 47;
            break;
        case "ArrowUp":
            keycode = 48;
            break;
        case "ArrowLeft":
            keycode = 49;
            break;
        case "ArrowDown":
            keycode = 50;
            break;
        case "ArrowRight":
            keycode = 51;
            break;
    }
}, true);

document.addEventListener('keyup', function(event) {
    keycode = -1;
}, true);

export function getkeycode(){
    return keycode;
}