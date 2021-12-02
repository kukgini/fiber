use std::sync::Mutex;
use std::collections::HashMap;
use actix_web::{web, Responder, post, get}};
use uuid;
use crate::error::{HarnessError, HarnessErrorType, HarnessResult};
use crate::{Agent, State};
use crate::controllers::Request;
use aries_vcx::messages::proof_presentation::presentation_request::PresentationRequest as VcxPresentationRequest;
use aries_vcx::messages::a2a::A2AMessage;
use aries_vcx::messages::status::Status;
use aries_vcx::handlers::proof_presentation::verifier::verifier::{Verifier, VerifierState};
use aries_vcx::handlers::proof_presentation::prover::prover::{Prover, ProverState};
use aries_vcx::messages::proof_presentation::presentation_protocol::{PresentationProposalData, Attribute, Predicate};
use aries_vcx::handlers::connection::connection::Connection;
use crate::soft_assert_eq;