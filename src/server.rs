use nickel::{Nickel, Mountable, StaticFilesHandler};
use nickel::HttpRouter;

pub fn run() {
    let mut server = Nickel::new();

    server.mount("/test/",
                 middleware! { |req|
        format!("Got request with uri = '{}'", req.origin.uri)
    });

    // Fall-through behaviour, if StaticFilesHandler does not find a matching file,
    // the request uri must be reset so that it can be matched against other middleware.
    //
    server.mount("/static/", StaticFilesHandler::new("static/"));

    server.mount("/static/files/",
                 middleware! { |req|
        let path = req.path_without_query().unwrap();
        format!("No static file with path '{}'!", path)
    });

    server.listen("127.0.0.1:8080").unwrap();
}