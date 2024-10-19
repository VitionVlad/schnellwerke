export class Jsrelod{
    constructor(iframeid){
        var modeltext = document.getElementById(iframeid).contentWindow.document.body.innerText;
        var st = modeltext.split('\n').join(' ').split(' ');
        var xverts = [];
        var yverts = [];
        var zverts = [];

        var xuv = [];
        var yuv = [];

        var xnormals = [];
        var ynormals = [];
        var znormals = [];

        var iv = [];
        var iu = [];
        var inn = [];

        console.log("OBJLoader: SepModSize="+ st.length);

        for(var i = 0; i != st.length; i+=1){
            if(st[i]=="v"){
                xverts.push(parseFloat(st[i+1]));
                yverts.push(parseFloat(st[i+2]));
                zverts.push(parseFloat(st[i+3]));
            }
            if(st[i]=="vt"){
                xuv.push(parseFloat(st[i+1]));
                yuv.push(parseFloat(st[i+2]));
            }
            if(st[i]=="vn"){
                xnormals.push(parseFloat(st[i+1]));
                ynormals.push(parseFloat(st[i+2]));
                znormals.push(parseFloat(st[i+3]));
            }
            if(st[i]=="f"){
                var i1 = st[i+1].split("/");
                var i2 = st[i+2].split("/");
                var i3 = st[i+3].split("/");

                iv.push(xverts[parseInt(i1[0]-1)]);
                iv.push(yverts[parseInt(i1[0]-1)]);
                iv.push(zverts[parseInt(i1[0]-1)]);

                iv.push(xverts[parseInt(i2[0]-1)]);
                iv.push(yverts[parseInt(i2[0]-1)]);
                iv.push(zverts[parseInt(i2[0]-1)]);

                iv.push(xverts[parseInt(i3[0]-1)]);
                iv.push(yverts[parseInt(i3[0]-1)]);
                iv.push(zverts[parseInt(i3[0]-1)]);

                iu.push(xuv[parseInt(i1[1]-1)]);
                iu.push(yuv[parseInt(i1[1]-1)]);

                iu.push(xuv[parseInt(i2[1]-1)]);
                iu.push(yuv[parseInt(i2[1]-1)]);

                iu.push(xuv[parseInt(i3[1]-1)]);
                iu.push(yuv[parseInt(i3[1]-1)]);

                inn.push(xnormals[parseInt(i1[2]-1)]);
                inn.push(ynormals[parseInt(i1[2]-1)]);
                inn.push(znormals[parseInt(i1[2]-1)]);

                inn.push(xnormals[parseInt(i2[2]-1)]);
                inn.push(ynormals[parseInt(i2[2]-1)]);
                inn.push(znormals[parseInt(i2[2]-1)]);

                inn.push(xnormals[parseInt(i3[2]-1)]);
                inn.push(ynormals[parseInt(i3[2]-1)]);
                inn.push(znormals[parseInt(i3[2]-1)]);
            }
        }
        this.iv32 = new Float32Array(iv);
        this.iu32 = new Float32Array(iu);
        this.in32 = new Float32Array(inn);
        this.len = this.iv32.length/3;
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
        console.log("SDFLoader: SepSceneSize="+ st.length);
        var arr1 = [];
        var arr2 = [];
        var arr3 = [];
        var arr4 = [];
        var arr5 = [];
        for(var i = 0; i != st.length; i+=1){
            if(st[i] == "md"){ 
                console.log("SDFLoader: found model mesh at index ="+ i);
                arr1.push(parseFloat(st[i+1]));
                arr1.push(parseFloat(st[i+2]));
                arr1.push(parseFloat(st[i+3]));
                arr1.push(parseFloat(st[i+4]));
                arr1.push(parseFloat(st[i+5]));
                arr1.push(parseFloat(st[i+6]));
                arr1.push(parseFloat(st[i+7]));
                arr1.push(parseFloat(st[i+8]));
                arr1.push(parseFloat(st[i+9]));
                arr1.push(parseFloat(st[i+10]));
            }
            if(st[i] == "cs"){ 
                console.log("SDFLoader: found cube mesh at index ="+ i);
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
            if(st[i] == "cu"){ 
                console.log("SDFLoader: found cubeuv mesh at index ="+ i);
                arr3.push(parseFloat(st[i+1]));
                arr3.push(parseFloat(st[i+2]));
                arr3.push(parseFloat(st[i+3]));
                arr3.push(parseFloat(st[i+4]));
                arr3.push(parseFloat(st[i+5]));
                arr3.push(parseFloat(st[i+6]));
                arr3.push(parseFloat(st[i+7]));
                arr3.push(parseFloat(st[i+8]));
                arr3.push(parseFloat(st[i+9]));
            }
            if(st[i] == "pl"){ 
                arr4.push(parseFloat(st[i+1]));
                arr4.push(parseFloat(st[i+2]));
                arr4.push(parseFloat(st[i+3]));
                arr4.push(parseFloat(st[i+4]));
                arr4.push(parseFloat(st[i+5]));
                arr4.push(parseFloat(st[i+6]));
                arr4.push(parseFloat(st[i+7]));
                arr4.push(parseFloat(st[i+8]));
                arr4.push(parseFloat(st[i+9]));
                console.log("SDFLoader: found plane mesh at index ="+ i);
            }
            if(st[i] == "mat"){ 
                console.log("SDFLoader: found material at index ="+ i);
                arr5.push(parseFloat(st[i+1]));
                arr5.push(parseFloat(st[i+2]));
                arr5.push(parseFloat(st[i+3]));
                for(var b = 0; b < parseFloat(st[i+3]); b+=1){
                    arr5.push(parseFloat(st[i+4+b]));
                }
            }
        }
        this.mdarr = new Float32Array(arr1);
        this.cbarr = new Float32Array(arr2);
        this.cuarr = new Float32Array(arr3);
        this.plarr = new Float32Array(arr4);
        this.matarr = new Float32Array(arr5);
        console.log("SDFLoader: SceneArrLen="+ this.mdarr.length/10 + " " + this.cbarr.length/9 + " " + this.cuarr.length/9 + " " + this.plarr.length/9 + " " + this.matarr.length);
    }
    getmd(){
        return this.mdarr;
    }
    getcb(){
        return this.cbarr;
    }
    getcu(){
        return this.cuarr;
    }
    getpl(){
        return this.plarr;
    }
    getmat(){
        return this.matarr;
    }
}

export function get_text_iframe(id){
    console.log("TextLoader: trying to get text from iframe id = "+ id);
    return document.getElementById(id).contentWindow.document.body.innerText;
}

export function remove_elem(id){
    document.getElementById(id).remove();
}