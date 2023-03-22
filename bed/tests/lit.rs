use lit::{event_handler::Default, Config};
use std::env::consts::EXE_SUFFIX;

const NAME: &str = "bed";

#[test]
fn lit() {
    lit::run::tests(Default::default(), |config: &mut Config| {
        config.add_search_path("tests");
        config.add_extension(NAME);
        config.constants.insert(NAME.to_string(), exe_path());
    })
    .expect("failure");
}

fn exe_path() -> String {
    std::env::current_exe()
        .ok()
        .map(|mut path| {
            path.pop();
            path.pop();
            path
        })
        .unwrap()
        .join(format!("{NAME}{EXE_SUFFIX}"))
        .into_os_string()
        .into_string()
        .unwrap()
}
