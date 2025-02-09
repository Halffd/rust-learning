use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::fs;
use super::db::{Database, DatabaseError};

// Shared state between handlers
pub struct AppState {
    db: Database,
}

async fn serve_file(path: &str) -> Result<Response<Body>, Infallible> {
    match fs::read(format!("src/server/static/{}", path)).await {
        Ok(content) => Ok(Response::builder()
            .header("Content-Type", "text/html")
            .body(Body::from(content))
            .unwrap()),
        Err(_) => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("File not found"))
            .unwrap()),
    }
}

async fn handle_request(
    req: Request<Body>,
    state: Arc<AppState>,
) -> Result<Response<Body>, Infallible> {
    let path = req.uri().path();
    let method = req.method();

    Ok(match (method, path) {
        (&Method::GET, "/") => serve_file("index.html").await?,
        (&Method::GET, "/login") => serve_file("login.html").await?,
        (&Method::GET, "/register") => serve_file("register.html").await?,
        (&Method::POST, "/register") => {
            let params = parse_form_data(req).await;
            let username = params.get("username").map(|s| s.as_str()).unwrap_or("");
            let password = params.get("password").map(|s| s.as_str()).unwrap_or("");

            match state.db.register_user(username, password).await {
                Ok(_) => Response::builder()
                    .status(StatusCode::FOUND)
                    .header("Location", "/login")
                    .body(Body::from("Registration successful! Please login."))
                    .unwrap(),
                Err(DatabaseError::UserExists) => Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(Body::from("Username already exists"))
                    .unwrap(),
                Err(_) => Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::from("Registration failed"))
                    .unwrap(),
            }
        }
        (&Method::POST, "/login") => {
            let params = parse_form_data(req).await;
            let username = params.get("username").map(|s| s.as_str()).unwrap_or("");
            let password = params.get("password").map(|s| s.as_str()).unwrap_or("");

            match state.db.verify_user(username, password).await {
                Ok(true) => Response::builder()
                    .status(StatusCode::FOUND)
                    .header("Location", "/dashboard")
                    .body(Body::empty())
                    .unwrap(),
                _ => Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body(Body::from("Invalid credentials"))
                    .unwrap(),
            }
        }
        (&Method::GET, "/dashboard") => serve_file("dashboard.html").await?,
        _ => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("Not Found"))
            .unwrap(),
    })
}

async fn parse_form_data(req: Request<Body>) -> std::collections::HashMap<String, String> {
    let body_bytes = hyper::body::to_bytes(req.into_body()).await.unwrap();
    let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
    
    body_str
        .split('&')
        .filter_map(|s| {
            let mut parts = s.split('=');
            Some((
                parts.next()?.to_string(),
                parts.next()?.to_string(),
            ))
        })
        .collect()
}

pub async fn start_server() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize database
    let db = Database::connect("data.db").await?;
    let state = Arc::new(AppState { db });

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);

    let make_svc = make_service_fn(move |_conn| {
        let state = state.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                let state = state.clone();
                handle_request(req, state)
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);
    server.await?;

    Ok(())
}
