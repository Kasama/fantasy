use std::sync::Arc;

use rtdlib::types::*;
use crate::errors::*;
use crate::api::Api;


/// Telegram client event listener
#[derive(Clone, Default)]
pub struct Listener {
  exception: Option<Arc<dyn Fn((&Api, &TGError)) + Send + Sync + 'static>>,
  receive: Option<Arc<dyn Fn((&Api, &String)) -> TGResult<()> + Send + Sync + 'static>>,

{% for name, td_type in listener %}{% set token = find_token(token_name = td_type) %}  {{name | to_snake}}: Option<Arc<dyn Fn((&Api, &{{token.name | to_camel}})) -> TGResult<()> + Send + Sync + 'static>>,
{% endfor %}

{% for token in tokens %}{% if token.blood and token.blood == 'Update' %}  {{token.name | td_remove_prefix(prefix='Update') | to_snake}}: Option<Arc<dyn Fn((&Api, &{{token.name | to_camel}})) -> TGResult<()> + Send + Sync + 'static>>,
{% elif token.is_return_type %}  result_{{token.name | to_snake}}: Option<Arc<dyn Fn((&Api, &{{token.name | to_camel}})) -> TGResult<()> + Send + Sync + 'static>>,
{% endif %}{% endfor %}

}


impl Listener {
  pub fn new() -> Self { Listener::default() }

  pub(crate) fn has_receive_listen(&self) -> bool { self.receive.is_some() }

  pub(crate) fn lout(&self) -> Lout { Lout::new(self.clone()) }


  /// when receive data from tdlib
  pub fn on_receive<F>(&mut self, fnc: F) -> &mut Self where F: Fn((&Api, &String)) -> TGResult<()> + Send + Sync + 'static {
    self.receive = Some(Arc::new(fnc));
    self
  }

  /// when telegram client throw exception
  pub fn on_exception<F>(&mut self, fnc: F) -> &mut Self where F: Fn((&Api, &TGError)) + Send + Sync + 'static {
    self.exception = Some(Arc::new(fnc));
    self
  }

{% for name, td_type in listener %}{% set token = find_token(token_name = td_type) %}
  /// {{token.description}}
  pub fn on_{{name | to_snake}}<F>(&mut self, fnc: F) -> &mut Self where F: Fn((&Api, &{{token.name | to_camel}})) -> TGResult<()> + Send + Sync + 'static {
    self.{{name}} = Some(Arc::new(fnc));
    self
  }
{% endfor %}



{% for token in tokens %}{% if token.blood and token.blood == 'Update' %}
  /// {{token.description}}
  pub fn on_{{token.name | td_remove_prefix(prefix='Update') | to_snake}}<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &{{token.name | to_camel}})) -> TGResult<()> + Send + Sync + 'static {
    self.{{token.name | td_remove_prefix(prefix='Update') | to_snake}} = Some(Arc::new(fnc));
    self
  }
{% elif token.is_return_type %}
  /// {{token.description}}
  pub fn on_result_{{token.name | to_snake}}<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &{{token.name | to_camel}})) -> TGResult<()> + Send + Sync + 'static {
    self.result_{{token.name | to_snake}} = Some(Arc::new(fnc));
    self
  }
{% endif %}{% endfor %}
}


/// Get listener
pub struct Lout {
  listener: Listener,
  supports: Vec<&'static str>
}

impl Lout {
  fn new(listener: Listener) -> Self {
    let supports = vec![
{% for name, td_type in listener %}{% set token = find_token(token_name = td_type) %}      "{{token.name | to_snake | to_camel_lowercase}}",
{% endfor %}

{% for token in tokens %}{% if token.blood and token.blood == 'Update' %}      "{{token.name}}",
{% endif %}{% endfor %}
    ];
    Self { listener, supports }
  }

  pub fn is_support<S: AsRef<str>>(&self, name: S) -> bool {
    self.supports.iter()
      .find(|&&item| item == name.as_ref())
      .is_some()
  }

  /// when telegram client throw exception
  pub fn exception(&self) -> &Option<Arc<dyn Fn((&Api, &TGError)) + Send + Sync + 'static>> {
    &self.listener.exception
  }

  /// when receive data from tdlib
  pub fn receive(&self) -> &Option<Arc<dyn Fn((&Api, &String)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.receive
  }

{% for name, td_type in listener %}{% set token = find_token(token_name = td_type) %}
  /// {{token.description}}
  pub fn {{name | to_snake}}(&self) -> &Option<Arc<dyn Fn((&Api, &{{token.name | to_camel}})) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.{{name | to_snake}}
  }
{% endfor %}


{% for token in tokens %}{% if token.blood and token.blood == 'Update' %}
  /// {{token.description}}
  pub fn {{token.name | td_remove_prefix(prefix='Update') | to_snake}}(&self) -> &Option<Arc<dyn Fn((&Api, &{{token.name | to_camel}})) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.{{token.name | td_remove_prefix(prefix='Update') | to_snake}}
  }
{% elif token.is_return_type %}
  /// {{token.description}}
  pub fn result_{{token.name | to_snake}}(&self) -> &Option<Arc<dyn Fn((&Api, &{{token.name | to_camel}})) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.result_{{token.name | to_snake}}
  }
{% endif %}{% endfor %}
}


