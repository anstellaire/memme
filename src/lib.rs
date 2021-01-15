// crates
#[macro_use]
extern crate diesel;

// modules
mod storage;
mod frontend;
mod backend;

pub fn run() {
    let conn = storage::diesel::DieselDataProvider::establish("test.db");

    let view = frontend::cli::view::CliView{};

    let ctrl = backend::default::controller::DefaultController{
        conn: &conn, view: &view
    };

    let input = frontend::cli::input::Input{
        ctrl: &ctrl
    };

    input.show();
}
