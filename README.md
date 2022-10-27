# KitsUVM

Random poc trying to rebuild a better easier uvm.

Focuses on improving generated files by checking DUT ports and matching interface ports.

Relies on [tera](https://github.com/Keats/tera) (template engine), [toml-rs](https://github.com/toml-rs/toml) (TOML parser) and [sv-parser](https://github.com/dalance/sv-parser) (SystemVerilog parser).

```mermaid
flowchart LR
  CLI[CLI] -->|config files paths| TomlParser[TOML Parser]
  ConfigFiles --o TomlParser
  subgraph ConfigFiles[Config Files]
    direction LR
    Common[./common.toml]
    PinList[./pinlist.toml]
    ITemplate[[./interface_tpl.toml]]
  end

  TomlParser -->|generates| UVM[KitsUVM]
  TomlParser -->|DUT path| SVParser[SV Parser]

  DUT --o SVParser
  subgraph DUT[DUT File]
    DUTFile[./dut.sv]
  end
  SVParser -->|DUT/interfaces compat| UVM

  UVM -->|drives| Tera[Template engine]
  TemplateFiles --o Tera
  subgraph TemplateFiles[Template Files]
    TemplateDir[[./templates/*.sv.j2]]
  end

  Tera -->|creates| Project[UVM Project]
```

# Install

[rust & cargo setup](https://www.rust-lang.org/learn/get-started)
