# http_mem

allow for r/w of guest or host memory over HTTP.

simple and easy to write bindings for with the added benefit of an existing API for Rust.


### Memflow Installation

In order to read/write guest memory, the library uses memflow.


This library is tested using kvm as the connector and win32 as the target OS.


To quickly install and set these up for your host OS, its preferred to use memflowup.


It can be installed with `curl --proto '=https' --tlsv1.2 -sSf https://sh.memflow.io | sh`, then you can simply feed it `memflowup install -s -S -d memflow-win32 memflow-kvm`.
