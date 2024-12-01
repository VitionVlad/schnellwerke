export class Jsaudioctx{
    constructor(){
        this.audioCtx = null;
        let self = this;
        document.addEventListener("click", () => {
            self.audioCtx = new AudioContext();
            self.audioCtx.resume();
        })
    }
}

export class Jsaudiosource{
    createsrc(){
        if(this.ctx.audioCtx != null && !this.auc){
            this.audioElement = document.getElementById(this.id);
            this.track = this.ctx.audioCtx.createMediaElementSource(this.audioElement);
            this.panner = new StereoPannerNode(this.ctx.audioCtx);
            this.gainNode = this.ctx.audioCtx.createGain();
            this.track.connect(this.gainNode).connect(this.panner).connect(this.ctx.audioCtx.destination);
            this.pl = false;
            this.auc = true;
        }
    }
    constructor(ctx, id){
        this.ctx = ctx;
        this.id = id;
        this.auc = false;
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
}