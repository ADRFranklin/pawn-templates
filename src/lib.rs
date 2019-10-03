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
        Templates::load_template_from_file,
        Templates::set_template_global_var_int,
        Templates::set_template_global_var_float,
        Templates::set_template_global_var_string,
        Templates::set_template_var_int,
        Templates::set_template_var_float,
        Templates::set_template_var_string,
        Templates::render_template,
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
            global_vars: liquid::value::Object::new()
        }
    }
);
