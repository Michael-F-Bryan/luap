fn main() {
    let args = libtest_mimic::Arguments::from_args();
    let tests = integration_tests::discover();
    libtest_mimic::run(&args, tests).exit();
}
