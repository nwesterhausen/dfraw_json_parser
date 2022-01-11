pub fn serve_files(directory: String, port: u16) {
    
    let server_string = format!("localhost:{}", port);
    println!("Starting server at http://{}", server_string);

    rouille::start_server(&server_string, move |request| {
        {
            // The `match_assets` function tries to find a file whose name corresponds to the URL
            // of the request. The second parameter (`"."`) tells where the files to look for are
            // located.
            // In order to avoid potential security threats, `match_assets` will never return any
            // file outside of this directory even if the URL is for example `/../../foo.txt`.
            let response = rouille::match_assets(&request, &directory);

            // If a file is found, the `match_assets` function will return a response with a 200
            // status code and the content of the file. If no file is found, it will instead return
            // an empty 404 response.
            // Here we check whether if a file is found, and if so we return the response.
            if response.is_success() {
                return response;
            }
        }

        // This point of the code is reached only if no static file matched the request URL.

        // In a real website you probably want to serve non-static files here (with the `router!`
        // macro for example), but here we just return a 404 response.
        rouille::router!(request,
            (GET) (/) => {
                // If you requested '/' redirect to index.html
                rouille::Response::redirect_302("/index.html")
            },

            _ => rouille::Response::empty_404()
        )
    });
}