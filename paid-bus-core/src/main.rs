use proto::paidbus::ticket::{self, NewTicketReply};
use sqlx::SqlitePool;
use tonic::{Request, Response, Status};

mod proto;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
}

struct Backend {
    conn: SqlitePool,
}

impl Backend {
    async fn new() -> Self {
        let conn = SqlitePool::connect("sqlite://./db.sqlite3").await.unwrap();

        // TODO: create tables

        Self { conn }
    }

    fn get_tickets(&self) -> Vec<String> {
        // TODO: get from database
        vec![]
    }
}

#[tonic::async_trait]
impl ticket::ticket_server::Ticket for Backend {
    async fn new_ticket(
        &self,
        request: Request<ticket::NewTicketRequest>,
    ) -> Result<Response<ticket::NewTicketReply>, Status> {
        let tickets = self.get_tickets();

        Ok(Response::new(NewTicketReply { tickets }))
    }
}
