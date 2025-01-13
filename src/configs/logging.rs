use crate::configs::get_resources;

pub fn init_logging() {
    let resources_path = get_resources();
    log4rs::init_file(format!("{resources_path}/log4rs.yaml"), Default::default()).unwrap();
}
