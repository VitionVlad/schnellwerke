export class Jsaudio{
    constructor(url){
        this.src = new Audio(url);
    }
    play(){
        if(this.src.played == this.src.duration){
            this.src.currentTime=0;
        }
        this.src.play();
    }
    stop(){
        this.src.stop();
    }
    pause(){
        this.src.pause();
    }
    setvolume(vol){
        this.src.volume = vol;
    }
    settime(time){
        this.src.currentTime = time;
    }
    ended(){
        return this.src.played == this.src.duration;
    }
}