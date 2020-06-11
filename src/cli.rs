use clap::{App, AppSettings, Arg};

pub fn build_cli() -> App<'static, 'static> {
    App::new("myapp")
        .version("1.0")
        .author("telumo <drumscohika@gmail.com>")
        .about("Sample Network Problems")
        .setting(AppSettings::DeriveDisplayOrder)
        .arg(
            Arg::from_usage("-i --interface <INTERFACE> 'target interface name'")
        )
}
