use liquid::value::Object;
use liquid::value::Value;
use log::error;
use samp::native;
use samp::prelude::*;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Read;

pub struct Templates {
    pub pool: HashMap<i32, liquid::Template>,
    pub id: i32,
    pub globals: Object,
}

#[derive(Debug)]
enum ArgumentPairType {
    Invalid = 0,
    String = 1,
    Int = 2,
    Float = 3,
}

impl From<i32> for ArgumentPairType {
    fn from(i: i32) -> ArgumentPairType {
        match i {
            1 => ArgumentPairType::String,
            2 => ArgumentPairType::Int,
            3 => ArgumentPairType::Float,
            _ => ArgumentPairType::Invalid,
        }
    }
}

impl Templates {
    #[native(name = "CreateTemplate")]
    pub fn create_template(&mut self, _: &Amx, template: AmxString) -> AmxResult<i32> {
        let ret = match self.make_template(template.to_string()) {
            Ok(v) => v,
            Err(_) => {
                return Ok(1);
            }
        };

        Ok(ret)
    }

    #[native(raw, name = "RenderTemplate")]
    pub fn render_template(&mut self, _: &Amx, mut params: samp::args::Args) -> AmxResult<i32> {
        let arg_count = params.count() - 3;
        let pairs = if arg_count == 0 || arg_count % 3 == 0 {
            arg_count / 3
        } else {
            error!("invalid variadic argument pattern passed to RenderTemplate.");
            return Ok(1);
        };

        let template_id = match params.next::<i32>() {
            None => {
                error!("invalid template id");
                return Ok(1);
            },
            Some(v) => v,
        };

        let t = match self.pool.get(&template_id) {
            Some(t) => t,
            None => return Ok(2),
        };

        let output_str = match params.next::<UnsizedBuffer>() {
            None => {
                error!("invalid buffer");
                return Ok(1);
            },
            Some(v) => v,
        };

        let output_len = match params.next::<usize>() {
            None => {
                error!("invalid output len");
                return Ok(1);
            },
            Some(v) => v,
        };

        let mut variables = self.globals.clone();

        for _ in 0..pairs {
            let var_type = match params.next::<Ref<i32>>() {
                None => {
                    error!("invalid type expected int");
                    return Ok(-1);
                }
                Some(t) => t,
            };

            let key = match params.next::<AmxString>() {
                None => {
                    error!("invalid type expected string");
                    return Ok(-1);
                }
                Some(k) => k.to_string(),
            };

            match ArgumentPairType::from(*var_type) {
                ArgumentPairType::String => {
                    let value = params.next::<AmxString>().unwrap().to_string();
                    variables.insert(key.into(), liquid::value::Value::scalar(value));
                }
                ArgumentPairType::Int => {
                    let value = params.next::<Ref<i32>>().unwrap();
                    variables.insert(key.into(), liquid::value::Value::scalar(*value));
                }
                ArgumentPairType::Float => {
                    let value = params.next::<Ref<f32>>().unwrap();
                    variables.insert(key.into(), liquid::value::Value::scalar(f64::from(*value)));
                }
                _ => return Ok(3),
            }
        }

        let output = match t.render(&variables) {
            Ok(v) => v,
            Err(e) => {
                error!("{}", e);
                return Ok(4);
            }
        };

        let mut dest = output_str.into_sized_buffer(output_len);
        let _ = samp::cell::string::put_in_buffer(&mut dest, &output);

        Ok(0)
    }

    #[native(name = "MakeTemplateVarInt")]
    pub fn make_template_var_int(
        &mut self,
        _: &Amx,
        namespace: AmxString,
        key: AmxString,
        value: i32,
    ) -> AmxResult<i32> {
        let namespace_str = namespace.to_string();
        if namespace_str.is_empty() {
            error!("namespace expects a string, none given.");
            return Ok(1);
        }

        let key_str = key.to_string();
        if key_str.is_empty() {
            error!("key expects a string, none given.");
            return Ok(1);
        }

        let mut variable: Object = self.get_global_variables(&namespace_str);
        variable.insert(key_str.into(), Value::scalar(value));
        self.globals
            .insert(namespace_str.into(), Value::Object(variable));

        Ok(0)
    }

    #[native(name = "MakeTemplateVarFloat")]
    pub fn make_template_var_float(
        &mut self,
        _: &Amx,
        namespace: AmxString,
        key: AmxString,
        value: f32,
    ) -> AmxResult<i32> {
        let namespace_str = namespace.to_string();
        if namespace_str.is_empty() {
            error!("namespace expects a string, none given.");
            return Ok(1);
        }

        let key_str = key.to_string();
        if key_str.is_empty() {
            error!("key expects a string, none given.");
            return Ok(1);
        }

        let mut variable: Object = self.get_global_variables(&namespace_str);
        variable.insert(key_str.into(), Value::scalar(f64::from(value)));
        self.globals
            .insert(namespace_str.into(), Value::Object(variable));

        Ok(0)
    }

    #[native(name = "MakeTemplateVarString")]
    pub fn make_template_var_string(
        &mut self,
        _: &Amx,
        namespace: AmxString,
        key: AmxString,
        value: AmxString,
    ) -> AmxResult<i32> {
        let namespace_str = namespace.to_string();
        if namespace_str.is_empty() {
            error!("namespace expects a string, none given.");
            return Ok(1);
        }

        let key_str = key.to_string();
        if key_str.is_empty() {
            error!("key expects a string, none given.");
            return Ok(1);
        }

        let mut variable: Object = self.get_global_variables(&namespace_str);
        variable.insert(key_str.into(), Value::scalar(value.to_string()));
        self.globals
            .insert(namespace_str.into(), Value::Object(variable));

        Ok(0)
    }

    #[native(name = "LoadTemplateFromFile")]
    pub fn load_template_from_file(&mut self, _: &Amx, path: AmxString) -> AmxResult<i32> {
        let path_str = path.to_string();
        if path_str.is_empty() {
            error!("path expected a string, none given.");
            return Ok(1);
        }
        let current_path = &path_str;
        let mut file = match File::open(current_path) {
            Ok(file) => file,
            Err(_) => {
                error!(
                    "the file could not be found at the path: {}",
                    path_str.clone()
                );
                return Ok(1);
            }
        };

        let mut file_contents = String::new();
        match file.read_to_string(&mut file_contents) {
            Err(e) => {
                error!("{}", e);
                return Ok(1);
            }
            Ok(v) => v,
        };

        let ret = match self.make_template(file_contents) {
            Ok(v) => v,
            Err(_) => return Ok(1),
        };

        Ok(ret)
    }

    fn alloc(&mut self, template: liquid::Template) -> i32 {
        self.id += 1;
        self.pool.insert(self.id, template);
        self.id
    }

    fn get_global_variables(&mut self, namespace: &str) -> Object {
        let mut object = Object::new();
        for (key, value) in self.globals.iter() {
            if key == namespace {
                object = value.as_object().unwrap().clone();
                break;
            }
        }

        object
    }

    fn make_template(&mut self, template: String) -> Result<i32, Box<dyn Error>> {
        let parser = liquid::ParserBuilder::with_liquid().build().unwrap();

        let t = match parser.parse(&template) {
            Ok(v) => v,
            Err(e) => {
                error!("{}", e);
                return Ok(1);
            }
        };

        let id = self.alloc(t);
        Ok(id)
    }
}

impl Default for Templates {
    fn default() -> Self {
        Templates {
            pool: HashMap::new(),
            id: 0,
            globals: liquid::value::Object::new(),
        }
    }
}

impl SampPlugin for Templates {}
