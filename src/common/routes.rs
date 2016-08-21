// src/common/routes.rs

/// MM Routes

// Import Modules
// External
use nickel::{HttpRouter, MediaType, Nickel, Router};
// Common Utilities
use ::common::api_result::{JsonEncoder};
// Controllers
use ::controllers::controller_manager::ControllerManager;

pub fn get_routes() -> Router<ControllerManager> {
    let mut router = Nickel::router();
    router.get("/", middleware! { |_request, mut response|
        info!("API Endpoint: /");
        response.set(MediaType::Json);
        format!(r#"{{"name":"Money Map", "version":"{}", "status":"ok"}}"#, env!("CARGO_PKG_VERSION"))
    });

    //Test Actions
    router.get("/test/retrieve", middleware! { |request, mut response|
        info!("API Endpoint: POST /test/retrieve");
        let cm: &ControllerManager = request.server_data();
        let result = &cm.test_controller.retrieve();

        response.set(MediaType::Json);
        JsonEncoder::encode(result)
    });

    router.post("/test/save", middleware! { |request, mut response|
        info!("API Endpoint: POST /test/save");
        let cm: &ControllerManager = request.server_data();
        let result = &cm.test_controller.save(request);

        response.set(MediaType::Json);
        JsonEncoder::encode(result)
    });

    router.get("/test/failure", middleware! { |request, mut response|
        info!("API Endpoint: GET /test/failure");
        let cm: &ControllerManager = request.server_data();
        let result = &cm.test_controller.failure();

        response.set(MediaType::Json);
        JsonEncoder::encode(result)
    });

    //Users Actions
    router.get("/users", middleware! { |_request, mut response|
        info!("API Endpoint: GET /users");
        response.set(MediaType::Json);
        //let user = Controllers::users::PubUser::new("John".to_string(), "Smith".to_string(), "test@test.com".to_string());
        format!("{{\"status\":\"success\"}}")
    });

    router.post("/users", middleware! { |request, mut response|
        info!("API Endpoint: POST /users");
        let cm: &ControllerManager = request.server_data();
        let result = &cm.users_controller.create(request);

        response.set(MediaType::Json);
        JsonEncoder::encode(result)
    });
router
}
