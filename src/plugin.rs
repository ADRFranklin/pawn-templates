use std::collections::HashMap;
use samp::prelude::*;
use samp::{native};
use log::{error};


pub struct Templates {
    pub pool: HashMap<i32, liquid::Template>,
    pub id: i32,
    pub variables: liquid::value::Object
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
    pub fn render_template(
        &mut self, 
        _: &Amx, 
        mut params: samp::args::Args
    ) -> AmxResult<i32> {
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

        let mut variables = self.variables.clone();

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

    #[native(raw, name = "CreateTemplateVar")]
    pub fn create_template_var(
        &mut self,
        _: &Amx,
        mut params: samp::args::Args
    ) -> AmxResult<i32> {
        let arg_count = params.count();
        let pairs = if arg_count == 0 || arg_count % 3 == 0 {
            arg_count / 3
        } else {
            error!("invalid variadic argument pattern passed to CreateTemplateVar.");
            return Ok(1);
        };

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
                },
                Some(k) => k.to_string(),
            };

            match ArgumentPairType::from_i32(*var_type) {
                ArgumentPairType::String => {
                    let value = params.next::<AmxString>().unwrap().to_string();
                    self.variables.insert(key.into(), liquid::value::Value::scalar(value));
                }
                ArgumentPairType::Int => {
                    let value = params.next::<Ref<i32>>().unwrap();
                    self.variables.insert(key.into(), liquid::value::Value::scalar(*value));
                }
                ArgumentPairType::Float => {
                    let value = params.next::<Ref<f32>>().unwrap();
                    self.variables.insert(key.into(), liquid::value::Value::scalar(*value as f64));
                }
                _ => return Ok(3),         
            };
        }

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
            variables: liquid::value::Object::new()
        }
    }
}

impl SampPlugin for Templates {}