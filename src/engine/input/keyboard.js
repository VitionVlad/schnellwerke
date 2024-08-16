var keycode = -1;

document.addEventListener('keydown', function(event) {
    switch(event.key){
        case "q" || "Q":
            keycode = 0;
            break;
        case "w" || "W":
            keycode = 1;
            break;
        case "e" || "E":
            keycode = 2;
            break;
        case "r" || "R":
            keycode = 3;
            break;
        case "t" || "T":
            keycode = 4;
            break;
        case "y" || "Y":
            keycode = 5;
            break;
        case "u" || "U":
            keycode = 6;
            break;
        case "i" || "I":
            keycode = 7;
            break;
        case "o" || "O":
            keycode = 8;
            break;
        case "p" || "P":
            keycode = 9;
            break;
        case "a" || "A":
            keycode = 10;
            break;
        case "s" || "S":
            keycode = 11;
            break;
        case "d" || "D":
            keycode = 12;
            break;
        case "f" || "F":
            keycode = 13;
            break;
        case "g" || "G":
            keycode = 14;
            break;
        case "h" || "H":
            keycode = 15;
            break;
        case "j" || "J":
            keycode = 16;
            break;
        case "k" || "K":
            keycode = 17;
            break;
        case "l" || "L":
            keycode = 18;
            break;
        case ";":
            keycode = 19;
            break;
        case "z" || "Z":
            keycode = 20;
            break;
        case "x" || "X":
            keycode = 21;
            break;
        case "c" || "C":
            keycode = 22;
            break;
        case "v" || "V":
            keycode = 23;
            break;
        case "b" || "B":
            keycode = 24;
            break;
        case "n" || "N":
            keycode = 25;
            break;
        case "m" || "M":
            keycode = 26;
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