{% import "generic_function.sv" as gen_funct %}
{% extends "base_template.sv" %}

{% block class_def %}
class {{ class.name }} extends uvm_test;
  `uvm_component_utils({{ class.name }});

  {% for member in class.members -%}
  {{ member.name }};
  {% endfor -%}

  {%- for function in class.functions -%}
  {%- if not function.is_declared_internally -%}
  {{- gen_funct::generate_extern_function_signature(funct = function) -}}
  {%- endif -%}
  {%- endfor -%}

  {%- for function in class.functions -%}
  {%- if function.is_declared_internally -%}
  {{- gen_funct::generate_function(funct = function) -}}
  {%- endif -%}
  {%- endfor -%}
endclass: {{ class.name }}
{%- endblock class_def %}

{% block post_class_def %}
{%- for function in class.functions -%}
{% if not function.is_declared_internally -%}
{{ gen_funct::generate_extern_function(funct = function, class_name = class.name) }}
{%- endif %}
{%- endfor -%}
{% endblock post_class_def %}
