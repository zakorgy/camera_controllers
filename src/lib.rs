#![crate_name = "camera_controllers"]

//! A library for 3D camera control

#[macro_use]
extern crate bitflags;
extern crate input;
extern crate vecmath;
extern crate quaternion;
extern crate cam;

pub use cam::{
    Camera,
    CameraPerspective,
    model_view_projection,
};

pub use first_person::{
    FirstPerson,
    FirstPersonSettings,
};

pub use orbit_zoom_camera::{
    OrbitZoomCamera,
    OrbitZoomCameraSettings,
    Keys,
};

mod first_person;
mod orbit_zoom_camera;
