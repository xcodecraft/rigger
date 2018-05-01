
# code structure

``` shell     
├── blueprint.md
├── Cargo.toml
├── core
│   ├── Cargo.toml
│   └── src
│       ├── cmd.rs
│       ├── cmds
│       │   ├── mod.rs
│       │   └── run.rs
│       ├── creator.rs  # res,cmd  creator ;
│       ├── def.rs
│       ├── err.rs    # err define ;
│       ├── inner
│       │   ├── env.rs
│       │   ├── mod.rs
│       │   ├── prj.rs
│       │   ├── proxy.rs
│       │   ├── rgm.rs
│       │   ├── sys.rs
│       │   ├── utls.rs
│       │   └── var.rs
│       ├── lib.rs
│       ├── model.rs　　#core abstract　for rigger
│       ├── parser.rs
│       └── res.rs    # res abstract ;
├── docs
├── extends　　# extendtion for resource
│   ├── Cargo.toml
│   ├── meterial
│   │   └── run_tpl.yaml
│   └── src
│       ├── lib.rs
│       ├── service
│       │   ├── beanstalk.rs
│       │   ├── crontab.rs
│       │   ├── mod.rs
│       │   ├── mysql.rs
│       │   ├── nginx.rs
│       │   ├── php_fpm.rs
│       │   └── varnishd.rs
│       └── unix
│           ├── copy.rs
│           ├── link.rs
│           ├── mod.rs
│           └── path.rs
├── lib
│   ├── Cargo.toml
│   ├── material
│   │   ├── dev.yaml
│   │   ├── run_mid.yaml
│   │   └── run.yaml
│   └── src
│       ├── def.rs
│       ├── eexp.rs  #env express parser
│       ├── lib.rs
│       ├── macros.rs
│       └── yaml.rs
├── LICENSE
├── package.json
├── README.md
└── rg
    ├── Cargo.toml
    └── src
        └── main.rs        
        
```
