use actix_web::{web, App, HttpServer, Responder};
use neo4rs::{Graph, query};

async fn check_neo4j_connection() -> Result<String, Box<dyn std::error::Error>> {
    // Establish connection to Neo4j
    let graph = Graph::new("neo4j://username:password@localhost:7687").await?;

    // Execute a simple query
    let mut result = graph.execute(query("CALL dbms.components()")).await?;

    // Assuming the query returns a single row with a single column
    if let Some(row) = result.next().await? {
        // Extract the server version
        let version: String = row.get("version")?;
        return Ok(version);
    }

    Err("No data returned from Neo4j".into())
}

async fn greet() -> impl Responder {
    "Hello, Actix-Web!"
}

// Main function with Neo4j connection check
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Check Neo4j connection
    match check_neo4j_connection().await {
        Ok(version) => println!("Connected to Neo4j version: {}", version),
        Err(e) => eprintln!("Failed to connect to Neo4j: {}", e),
    }

    // Start the Actix-Web server
    HttpServer::new(|| {
        App::new().route("/", web::get().to(greet))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}