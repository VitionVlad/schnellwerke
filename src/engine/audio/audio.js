class beethoven{
    constructor(){
        this.ctx = [];
        this.src = [];
    }
}

var bh = new beethoven();

class Jsaudioctx{
    constructor(){
        this.audioCtx = null;
        let self = this;
        this.volume = 1.0;
        document.addEventListener("click", () => {
            if(self.audioCtx == null){
                self.audioCtx = new AudioContext();
                self.audioCtx.resume();
            }
        })
    }
}

class Jsaudiosource{
    createsrc(){
        if(this.ctx.audioCtx != null && !this.auc){
            this.loop = false;
            this.audioElement = new Audio(this.audiosrc);
            this.audioElement.loop = this.loop;
            this.track = this.ctx.audioCtx.createMediaElementSource(this.audioElement);
            this.panner = new StereoPannerNode(this.ctx.audioCtx);
            this.gainNode = this.ctx.audioCtx.createGain();
            this.track.connect(this.gainNode).connect(this.panner).connect(this.ctx.audioCtx.destination);
            this.pl = false;
            this.auc = true;
        }
    }
    constructor(ctx, audiosrc){
        this.ctx = bh.ctx[ctx];
        this.ci = ctx;
        this.audiosrc = audiosrc;
        this.auc = false;
        this.loop = true;
        this.createsrc();
    }
    setrelxy(px){
        this.createsrc();
        if(this.ctx.audioCtx != null){
            this.panner.pan.value = px;
        }
    }
    setvolume(gainValue) {
        this.createsrc();
        if(this.ctx.audioCtx != null){
            this.gainNode.gain.value = Math.max(0, Math.min(gainValue, 1));
        }
    }
    play(){
        this.createsrc();
        if(this.ctx.audioCtx != null){
            if(!this.pl){
                this.audioElement.play();
                this.pl = !this.pl;
            }
        }
    }
    pause(){
        this.createsrc();
        if(this.ctx.audioCtx != null){
            if(this.pl){
                this.audioElement.pause();
                this.pl = !this.pl;
            }
        }
    }
    setloop(val){
        this.loop = val;
        this.createsrc();
        if(this.ctx.audioCtx != null){
            this.audioElement.loop = this.loop;
        }
    }
    setpos(val){
        this.createsrc();
        if(this.ctx.audioCtx != null){
            this.audioElement.currentTime = val;
            this.audioElement.play();
            this.pl = true;
        }
    }
    ended(){
        if(this.audioElement){
            return this.audioElement.ended ? 1 : 0;
        }
        return 0;
    }
}

export function newmozart(){
    let mhi = bh.ctx.length;
    bh.ctx.push(new Jsaudioctx());
    return mhi;
}
export function mozartsetvolume(mhi, vol){
    bh.ctx[mhi].volume = vol;
}
export function newsound(mhi, path){
    let msn = bh.src.length;
    bh.src.push(new Jsaudiosource(mhi, path));
    return msn;
}
export function soundplay(msn, pan, vol){
    bh.src[msn].setvolume(vol * bh.src[msn].ctx.volume);
    bh.src[msn].setrelxy(pan);
    bh.src[msn].play();
}
export function soundstop(msn){
    bh.src[msn].pause();
}
export function soundsetloop(msn, val){
    bh.src[msn].setloop(val);
}
export function soundsetpos(msn, val){
    bh.src[msn].setpos(val);
}
export function cend(msn){
    return bh.src[msn].ended();
}
export function destroymozart(mhi){
}