use std::sync::Mutex;
use actix_web::{web, Responder, post, get};
use uuid;
use crate::{Agent, State};
use crate::error::{HarnessError, HarnessErrorType, HarnessResult};
use crate::controllers::Request;
use aries_vcx::messages::connection::invite::{Invitation, PairwiseInvitation};
use aries_vcx::handlers::connection::connection::{Connection, ConnectionState};
use aries_vcx::handlers::connection::invitee::state_machine::InviteeState;
use aries_vcx::handlers::connection::inviter::state_machine::InviterState;

#[derive(Deserialize, Default)]
struct ConnectionRequest {
    request: String
}

#[derive(Deserialize, Default)]
struct Comment {
    comment: String
}

fn _get_state(connection: &Connection) -> State {
    match connection.get_state() {
        ConnectionState::Invitee(state) => match state {
            InviteeState::Initial => State::Initial,
            InviteeState::Invited => State::Invited,
            InviteeState::Requested => State::Requested,
            InviteeState::Responded => State::Responded,
            InviteeState::Completed => State::Complete
        }
        ConnectionState::Inviter(state) => match state {
            InviterState::Initial => State::Initial,
            InviterState::Invited => State::Invited,
            InviterState::Requested => State::Requested,
            InviterState::Responded => State::Responded,
            InviterState::Completed => State::Complete
        }
    }
}

impl Agent {
    pub fn create_invitation(&mut self) -> HarnessResult<String> {
        let id = uuid::Uuid::new_v4().to_string();
        let mut connection = Connection::create(&id, false).map_err(|err| HarnessError::from(err))?;
        connection.connect().map_err(|err| HarnessError::from(err))?;
        let invite = connection.get_invite_details()
            .ok_or(HarnessError::from_msg(HarnessErrorType::InternalServerError, "Failed to get invite details"))?;
        self.dbs.connection.set(&id, &connection).map_err(|err| HarnessError::from(err))?;
        Ok(json!({ "connection_id": id, "invitation": invite }).to_string())
    }
}

#[post("/create-invitation")]
pub async fn create_invitation(agent: web::Data<Mutex<Agent>>) -> impl Responder {
    agent.lock().unwrap().create_invitation()
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::scope("/command/connection")
                .service(create_invitation)
        )
        .service(
            web::scope("/response/connection")
        );
}
