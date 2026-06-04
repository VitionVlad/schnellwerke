export function set_val(str, val){
    localStorage.setItem(str, val);
}

export function get_val(str){
    if (!localStorage.getItem(str)) {
      localStorage.setItem(str, 1.0);
      return 1.0;
    } else {
      return localStorage.getItem(str);
    }
}