export async function openfs(path){
    const str = await fetch(path).then( r => r.bytes());
    return str;
}