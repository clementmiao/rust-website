FROM jimmycuadra/rust
EXPOSE 8080
COPY Cargo.toml /source
COPY src/main.rs /source/src/
COPY assets/template.tpl /source/assets/
COPY assets/css/main.css /source/assets/css/
CMD cargo run
