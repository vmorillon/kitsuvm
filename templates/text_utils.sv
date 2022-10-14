{% macro indent_multiline_text(multiline, indent) -%}
{# try to fix missing indent filter for multiline text #}
{# TODO: fix tera #}
{%- set newline = "
" -%}
{%- set_global indent_space = "" -%}
{%- for i in range(end=indent) -%}
  {%- set_global indent_space = indent_space ~ " " -%}
{%- endfor -%}
{{ multiline | replace(from=newline, to=newline~indent_space) }}
{%- endmacro indent_multiline_text %}
