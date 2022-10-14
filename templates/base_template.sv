`ifndef {% filter upper %}{{ class.name }}{% endfilter %}_GUARD
`define {% filter upper %}{{ class.name }}{% endfilter %}_GUARD

{% block header -%}
{%- if header -%}
{%- include "header.sv" -%}
{%- endif -%}
{%- endblock header %}

{% block class_def -%}
{%- endblock class_def %}

{% block post_class_def -%}
{%- endblock post_class_def %}

`endif // {% filter upper %}{{ class.name }}{% endfilter %}_GUARD
