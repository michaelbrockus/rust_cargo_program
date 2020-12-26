sources = files(
    meson.source_root() / 'Cargo.toml',
    'program.rs',
    'main.rs')

cargo_release = custom_target(
    'cargo-build',
    build_by_default: true,
    input: sources,
    output: 'prog',
    console: true,
    install: true,
    install_dir: get_option('bindir'),
    command: [
        cargo_wrapper,
        meson.build_root(),
        meson.source_root(),
        '@OUTPUT@',
        get_option('buildtype'),
        'prog'
  ]
)
