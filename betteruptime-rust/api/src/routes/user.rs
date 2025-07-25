use std::sync::{Arc, Mutex};

use poem::{
    handler, web::{Data, Json}
};
use crate::request_inputs::{CreateUserInput};
use crate::request_outputs::{CreateUserOutput, SigninOutput};
use store::store::Store;

#[handler]
pub fn sign_up(Json(data): Json<CreateUserInput>, Data(s): Data<&Arc<Mutex<Store>>>) -> Json<CreateUserOutput> {
    let mut locked_s = s.lock().unwrap();
    let id = locked_s.sign_up(data.username, data.password).unwrap();

    let response = CreateUserOutput {
        id
    };
    
    Json(response)
}

#[handler]
pub fn sign_in(Json(data): Json<CreateUserInput>, Data(s): Data<&Arc<Mutex<Store>>>) -> Json<SigninOutput> {
    let mut locked_s = s.lock().unwrap();
    let _exists = locked_s.sign_in(data.username, data.password).unwrap();

    let response = SigninOutput {
        jwt: String::from("harkirtat")
    };
    
    Json(response)
}