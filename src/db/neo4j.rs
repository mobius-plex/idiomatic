use neo4rs::{Graph, BoltClient};

pub async fn establish_connection() -> Graph {
    // Replace with your database credentials and URL
    let graph = Graph::new("neo4j://username:password@localhost:7687").await.unwrap();
    graph
}
