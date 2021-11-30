use crate::execute_demos;
use crate::DemoItem;

pub fn new() -> DemoItem {
    DemoItem {
        title: "sice demos",
        execute: execute,
        enable: true,
    }
}

fn execute() {
    let demos = vec![DemoItem {
        title: "slice_basic_usage",
        execute: slice_basic_usage,
        enable: true,
    }];
    execute_demos(&demos);
}

fn slice_basic_usage() {}
