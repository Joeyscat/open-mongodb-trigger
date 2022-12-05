use abi::{
    function::function_service_client::FunctionServiceClient,
    trigger::trigger_service_client::TriggerServiceClient, Func,
};
use anyhow::Result;
use clap::{Parser, Subcommand};
use std::{fs::File, io::Read, path::PathBuf, str::FromStr};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Test {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },

    /// trigger manager
    #[command(subcommand)]
    Trigger(TriggerCommands),

    /// function manager
    #[command(subcommand)]
    Function(FunctionCommands),
}

#[derive(Subcommand)]
enum TriggerCommands {
    /// create a trigger.
    Create {
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        user_id: String,
        /// now supported: database
        #[arg(short, long)]
        type_: String,
        #[arg(short, long)]
        source: String,
        #[arg(short, long)]
        database: String,
        #[arg(short, long)]
        collection: String,
        #[arg(short, long)]
        operation_types: String,
        #[arg(short, long)]
        function_id: String,
    },
    /// delete a trigger.
    Delete {
        #[arg(short, long)]
        id: String,
    },
    /// disable a trigger.
    Disable {
        #[arg(short, long)]
        id: String,
    },
    /// enable a trigger.
    Enable {
        #[arg(short, long)]
        id: String,
    },
    /// get a trigger.
    Get {
        #[arg(short, long)]
        id: String,
    },
}

#[derive(Subcommand)]
enum FunctionCommands {
    /// create a function.
    Create {
        #[arg(short, long)]
        name: String,
        /// path of the function file (*.wasm)
        #[arg(short, long)]
        path: String,
        /// now supported: wasm
        #[arg(short, long)]
        type_: String,
        #[arg(short, long)]
        user_id: String,
        #[arg(short, long)]
        lang: String,
    },
    /// update file for a function.
    UpdateFile {
        #[arg(short, long)]
        id: String,
        /// path of the function file (*.wasm)
        #[arg(short, long)]
        path: String,
    },
    /// delete a function.
    Delete {
        #[arg(short, long)]
        id: String,
    },
    /// get a function.
    Get {
        #[arg(short, long)]
        id: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // You can check the value provided by positional arguments, or option arguments
    if let Some(name) = cli.name.as_deref() {
        println!("Value for name: {}", name);
    }

    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match cli.debug {
        0 => {
            // println!("Debug mode is off")
        }
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    let dst = "http://127.0.0.1:6788";

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match cli.command {
        Some(Commands::Test { list }) => {
            if list {
                println!("Printing testing lists...");
            } else {
                println!("Not printing testing lists...");
            }
            Ok(())
        }
        Some(Commands::Trigger(cmd)) => match cmd {
            TriggerCommands::Create {
                name,
                user_id,
                type_,
                source,
                database,
                collection,
                operation_types,
                function_id,
            } => {
                let trigger = match type_.as_str() {
                    "database" => {
                        let op_types = operation_types
                            .split(',')
                            .collect::<Vec<&str>>()
                            .iter()
                            .map(|x| match *x {
                                "insert" => 1,
                                "update" => 2,
                                "replace" => 3,
                                "delete" => 4,
                                _ => 0,
                            })
                            .collect::<Vec<i32>>();
                        let config = abi::trigger::DatabaseConfig {
                            data_source: source,
                            database,
                            collection,
                            operation_types: op_types,
                        };
                        abi::Trigger::new_database(user_id, name, function_id, false, config)
                    }
                    _ => {
                        println!("unsupported trigger type: {}", type_);
                        return Ok(());
                    }
                };
                let mut client = TriggerServiceClient::connect(dst).await?;
                let request = tonic::Request::new(abi::trigger::CreateRequest {
                    trigger: Some(trigger),
                });

                let response = client.create(request).await?;

                let r = response.get_ref().clone();
                println!("{}", r.trigger.unwrap().id);
                Ok(())
            }
            TriggerCommands::Delete { id } => {
                let mut client = TriggerServiceClient::connect(dst).await?;
                let request = tonic::Request::new(abi::trigger::DeleteRequest { id });
                let response = client.delete(request).await?;

                let r = response.get_ref().clone();
                println!("{}", r.trigger.unwrap().id);
                Ok(())
            }
            TriggerCommands::Disable { id } => {
                let mut client = TriggerServiceClient::connect(dst).await?;
                let request = tonic::Request::new(abi::trigger::DisableRequest { id });
                let response = client.disable(request).await?;

                let r = response.get_ref().clone();
                println!("{}", r.trigger.unwrap().id);
                Ok(())
            }
            TriggerCommands::Enable { id } => {
                let mut client = TriggerServiceClient::connect(dst).await?;
                let request = tonic::Request::new(abi::trigger::EnableRequest { id });
                let response = client.enable(request).await?;

                let r = response.get_ref().clone();
                println!("{}", r.trigger.unwrap().id);
                Ok(())
            }
            TriggerCommands::Get { id } => {
                let mut client = TriggerServiceClient::connect(dst).await?;
                let request = tonic::Request::new(abi::trigger::GetRequest { id });
                let response = client.get(request).await?;

                let t = response.get_ref().clone().trigger.unwrap();
                let output = format!(
                    r#"id: {}
name: {}
user_id: {}
type: {}
function_id: {}"#,
                    t.id, t.name, t.user_id, t.trigger_type, t.function_id
                );
                println!("{}", output);
                Ok(())
            }
        },
        Some(Commands::Function(cmd)) => match cmd {
            FunctionCommands::Create {
                name,
                path,
                type_,
                user_id,
                lang,
            } => {
                let function = match type_.as_str() {
                    "wasm" => {
                        let lang = abi::function::Lang::from_str(&lang)?;

                        let mut f = File::open(path)?;
                        let mut func_bytes = Vec::new();
                        f.read_to_end(&mut func_bytes)?;
                        abi::Function::new_wasm(user_id, name, func_bytes, lang)
                    }
                    _ => {
                        println!("unsupported function type: {}", type_);
                        return Ok(());
                    }
                };
                let mut client = FunctionServiceClient::connect(dst).await?;
                let request = tonic::Request::new(abi::function::CreateRequest {
                    function: Some(function),
                });
                let response = client.create(request).await?;

                let r = response.get_ref().clone();
                println!("{}", r.function.unwrap().id);
                Ok(())
            }
            FunctionCommands::UpdateFile { id, path } => {
                let mut client = FunctionServiceClient::connect(dst).await?;
                let mut f = File::open(path)?;
                let mut func_bytes = Vec::new();
                f.read_to_end(&mut func_bytes)?;
                let func = Some(abi::function::update_request::Func::Wasm(func_bytes));

                let request = tonic::Request::new(abi::function::UpdateRequest { id, func });
                let response = client.update(request).await?;

                let r = response.get_ref().clone();
                println!("{}", r.function.unwrap().id);
                Ok(())
            }
            FunctionCommands::Delete { id } => {
                let mut client = FunctionServiceClient::connect(dst).await?;
                let request = tonic::Request::new(abi::function::DeleteRequest { id });
                let response = client.delete(request).await?;

                let r = response.get_ref().clone();
                println!("{}", r.function.unwrap().id);
                Ok(())
            }
            FunctionCommands::Get { id } => {
                let mut client = FunctionServiceClient::connect(dst).await?;
                let request = tonic::Request::new(abi::function::GetRequest { id });
                let response = client.get(request).await?;

                let f = response.get_ref().clone().function.unwrap();
                let output = format!(
                    r#"id: {}
name: {}
user_id: {}
type: {}
func: {}"#,
                    f.id,
                    f.name,
                    f.user_id,
                    f.function_type,
                    match f.func {
                        Some(Func::Wasm(x)) => format!("{} bytes wasm binary", x.len()),
                        None => "unknown".to_string(),
                    }
                );
                println!("{}", output);

                Ok(())
            }
        },

        None => Ok(()),
    }

    // Continued program logic goes here...
}
