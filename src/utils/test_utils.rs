#[allow(dead_code)]
pub fn str_vec_to_string_vec(str_vec: Vec<&str>) -> Vec<String> {
    str_vec.into_iter().map(|str| (*str).to_string()).collect()
}

#[allow(dead_code)]
pub fn str_vec_2d_to_string_vec_2d(str_vec_2d: Vec<Vec<&str>>) -> Vec<Vec<String>> {
    str_vec_2d.into_iter().map(str_vec_to_string_vec).collect()
}

#[allow(dead_code)]
pub fn str_vec_2d_contents_same(strs1: Vec<Vec<String>>, strs2: Vec<Vec<String>>) -> bool {
    let mut sorted_strs1: Vec<String> = strs1
        .into_iter()
        .map(|mut strs| {
            strs.sort();
            strs.join("")
        })
        .collect();
    let mut sorted_strs2: Vec<String> = strs2
        .into_iter()
        .map(|mut strs| {
            strs.sort();
            strs.join("")
        })
        .collect();
    sorted_strs1.sort();
    sorted_strs2.sort();
    sorted_strs1 == sorted_strs2
}
