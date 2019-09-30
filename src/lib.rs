extern crate liquid;
extern crate log;
extern crate samp;

mod plugin;

use crate::plugin::*;
use samp::initialize_plugin;
use std::collections::HashMap;

initialize_plugin!(
    natives: [
        Templates::create_template,
        Templates::render_template,
        Templates::make_template_var_int,
        Templates::make_template_var_float,
        Templates::make_template_var_string,
        Templates::load_template_from_file,
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
            globals: liquid::value::Object::new()
        }
    }
);
