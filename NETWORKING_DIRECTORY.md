```
networking/
└── src/
    ├── lib.rs
    ├── gateway/
    │   ├── mod.rs
    │   ├── gateway_error.rs    
    │   └── clients/
    │       ├── mod.rs
    │       ├── client_error.rs    
    │       └── candle/
    │           ├── mod.rs
    │           ├── candle_error.rs    
    │           └── llama/
    │               ├── mod.rs
    │               ├── config.rs
    │               ├── model.rs
    │               ├── tokenizer.rs
    │               └── generator.rs
    ├── northbound_bus.rs
    ├── southbound_bus.rs
    └── network_error.rs
```