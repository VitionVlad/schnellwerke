export class Jskeyboard{
    key_to_code(key){
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
    constructor(){
        this.keycodes = new Int8Array(256);
        this.lastkey = -1;
        var that = this;
        document.addEventListener('keydown', function(event) {
            var id = that.key_to_code(event.key);
            that.lastkey = id;
            if(id != -1){
                that.keycodes[id] = 1;
            }
        }, true);
        document.addEventListener('keyup', function(event) {
            var id = that.key_to_code(event.key);
            if(id != -1){
                that.keycodes[id] = -1;
            }
        }, true);
    }
    getkey(keyid){
        return this.keycodes[keyid];
    }
    getlastkey(){
        return this.lastkey;
    }
}

export class Jsmouse{
    constructor(){
        this.px = 0;
        this.py = 0;
        this.mlc = false;
        this.mrc = false;
        this.mmc = false;
        var self = this;
        document.addEventListener("mousemove", function(event){
            self.px = event.movementX;
            self.py = event.movementY;
        }, false);   
        document.onclick = function(e){
            switch (e.button) {
              case 0:
                self.mlc = true;
                break;
              case 1:
                self.mmc = true;
                break;
              case 2:
                self.mrc = true;
                break;
              default:
                console.log(`Unknown button code: ${e.button}`);
            }
        }
    }
    jgetx(){ 
        var t = this.px;
        this.px = 0;
        return Number(t); 
    }
    jgety(){
        var t = this.py;
        this.py = 0;
        return Number(t); 
    }
    getmlc(){
        var t = this.mlc;
        this.mlc = 0;
        return Boolean(t);
    }
    getmmc(){
        var t = this.mmc;
        this.mmc = 0;
        return Boolean(t);
    }
    getmrc(){
        var t = this.mrc;
        this.mrc = 0;
        return Boolean(t);
    }
}

export class Jstouch{
    constructor(){
        this.tx = 0;
        this.ty = 0;
        this.inuse = false;
        this.index = 0;
        var self = this;
        document.addEventListener("touchmove", function(event){
            self.tx = event.touches[self.index].clientX;
            self.ty = event.touches[self.index].clientY;
        }, false);     
        document.addEventListener("touchend", function(){
            self.inuse = false;
        }, false); 
        document.addEventListener("touchstart", function(){
            self.inuse = true;
        }, false); 
    }
    jgettx(){
        return Number(this.tx); 
    }
    jgetty(){
        return Number(this.ty); 
    }
    jmaxx(){
        return Number(this.tx); 
    }
    jmaxy(){
        return Number(this.ty); 
    }
    jgetuse(){
        return Number(this.inuse); 
    }
    jsettouchindex(lindex){
        this.index = lindex;
    }
}

export class Jsgamepad{
    constructor(){
        this.gamepad = [];
        var self = this;
        window.addEventListener("gamepadconnected", function(e){
            self.gamepad.push(e.gamepad);
        });
        window.addEventListener("gamepaddisconnected", function(e){
            self.gamepad.push(e.gamepad);
        });
    }
    getgamepadnum(){
        return this.gamepad.length;
    }
    getgamepadbnum(gi){
        return this.gamepad[gi].button.length;
    }
    getgamepadanum(gi){
        return this.gamepad[gi].axis.length;
    }
    getgamepadaxis(gi, ai){
        return this.gamepad[gi].axis[ai];
    }
    getbuttonpressed(gi, bi){
        return this.gamepad[gi].button[bi].pressed;
    }
}