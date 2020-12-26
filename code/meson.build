cargo_toml = files(
    '..' / 'Cargo.toml',
    '..' / 'Cargo.lock'
)

code_lib = static_library('code_lib', files('program.rs'),
    include_directories: '.',
    install: true)

code_dep = declare_dependency(
    link_with: code_lib,
    include_directories: '.')

executable('prog', files('main.rs'), dependencies: code_dep, install: true)
