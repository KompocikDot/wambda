use http::{
    HeaderMap,
    header::{AUTHORIZATION, COOKIE, SET_COOKIE},
};

// Platform should not forward some of it's headers
pub async fn run(_query: &str, headers: &HeaderMap) -> (u16, String) {
    if headers.contains_key(AUTHORIZATION)
        || headers.contains_key(COOKIE)
    {
        return (crate::FAIL, String::from(""));
    }
    return (crate::PASS, String::from(""));
}
