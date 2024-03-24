use super::ahijor_error::Error;
use axum::routing::Router;
use std::fs;
use std::path::Path;
use tower_http::services::ServeDir;

const LOCAL_HOST: &str = r#"

Starting local server!!

Connect to http://localhost:3030
"#;

pub async fn start_local_server() -> Result<(), Error> {
    println!("Start local server function");
    let app: Router = add_routes_for_dir(Router::new(), Path::new("build"), "")?;

    let listen: tokio::net::TcpListener = match tokio::net::TcpListener::bind("0.0.0.0:3030").await
    {
        Ok(listener) => listener,
        Err(e) => return Err(Error::from(e)),
    };

    println!("{}", LOCAL_HOST);
    if let Err(e) = axum::serve(listen, app.into_make_service()).await {
        return Err(Error::from(e));
    };
    Ok(())
}

fn add_routes_for_dir(mut router: Router, dir: &Path, base_path: &str) -> Result<Router, Error> {
    let mut dirs_to_visit: Vec<(std::path::PathBuf, String)> =
        vec![(dir.to_path_buf(), base_path.to_string())];

    let mut routes_to_add: Vec<(String, std::path::PathBuf)> = Vec::new();

    while let Some((dir, base_path)) = dirs_to_visit.pop() {
        for entry in fs::read_dir(dir.clone())? {
            let entry: fs::DirEntry = entry?;
            let path: std::path::PathBuf = entry.path();
            if path.is_dir() {
                let route_path: String = format!(
                    "{}/{}",
                    base_path,
                    path.file_name().unwrap().to_str().unwrap()
                );
                println!("route_path is {}", route_path);
                println!("--------------------------------");
                dirs_to_visit.push((path, route_path));
            } else if path.file_name().unwrap() == "index.html" {
                let route_path: String = base_path.to_string();
                println!("dir : {:?}", &dir);
                routes_to_add.push((route_path.clone(), dir.to_path_buf()));
                println!("routes_to_add : {:?}", &routes_to_add);
            }
        }
    }
    for (route_path, path) in routes_to_add {
        println!("route_path is {}", &route_path);
        let nested_route: String = format!("{}/", route_path);
        println!("nested_route : {}", &nested_route);
        println!("path : {:?}", &path);
        router = router
            .route(
                nested_route.as_str(),
                axum::routing::get_service(ServeDir::new("./build")),
            )
            .route_service(
                // Double registration error
                "/favicon.ico",
                axum::routing::get_service(ServeDir::new("./build")),
            );
    }
    Ok(router)
}
