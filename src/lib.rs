extern crate samp;
extern crate liquid;
extern crate log;

mod plugin;

use std::collections::HashMap;
use crate::plugin::Templates;
use samp::initialize_plugin;

initialize_plugin!(
    natives: [
        Templates::create_template,
        Templates::render_template,
        Templates::create_template_var,
    ],
    {
        let samp_logger = samp::plugin::logger()
            .level(log::LevelFilter::Info);

        samp::encoding::set_default_encoding(samp::encoding::WINDOWS_1251);

        let _ = fern::Dispatch::new()
            .format(|callback, message, record| {
                callback.finish(format_args!("[pawn-templates] [{}]: {}", record.level().to_string().to_lowercase(), message))
            })
            .chain(samp_logger)
            .apply();

        Templates {
            pool: HashMap::new(),
            id: 0,
            variables: liquid::value::Object::new()
        }
    }
);