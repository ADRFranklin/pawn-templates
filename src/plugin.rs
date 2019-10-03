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
    pub pool: HashMap<i32, Template>,
    pub id: i32,
    pub global_vars: Object,
}

pub struct Template {
    pub template: liquid::Template,
    pub variables: Object,
}

#[derive(Debug, Clone, Copy)]
pub enum ReturnType {
    Success,
    Error,
    Value(i32),
}

impl AmxCell<'_> for ReturnType {
    fn as_cell(&self) -> i32 {
        match self {
            ReturnType::Success => 0,
            ReturnType::Error => 1,
            ReturnType::Value(result) => *result,
        }
    }
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
    #[native(name = "Template_Create")]
    pub fn create_template(&mut self, _: &Amx, template: AmxString) -> AmxResult<ReturnType> {
        let ret = match self.make_template(template.to_string()) {
            Ok(v) => v,
            Err(_) => {
                return Ok(ReturnType::Error);
            }
        };

        Ok(ReturnType::Value(ret))
    }

    #[native(name = "Template_LoadFromFile")]
    pub fn load_template_from_file(&mut self, _: &Amx, path: AmxString) -> AmxResult<ReturnType> {
        let path_str = path.to_string();
        if path_str.is_empty() {
            error!("path expected a string, none given.");
            return Ok(ReturnType::Error);
        }
        let current_path = &path_str;
        let mut file = match File::open(current_path) {
            Ok(file) => file,
            Err(_) => {
                error!(
                    "the file could not be found at the path: {}",
                    path_str.clone()
                );
                return Ok(ReturnType::Error);
            }
        };

        let mut file_contents = String::new();
        match file.read_to_string(&mut file_contents) {
            Err(e) => {
                error!("{}", e);
                return Ok(ReturnType::Error);
            }
            Ok(v) => v,
        };

        let ret = match self.make_template(file_contents) {
            Ok(v) => v,
            Err(_) => return Ok(ReturnType::Error),
        };

        Ok(ReturnType::Value(ret))
    }

    #[native(name = "Template_SetGlobalInt")]
    pub fn set_template_global_var_int(
        &mut self,
        _: &Amx,
        namespace: AmxString,
        key: AmxString,
        value: i32,
    ) -> AmxResult<ReturnType> {
        let namespace_str = namespace.to_string();
        if namespace_str.is_empty() {
            error!("namespace expects a string, none given.");
            return Ok(ReturnType::Error);
        }

        let key_str = key.to_string();
        if key_str.is_empty() {
            error!("key expects a string, none given.");
            return Ok(ReturnType::Error);
        }

        let mut variable: Object = self.get_global_variables(&namespace_str);
        variable.insert(key_str.into(), Value::scalar(value));
        self.global_vars
            .insert(namespace_str.into(), Value::Object(variable));

        Ok(ReturnType::Success)
    }

    #[native(name = "Template_SetGlobalFloat")]
    pub fn set_template_global_var_float(
        &mut self,
        _: &Amx,
        namespace: AmxString,
        key: AmxString,
        value: f32,
    ) -> AmxResult<ReturnType> {
        let namespace_str = namespace.to_string();
        if namespace_str.is_empty() {
            error!("namespace expects a string, none given.");
            return Ok(ReturnType::Error);
        }

        let key_str = key.to_string();
        if key_str.is_empty() {
            error!("key expects a string, none given.");
            return Ok(ReturnType::Error);
        }

        let mut variable: Object = self.get_global_variables(&namespace_str);
        variable.insert(key_str.into(), Value::scalar(f64::from(value)));
        self.global_vars
            .insert(namespace_str.into(), Value::Object(variable));

        Ok(ReturnType::Success)
    }

    #[native(name = "Template_SetGlobalString")]
    pub fn set_template_global_var_string(
        &mut self,
        _: &Amx,
        namespace: AmxString,
        key: AmxString,
        value: AmxString,
    ) -> AmxResult<ReturnType> {
        let namespace_str = namespace.to_string();
        if namespace_str.is_empty() {
            error!("namespace expects a string, none given.");
            return Ok(ReturnType::Error);
        }

        let key_str = key.to_string();
        if key_str.is_empty() {
            error!("key expects a string, none given.");
            return Ok(ReturnType::Error);
        }

        let mut variable: Object = self.get_global_variables(&namespace_str);
        variable.insert(key_str.into(), Value::scalar(value.to_string()));
        self.global_vars
            .insert(namespace_str.into(), Value::Object(variable));

        Ok(ReturnType::Success)
    }

    #[native(name = "Template_SetInt")]
    pub fn set_template_var_int(
        &mut self,
        _: &Amx,
        template_id: i32,
        key: AmxString,
        value: i32,
    ) -> AmxResult<ReturnType> {
        let t = match self.pool.get_mut(&template_id) {
            Some(t) => t,
            None => {
                error!("invalid template id passed.");
                return Ok(ReturnType::Error);
            }
        };

        if key.is_empty() {
            error!("expected string, none given.");
            return Ok(ReturnType::Error);
        }

        t.variables
            .insert(key.to_string().into(), liquid::value::Value::scalar(value));

        Ok(ReturnType::Success)
    }

    #[native(name = "Template_SetFloat")]
    pub fn set_template_var_float(
        &mut self,
        _: &Amx,
        template_id: i32,
        key: AmxString,
        value: f32,
    ) -> AmxResult<ReturnType> {
        let t = match self.pool.get_mut(&template_id) {
            Some(t) => t,
            None => {
                error!("invalid template id passed.");
                return Ok(ReturnType::Error);
            }
        };

        if key.is_empty() {
            error!("expected string, none given.");
            return Ok(ReturnType::Error);
        }

        t.variables.insert(
            key.to_string().into(),
            liquid::value::Value::scalar(f64::from(value)),
        );

        Ok(ReturnType::Success)
    }

    #[native(name = "Template_SetString")]
    pub fn set_template_var_string(
        &mut self,
        _: &Amx,
        template_id: i32,
        key: AmxString,
        value: AmxString,
    ) -> AmxResult<ReturnType> {
        let t = match self.pool.get_mut(&template_id) {
            Some(t) => t,
            None => {
                error!("invalid template id passed.");
                return Ok(ReturnType::Error);
            }
        };

        if key.is_empty() {
            error!("expected string for key, none given.");
            return Ok(ReturnType::Error);
        }

        t.variables.insert(
            key.to_string().into(),
            liquid::value::Value::scalar(value.to_string()),
        );

        Ok(ReturnType::Success)
    }

    #[native(raw, name = "Template_Render")]
    pub fn render_template(
        &mut self,
        _: &Amx,
        mut params: samp::args::Args,
    ) -> AmxResult<ReturnType> {
        let arg_count = params.count() - 3;
        let pairs = if arg_count == 0 || arg_count % 3 == 0 {
            arg_count / 3
        } else {
            error!("invalid variadic argument pattern passed to RenderTemplate.");
            return Ok(ReturnType::Error);
        };

        let template_id = match params.next::<i32>() {
            None => {
                error!("invalid template id");
                return Ok(ReturnType::Error);
            }
            Some(v) => v,
        };

        let t = match self.pool.get_mut(&template_id) {
            Some(t) => t,
            None => return Ok(ReturnType::Error),
        };

        let output_str = match params.next::<UnsizedBuffer>() {
            None => {
                error!("invalid buffer");
                return Ok(ReturnType::Error);
            }
            Some(v) => v,
        };

        let output_len = match params.next::<usize>() {
            None => {
                error!("invalid output len");
                return Ok(ReturnType::Error);
            }
            Some(v) => v,
        };

        let mut object = self.global_vars.clone();
        object.extend(t.variables.clone());

        for _ in 0..pairs {
            let var_type = match params.next::<Ref<i32>>() {
                None => {
                    error!("invalid type expected int");
                    return Ok(ReturnType::Error);
                }
                Some(t) => t,
            };

            let key = match params.next::<AmxString>() {
                None => {
                    error!("invalid type expected string");
                    return Ok(ReturnType::Error);
                }
                Some(k) => k.to_string(),
            };

            match ArgumentPairType::from(*var_type) {
                ArgumentPairType::String => {
                    let value = params.next::<AmxString>().unwrap().to_string();
                    object.insert(key.into(), liquid::value::Value::scalar(value));
                }
                ArgumentPairType::Int => {
                    let value = params.next::<Ref<i32>>().unwrap();
                    object.insert(key.into(), liquid::value::Value::scalar(*value));
                }
                ArgumentPairType::Float => {
                    let value = params.next::<Ref<f32>>().unwrap();
                    object.insert(key.into(), liquid::value::Value::scalar(f64::from(*value)));
                }
                _ => return Ok(ReturnType::Error),
            }
        }

        let output = match t.template.render(&object) {
            Ok(v) => v,
            Err(e) => {
                error!("{}", e);
                return Ok(ReturnType::Error);
            }
        };

        let mut dest = output_str.into_sized_buffer(output_len);
        let _ = samp::cell::string::put_in_buffer(&mut dest, &output);

        Ok(ReturnType::Success)
    }

    fn alloc(&mut self, template: Template) -> i32 {
        self.id += 1;
        self.pool.insert(self.id, template);
        self.id
    }

    fn get_global_variables(&mut self, namespace: &str) -> Object {
        let mut object = Object::new();
        for (key, value) in self.global_vars.iter() {
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

        let temp = Template {
            template: t,
            variables: Object::new(),
        };

        let id = self.alloc(temp);
        Ok(id)
    }
}

impl Default for Templates {
    fn default() -> Self {
        Templates {
            pool: HashMap::new(),
            id: 0,
            global_vars: liquid::value::Object::new(),
        }
    }
}

impl SampPlugin for Templates {}
