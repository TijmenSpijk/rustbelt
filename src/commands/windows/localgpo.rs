//! This module defines the `ExampleCommand` which is an example implementation of a command
//! using the `Command` trait. It demonstrates how to register a command and implement its
//! execution logic.

use std::collections::HashMap;

use clap::Command as ClapCommand;
use windows::{core::*, Win32::System::Variant::VARIANT};

use crate::{
    commands::base::{registry::CommandRegistration, Command, CommandDTO, CommandData, CommandResult::{self, Simple}},
    runtime::Runtime,
    utils::registry::{get_string_value, get_sub_key_names, open_sub_key, RegistryHive},
};

enum GPOLink {
    NO_LINK_INFORMATION = 0,
    LOCAL_MACHINE = 1,
    SITE = 2,
    DOMAIN = 3,
    ORGANIZATIONAL_UNIT = 4
}

enum GPOOptions {
    ALL_SECTIONS_ENABLED = 0,
    USER_SECTION_DISABLED = 1,
    COMPUTER_SECTION_DISABLE = 2,
    ALL_SECTIONS_DISABLED = 3
}

pub struct LocalGPOCommand {
    data: CommandData,
}

inventory::submit! {
    CommandRegistration {
        name: "localgpo",
        factory: || Box::new(LocalGPOCommand::default()),
        clap_command: || ClapCommand
            ::new("localgpo")
            .version("0.1")
            .about("Local Group Policy (GPO) settings applied to the local machine/user.")
    }
}

impl Command for LocalGPOCommand {
    /// Executes the `ExampleCommand`.
    ///
    /// # Arguments
    ///
    /// * `runtime` - A reference to the `Runtime` instance.
    /// * `_` - A slice of strings representing the command arguments.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `CommandResult`.
    fn execute(&self, _: &Runtime, _: &[String]) -> Result<CommandResult> {
        // Local machine GPOs
        let machine_ids = get_sub_key_names(
            RegistryHive::LocalMachine,
            "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Group Policy\\DataStore\\Machine\\0"
        )?;

        println!("{:?}", machine_ids);

        let mut results: Vec<String> = vec![];

        for id in machine_ids {
            let settings = get_string_value(
                RegistryHive::LocalMachine, 
                format!("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Group Policy\\DataStore\\Machine\\0\\{}", id).as_str(), 
                ""
            )?;
            results.push(settings);
        }

        println!("{:?}", results);

        let results_formatted: Vec<HashMap<String, VARIANT>> = results
            .iter()
            .map(|result| {
                let mut temp = HashMap::new();
                temp.insert(
                "GPO".to_string(),
                VARIANT::from(result.as_str()),
            );
            return temp;
        })
        .collect();

        Ok(Simple(CommandDTO {
            source: "GPO".to_string(),
            data: results_formatted,
        }))
    }
}

impl Default for LocalGPOCommand {
    /// Creates a default instance of `ExampleCommand`.
    ///
    /// # Returns
    ///
    /// A new `ExampleCommand` instance with default `CommandData`.
    fn default() -> Self {
        LocalGPOCommand {
            data: CommandData {
                support_remote: false,
            },
        }
    }
}
