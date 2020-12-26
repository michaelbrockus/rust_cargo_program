project('Program', 'rust',
    version: '0.1.0',
    meson_version: '>=0.56.0',
    default_options: ['werror=true', 'rust_std=2018'])

cargo = find_program('cargo', required: true)
cargo_wrapper = find_program('.github' / 'cargo_wrapper.sh')
cargo_tester = find_program('.github' / 'cargo_tester.sh')

subdir('code')
subdir('test')
