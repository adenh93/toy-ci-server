use build_agent::config::{Cache, LogLevel, PipelineConfig, Settings, Step, StepFormat};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

fn get_config_fixture<P: Into<PathBuf>>(path: P) -> PipelineConfig {
    let file = File::open(path.into()).unwrap();
    let mut reader = BufReader::new(file);
    serde_yaml::from_reader(&mut reader).unwrap()
}

fn do_assertions(input: PipelineConfig, expected: PipelineConfig) {
    assert_eq!(input.title, expected.title);
    assert_eq!(input.environment, expected.environment);
    assert_eq!(input.caches, expected.caches);
    assert_eq!(input.settings, expected.settings);
    assert_eq!(input.steps, expected.steps);
}

#[test]
fn parses_config() {
    let config = get_config_fixture("tests/fixtures/workflow-a.yml");

    let expected = PipelineConfig {
        title: "Workflow A".to_owned(),
        environment: Some(HashMap::from([
            ("FIRST_VAR".to_owned(), "first_var_val".to_owned()),
            ("SECOND_VAR".to_owned(), "second_var_val".to_owned()),
        ])),
        caches: Some(vec![Cache {
            name: Some("Cache node_modules".to_owned()),
            path: "node_modules/".into(),
            key: "yarn.lock".to_owned(),
        }]),
        settings: Settings {
            logging: LogLevel::Verbose,
        },
        steps: vec![
            StepFormat::Full(Step {
                name: "Install dependencies".to_owned(),
                run: "yarn install:ci".to_owned(),
            }),
            StepFormat::Minimal("npm install -g esbuild".to_owned()),
            StepFormat::Full(Step {
                name: "Build".to_owned(),
                run: "yarn build".to_owned(),
            }),
            StepFormat::Full(Step {
                name: "Test".to_owned(),
                run: "yarn test:ci".to_owned(),
            }),
            StepFormat::Full(Step {
                name: "Deploy".to_owned(),
                run: "yarn deploy \\\nyarn post:deploy\n".to_owned(),
            }),
        ],
    };

    do_assertions(config, expected);
}

#[test]
fn parses_default_settings() {
    let config = get_config_fixture("tests/fixtures/workflow-b.yml");

    let expected = PipelineConfig {
        title: "Workflow B".to_owned(),
        environment: None,
        caches: None,
        settings: Settings {
            logging: LogLevel::default(),
        },
        steps: vec![
            StepFormat::Full(Step {
                name: "Install dependencies".to_owned(),
                run: "yarn install:ci".to_owned(),
            }),
            StepFormat::Full(Step {
                name: "Build".to_owned(),
                run: "yarn build".to_owned(),
            }),
            StepFormat::Full(Step {
                name: "Test".to_owned(),
                run: "yarn test:ci".to_owned(),
            }),
            StepFormat::Full(Step {
                name: "Deploy".to_owned(),
                run: "yarn deploy".to_owned(),
            }),
        ],
    };

    do_assertions(config, expected);
}
