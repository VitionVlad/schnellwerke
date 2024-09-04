export class Jsrelod{
    constructor(iframeid){
        this.modeltext = document.getElementById(iframeid).contentWindow.document.body.innerText;
        this.st = this.modeltext.split('\n').join(' ').split(' ');
        this.xverts = [];
        this.yverts = [];
        this.zverts = [];

        this.xuv = [];
        this.yuv = [];

        this.xnormals = [];
        this.ynormals = [];
        this.znormals = [];

        this.iv = [];
        this.iu = [];
        this.in = [];

        console.log("SepModSize="+ this.st.length);

        for(var i = 0; i != this.st.length; i+=1){
            if(this.st[i]=="v"){
                this.xverts.push(parseFloat(this.st[i+1]));
                this.yverts.push(parseFloat(this.st[i+2]));
                this.zverts.push(parseFloat(this.st[i+3]));
            }
            if(this.st[i]=="vt"){
                this.xuv.push(parseFloat(this.st[i+1]));
                this.yuv.push(parseFloat(this.st[i+2]));
            }
            if(this.st[i]=="vn"){
                this.xnormals.push(parseFloat(this.st[i+1]));
                this.ynormals.push(parseFloat(this.st[i+2]));
                this.znormals.push(parseFloat(this.st[i+3]));
            }
            if(this.st[i]=="f"){
                var i1 = this.st[i+1].split("/");
                var i2 = this.st[i+2].split("/");
                var i3 = this.st[i+3].split("/");

                this.iv.push(this.xverts[parseInt(i1[0]-1)]);
                this.iv.push(this.yverts[parseInt(i1[0]-1)]);
                this.iv.push(this.zverts[parseInt(i1[0]-1)]);

                this.iv.push(this.xverts[parseInt(i2[0]-1)]);
                this.iv.push(this.yverts[parseInt(i2[0]-1)]);
                this.iv.push(this.zverts[parseInt(i2[0]-1)]);

                this.iv.push(this.xverts[parseInt(i3[0]-1)]);
                this.iv.push(this.yverts[parseInt(i3[0]-1)]);
                this.iv.push(this.zverts[parseInt(i3[0]-1)]);

                this.iu.push(this.xuv[parseInt(i1[1]-1)]);
                this.iu.push(this.yuv[parseInt(i1[1]-1)]);

                this.iu.push(this.xuv[parseInt(i2[1]-1)]);
                this.iu.push(this.yuv[parseInt(i2[1]-1)]);

                this.iu.push(this.xuv[parseInt(i3[1]-1)]);
                this.iu.push(this.yuv[parseInt(i3[1]-1)]);

                this.in.push(this.xnormals[parseInt(i1[2]-1)]);
                this.in.push(this.ynormals[parseInt(i1[2]-1)]);
                this.in.push(this.znormals[parseInt(i1[2]-1)]);

                this.in.push(this.xnormals[parseInt(i2[2]-1)]);
                this.in.push(this.ynormals[parseInt(i2[2]-1)]);
                this.in.push(this.znormals[parseInt(i2[2]-1)]);

                this.in.push(this.xnormals[parseInt(i3[2]-1)]);
                this.in.push(this.ynormals[parseInt(i3[2]-1)]);
                this.in.push(this.znormals[parseInt(i3[2]-1)]);
            }
        }
        this.iv32 = new Float32Array(this.iv);
        this.iu32 = new Float32Array(this.iu);
        this.in32 = new Float32Array(this.in);
        this.len = this.iv32.length/3;
        this.xverts = [];
        this.yverts = [];
        this.zverts = [];

        this.xuv = [];
        this.yuv = [];

        this.xnormals = [];
        this.ynormals = [];
        this.znormals = [];

        this.iv = [];
        this.iu = [];
        this.in = [];
    }
    getvert(){
        return this.iv32;
    }
    getuv(){
        return this.iu32;
    }
    getnorm(){
        return this.in32;
    }
    getlen(){
        return this.len;
    }
}

export class Jsloadsdf{
    constructor(iframeid){
        var modeltext = document.getElementById(iframeid).contentWindow.document.body.innerText;
        var st = modeltext.split('\n').join(' ').split(' ');
        console.log("SepSceneSize="+ st.length);
        var arr1 = [];
        var arr2 = [];
        for(var i = 0; i != st.length; i+=10){
            if(st[i] == "md"){ 
                arr1.push(parseFloat(st[i+1]));
                arr1.push(parseFloat(st[i+2]));
                arr1.push(parseFloat(st[i+3]));
                arr1.push(parseFloat(st[i+4]));
                arr1.push(parseFloat(st[i+5]));
                arr1.push(parseFloat(st[i+6]));
                arr1.push(parseFloat(st[i+7]));
                arr1.push(parseFloat(st[i+8]));
                arr1.push(parseFloat(st[i+9]));
            }
            if(st[i] == "cb"){ 
                arr2.push(parseFloat(st[i+1]));
                arr2.push(parseFloat(st[i+2]));
                arr2.push(parseFloat(st[i+3]));
                arr2.push(parseFloat(st[i+4]));
                arr2.push(parseFloat(st[i+5]));
                arr2.push(parseFloat(st[i+6]));
                arr2.push(parseFloat(st[i+7]));
                arr2.push(parseFloat(st[i+8]));
                arr2.push(parseFloat(st[i+9]));
            }
        }
        this.mdarr = new Float32Array(arr1);
        this.cbarr = new Float32Array(arr2);
    }
    getmd(){
        return this.mdarr;
    }
    getcb(){
        return this.cbarr;
    }
}