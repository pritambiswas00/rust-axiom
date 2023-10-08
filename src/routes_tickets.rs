use axum::Router;
use axum::extract::Path;
use axum::routing::{post, delete};
use axum::{extract::State, Json};
use crate::model::{ ModelController, Ticket,TicketForCreate };
use crate::error::Result;


pub fn router(mc:ModelController)->Router {
      Router::new()
      .route("/tickets", post(create_ticket).get(list_tickets))
      .route("/tickets/:id", delete(delete_ticket))
      .with_state(mc)
}

async fn create_ticket(
    mc:State<ModelController>,
    Json(ticket):Json<TicketForCreate>
)->Result<Json<Ticket>>{
     println!("--> {:<12} - create ticket ", "HANDLER");
     let ticket = mc.create_ticket(ticket).await?;
     Ok(Json(ticket))
}

async fn list_tickets (
    State(mc) : State<ModelController>
) -> Result<Json<Vec<Ticket>>> {
      
      let tickets = mc.list_tickets().await?;
      Ok(Json(tickets))
}


async fn delete_ticket(
    State(mc): State<ModelController>,
    Path(id) : Path<u64>
)->Result<Json<Ticket>> {
     println!(".. {:<15} - Delete ticket", "Handler");
     let ticket = mc.delete_ticket(id).await?;
     Ok(Json(ticket))
}