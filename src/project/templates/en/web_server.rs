pub fn code() -> String {
    String::from(
r#"@web_server
() -> 
    data:
        settings: 
            hostname: "localhost"
            port: 8989
        
        homepage: 
            title: "hello world"
            content: "hello world"
        
"#)
}