use std::sync::Mutex;
use actix_web::{web, Responder, post, get};
use crate::error::{HarnessError, HarnessErrorType, HarnessResult};
use crate::Agent;
use crate::controllers::Request;
use aries_vcx::handlers::issuance::issuer::issuer::Issuer;

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct CredentialRevocationData {
    cred_rev_id: String,
    rev_registry_id: String,
    publish_immediately: bool,
}

impl Agent {
    pub fn revoke_credential(&mut self, revocation_data: &CredentialRevocationData) -> HarnessResult<String> {
        let issuer: Issuer = self.dbs.issuer.get(&revocation_data.rev_registry_id)
            .ok_or(HarnessError::from_msg(HarnessErrorType::NotFoundError, &format!("Issuer with id {} not found", &revocation_data.rev_registry_id)))?;
        issuer.revoke_credential(revocation_data.publish_immediately)?;
        self.dbs.issuer.set(&issuer.get_source_id()?, &issuer)?;
        Ok("OK".to_string())
    }

    pub fn get_revocation_registry(&mut self, id: &str) -> HarnessResult<String> {
        let issuer: Issuer = self.dbs.issuer.get(id)
            .ok_or(HarnessError::from_msg(HarnessErrorType::NotFoundError, &format!("Credential definition with id {} not found", id)))?;
        let rev_reg_id = issuer.get_rev_reg_id().unwrap();
        self.dbs.issuer.set(rev_reg_id, &issuer)?;
        Ok(json!({ "revocation_id": rev_reg_id, "revoc_reg_id": rev_reg_id }).to_string())
    }
}

#[post("/revoke")]
pub async fn revoke_credential(agent: web::Data<Mutex<Agent>>, req: web::Json<Request<CredentialRevicationData>>) -> impl Responder {
    agent.lock().unwrap().revoke_credential(&req.data)
        .with_header("Cache-Control", "private, no-store, must-revalidate")
}

#[get("/{cred_id}")]
pub async fn get_revocation_registry(agent: web::Data<Mutex<Agent>>, path: web::Path<String>) -> impl Responder {
    agent.lock().unwrap().get_revocation_registry(&path.into_inner())
        .with_header("Cache-Control", "private, no-store, must-revalidate")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service( web::scope("/response/revocation-registry").service(get_revocation_registry))
        .service(web::scope("/command/revocation").service(revoke_credential));
