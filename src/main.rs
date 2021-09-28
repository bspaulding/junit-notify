use hotwatch::{Event, Hotwatch};
use notify_rust::Notification;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use xml::reader::{EventReader, XmlEvent};

#[derive(Debug)]
struct TestSuite {
    name: String,
    tests: u32,
    failures: u32,
    errors: u32,
}

fn icon(failures: u32, errors: u32) -> &'static str {
    let fail_icon = "❌";
    let success_icon = "✅";
    let error_icon = "⚠️";

    if failures > 0 {
        fail_icon
    } else if errors > 0 {
        error_icon
    } else {
        success_icon
    }
}

fn test_report_message(suites: Vec<TestSuite>) -> String {
    let tests: u32 = suites.iter().map(|s| s.tests).sum();
    let failures: u32 = suites.iter().map(|s| s.failures).sum();
    let errors: u32 = suites.iter().map(|s| s.errors).sum();

    format!(
        "{} {} tests, {} failures, {} errors",
        icon(failures, errors), tests, failures, errors
    )
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let dir = &args[1];

    let mut hotwatch = Hotwatch::new().expect("Failed to initialize watcher!");
    hotwatch
        .watch(dir, |event: Event| {
            if let Event::Write(path) = event {
                match read_test_suites_from_report(&path) {
                    Ok(suites) => {
                        notify_suites(suites);
                    }
                    Err(error) => {
                        println!("Error parsing suites {:?}", error);
                    }
                }
            }
        })
        .expect("Failed to initialize watcher for directory");
    println!("Watching for junit files in {}...", dir);

    loop {}
}

fn notify_suites(suites: Vec<TestSuite>) {
    if suites.len() == 0 {
        return;
    }

    Notification::new()
        .summary("Test Report")
        .body(&test_report_message(suites))
        .show()
        .expect("oops");
}

fn get_attribute_value(attributes: &Vec<xml::attribute::OwnedAttribute>, attr_name: &str, default_value: &str) -> String {
    attributes
        .iter()
        .find(|a| a.name.local_name == attr_name)
        .map(|a| a.value.clone())
        .unwrap_or(String::from(default_value))
}

fn read_test_suites_from_report(path: &PathBuf) -> Result<Vec<TestSuite>, std::io::Error> {
    let file = File::open(path).unwrap();
    let file = BufReader::new(file);
    let parser = EventReader::new(file);
    let mut suites = vec![];
    for event in parser {
        match event {
            Ok(XmlEvent::StartElement {
                name, attributes, ..
            }) => {
                if name.local_name == "testsuite" {
                    suites.push(TestSuite {
                        name: get_attribute_value(&attributes, "name", "Untitled"),
                        tests: get_attribute_value(&attributes, "tests", "0").parse().unwrap_or(0),
                        failures: get_attribute_value(&attributes, "failures", "0").parse().unwrap_or(0),
                        errors: get_attribute_value(&attributes, "errors", "0").parse().unwrap_or(0),
                    });
                }
            }
            Err(e) => {
                println!("Parsing Error: {}", e);
                break;
            }
            _ => {}
        }
    }

    Ok(suites)
}
