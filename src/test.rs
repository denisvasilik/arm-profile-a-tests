use arm_semihosting::{hprintln};

pub const TESTS_MAX: usize = 16;

#[derive(Copy, Clone)]
pub struct TestCase<'a> {
    pub name: &'a str,
    pub run: Option<fn() -> Result<i32, ()>>,
}

impl<'a> TestCase<'a> {
    pub fn new(name: &'a str, test_fn: fn() -> Result<i32, ()>) -> TestCase {
        TestCase {
            name: name,
            run: Some(test_fn),
        }
    }
}

pub struct TestRunner<'a> {
    pub tests: [Option<TestCase<'a>>; TESTS_MAX],
}

impl<'a> TestRunner<'a> {
    pub fn new() -> TestRunner<'a> {
        TestRunner {
            tests : [None; TESTS_MAX],
        }
    }
}

pub struct TestApp<'a> {
    pub runner: Option<TestRunner<'a>>,
}

impl<'a> TestApp<'a> {
    pub fn new() -> TestApp<'a> {
        TestApp {
            runner: None,
        }
    }

    pub fn run(&self) {
        if let Some(runner) = &self.runner {
            for test_case_tuple in runner.tests.iter().enumerate() {
                if let (index, Some(test_case)) = test_case_tuple {
                    if let Some(test_fn) = test_case.run {
                        let result = test_fn();
                        self.print_result(index, test_case.name, result);
                    }
                }
            }
        }
    }

    pub fn print_result(&self, index: usize, test_name: &str, result: Result<i32, ()>) {
        let result_str : &str;
        if result.is_err() {
            result_str = "failed";
        } else {
            result_str = "success";
        }
        hprintln!("{}. {} [{}]", index, test_name, result_str);
    }
}
