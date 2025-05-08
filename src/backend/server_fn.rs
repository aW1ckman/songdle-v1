use dioxus::prelude::*;


// #[cfg(feature = "server")]
// let OAUTH: String = {
//     use std::fs;

//     let file = fs::File::open("etc/secret.json").expect("Secret should open");
//     let data: serde_json::Value = serde_json::from_reader(file).expect("Secret JSON parsing error");
//     let p = data.get("oauth").unwrap().to_string();
//     p
// };

#[server]
pub async fn echo_server(input: String) -> Result<String, ServerFnError> {
    // The body of server function like this comment are only included on the server. If you have any server-only logic like
    // database queries, you can put it here. Any imports for the server function should either be imported inside the function
    // or imported under a `#[cfg(feature = "server")]` block.
    use crate::s_context::AppState;
    let FromContext(context): FromContext<AppState> = extract().await?;
    Ok(format!("{input}{c}", c=context.oauth))
}

