use nu_plugin::{serve_plugin, MsgPackSerializer};
use nu_plugin_bio::BioPlugin;

fn main() {
    serve_plugin(&BioPlugin, MsgPackSerializer {})
}
