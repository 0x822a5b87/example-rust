// the lifetime of return value is equivalent to the lifetime of contents,
// because it is a slice refer to contents
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut v = Vec::new();
    for line in contents.lines().into_iter() {
        if line.contains(query) {
            v.push(line);
        }
    }
    v
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query_lower = query.to_lowercase();
    let mut v = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query_lower) {
            v.push(line);
        }
    }
    v
}

// pub fn search_case_insensitive_another<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
//     let query_lower = query.to_lowercase();
//     let contents_lower = contents.to_lowercase();
//     // error[E0515]: cannot return value referencing local variable `contents_lower`
//     //   --> src/search/mod.rs:29:5
//     //    |
//     // 29 |     search_case_insensitive(&query_lower, &contents_lower)
//     //    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---------------^
//     //    |     |                                     |
//     //    |     |                                     `contents_lower` is borrowed here
//     //    |     returns a value referencing data owned by the current function
//     search_case_insensitive(&query_lower, &contents_lower)
// }
