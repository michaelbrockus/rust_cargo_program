if get_option('with_tests').enabled()
    test_exe = executable('test_prog',
        files('test_cases.rs'),
        rust_args: '--test',
        dependencies : [code_dep])

    test('Execute project tests', test_exe)
endif
