pub use tonic;

pub mod google {
    pub mod protobuf {
        tonic::include_proto!("google.protobuf");
    }
}
pub mod models {
    pub mod controller {
        tonic::include_proto!("models.controller");
        tonic::include_proto!("models.controller.serde");
    }
}
pub mod services {
    pub mod controller {
        tonic::include_proto!("services.controller");
        tonic::include_proto!("services.controller.serde");
    }
    pub mod translation {
        tonic::include_proto!("services.translation");
        tonic::include_proto!("services.translation.serde");
    }
}
