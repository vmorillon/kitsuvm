{% import "generic_variable.sv" as gen_var %}
{% import "text_utils.sv" as text_utils %}

{% macro generate_function(funct) %}
  function {{ funct.name }}(
    {{-
      gen_var::generate_variable_inline_list(
        vars = funct.parameters)
    -}}
  );
  {{-
    text_utils::indent_multiline_text(
      multiline=funct.body,
      indent=4)
  }}
  endfunction: {{ funct.name }}
{% endmacro generate_function %}

{% macro generate_extern_function(funct, class_name) %}
function {{ class_name }}::{{ funct.name }}(
    {{-
      gen_var::generate_variable_inline_list(
        vars = funct.parameters)
    -}}
  );
  {{-
    text_utils::indent_multiline_text(
      multiline=funct.body,
      indent=2)
  }}
endfunction: {{ funct.name }}
{% endmacro generate_function %}

{% macro generate_extern_function_signature(funct) %}
  extern function {{ funct.name }}(
    {{- gen_var::generate_variable_inline_list(
      vars = funct.parameters)
    -}}
  );
{% endmacro generate_function %}
