export function set_val(str, val){
  localStorage.setItem(str, val);
}

export function get_val(str){
  if (!localStorage.getItem(str)) {
    localStorage.setItem(str, "{}");
    return "{}";
  } else {
    return localStorage.getItem(str);
  }
}