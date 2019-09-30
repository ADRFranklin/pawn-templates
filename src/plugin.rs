use liquid::value::Object;
use liquid::value::Value;
use log::error;
use samp::native;
use samp::prelude::*;
use std::collections::HashMap;

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

impl ArgumentPairType {
    fn from_i32(i: i32) -> ArgumentPairType {
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
        let parser = liquid::ParserBuilder::with_liquid().build().unwrap();

        let t = match parser.parse(&template.to_string()) {
            Ok(v) => v,
            Err(e) => {
                error!("{}", e);
                return Ok(1);
            }
        };

        let id = self.alloc(t);
        return Ok(id);
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

        let template_id = params.next::<i32>().unwrap();
        let t = match self.pool.get(&template_id) {
            Some(t) => t,
            None => return Ok(2),
        };

        let output_str = params.next::<UnsizedBuffer>().unwrap();
        let output_len = params.next::<usize>().unwrap();

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

            match ArgumentPairType::from_i32(*var_type) {
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
                    variables.insert(key.into(), liquid::value::Value::scalar(*value as f64));
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

        return Ok(0);
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
        }

        let mut variable: Object = Object::new();
        for (key, value) in self.globals.iter() {
            if key.to_string() == namespace_str {
                variable = value.as_object().unwrap().clone();
                break;
            }
        }

        variable.insert(key_str.into(), Value::scalar(value));
        self.globals
            .insert(namespace_str.into(), Value::Object(variable));

        return Ok(0);
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
        }

        let mut variable: Object = Object::new();
        for (key, value) in self.globals.iter() {
            if key.to_string() == namespace_str {
                variable = value.as_object().unwrap().clone();
                break;
            }
        }

        variable.insert(key_str.into(), Value::scalar(value as f64));
        self.globals
            .insert(namespace_str.into(), Value::Object(variable));

        return Ok(0);
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
        }

        let mut variable: Object = Object::new();
        for (key, value) in self.globals.iter() {
            if key.to_string() == namespace_str {
                variable = value.as_object().unwrap().clone();
                break;
            }
        }

        variable.insert(key_str.into(), Value::scalar(value.to_string()));
        self.globals
            .insert(namespace_str.into(), Value::Object(variable));

        return Ok(0);
    }

    fn alloc(&mut self, template: liquid::Template) -> i32 {
        self.id += 1;
        self.pool.insert(self.id, template);
        self.id
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
