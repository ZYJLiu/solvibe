use console::style;
use dialoguer::{Input, Select, theme::ColorfulTheme};
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::Duration;

use crate::cli::InitOptions;

// Add a MenuAction enum to control menu flow
#[derive(Debug, PartialEq)]
enum MenuAction {
    Continue,
    Back,
    Cancel,
}

pub fn handle_init(opts: InitOptions) {
    let client_repo = "https://github.com/ZYJLiu/solana-client";
    let fullstack_repo_web3js = "https://github.com/solana-developers/anchor-web3js-nextjs";
    let fullstack_repo_kit = "https://github.com/ZYJLiu/nextjs-anchor-codama";

    if opts.client {
        client_install_flow(client_repo);
    } else if opts.full {
        // Add selection between Kit and Web3js
        let fullstack_options = &["Kit", "Web3js"];
        let fullstack_choice = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Which fullstack template do you want to use?")
            .default(0)
            .items(fullstack_options)
            .interact()
            .unwrap();

        let dir = get_directory_name();

        let repo_to_clone = match fullstack_choice {
            0 => fullstack_repo_kit,
            1 => fullstack_repo_web3js,
            _ => fullstack_repo_web3js,
        };

        println!("Cloning fullstack program from {}...", repo_to_clone);
        clone_repo_into(repo_to_clone, &dir);
    } else {
        // Top-level prompt: client or fullstack
        loop {
            let top_options = &[
                "Client code (Typescript or Rust)",
                "Fullstack Anchor program with Nextjs (Kit or Web3js)",
                "Cancel",
            ];
            let top_selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("What template do you want to use?")
                .default(0)
                .items(top_options)
                .interact()
                .unwrap();
            if top_selection == 2 {
                println!("Cancelled.");
                return;
            }

            let action = match top_selection {
                0 => client_install_flow(client_repo),
                1 => {
                    // Add selection between Kit and Web3js
                    let fullstack_options = &["Kit", "Web3js"];
                    let fullstack_choice = Select::with_theme(&ColorfulTheme::default())
                        .with_prompt("Which fullstack template do you want to use?")
                        .default(0)
                        .items(fullstack_options)
                        .interact()
                        .unwrap();

                    let dir = get_directory_name();

                    let repo_to_clone = match fullstack_choice {
                        0 => fullstack_repo_kit,
                        1 => fullstack_repo_web3js,
                        _ => fullstack_repo_web3js,
                    };

                    println!("Cloning fullstack program from {}...", repo_to_clone);
                    clone_repo_into(repo_to_clone, &dir);
                    MenuAction::Continue
                }
                _ => MenuAction::Cancel,
            };
            if action == MenuAction::Back {
                // Should not happen at top-level, but just in case
                continue;
            } else {
                break;
            }
        }
    }
}

fn get_directory_name() -> String {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter directory name to clone into (default: starter)")
        .default("starter".into())
        .interact_text()
        .unwrap()
}

fn client_install_flow(repo_url: &str) -> MenuAction {
    loop {
        // Main client options
        let main_options = &["All", "Typescript Client", "Rust Client", "Back", "Cancel"];
        let main_choice = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Which client templates do you want to use? (Select one)")
            .default(0)
            .items(main_options)
            .interact()
            .unwrap();
        match main_choice {
            0 => {
                let dir = get_directory_name();
                clone_and_install_subdirs(
                    repo_url,
                    &dir,
                    &[
                        "typescript/kit",
                        "typescript/web3js",
                        "rust/async",
                        "rust/sync",
                    ],
                );
                return MenuAction::Continue;
            }
            1 => {
                let action = typescript_menu(repo_url);
                if action == MenuAction::Back {
                    continue;
                } else {
                    return action;
                }
            }
            2 => {
                let action = rust_menu(repo_url);
                if action == MenuAction::Back {
                    continue;
                } else {
                    return action;
                }
            }
            3 => return MenuAction::Back,
            4 => return MenuAction::Cancel,
            _ => println!("Cancelled."),
        }
    }
}

fn typescript_menu(repo_url: &str) -> MenuAction {
    loop {
        let ts_options = &["All", "Kit", "Web3.js", "Go Back", "Cancel"];
        let ts_choice = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Which Typescript client template do you want to use? (Select one)")
            .default(0)
            .items(ts_options)
            .interact()
            .unwrap();
        match ts_choice {
            0 => {
                let dir = get_directory_name();
                clone_and_install_subdirs(
                    repo_url,
                    &dir,
                    &["typescript/kit", "typescript/web3js"],
                );
                return MenuAction::Continue;
            }
            1 => {
                let dir = get_directory_name();
                clone_and_install_subdirs(repo_url, &dir, &["typescript/kit"]);
                return MenuAction::Continue;
            }
            2 => {
                let dir = get_directory_name();
                clone_and_install_subdirs(repo_url, &dir, &["typescript/web3js"]);
                return MenuAction::Continue;
            }
            3 => return MenuAction::Back,
            4 => return MenuAction::Cancel,
            _ => println!("Cancelled."),
        }
    }
}

fn rust_menu(repo_url: &str) -> MenuAction {
    loop {
        let rust_options = &["All", "Async (tokio)", "Sync", "Go Back", "Cancel"];
        let rust_choice = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Which Rust client template do you want to use? (Select one)")
            .default(0)
            .items(rust_options)
            .interact()
            .unwrap();
        match rust_choice {
            0 => {
                let dir = get_directory_name();
                clone_and_install_subdirs(repo_url, &dir, &["rust/async", "rust/sync"]);
                return MenuAction::Continue;
            }
            1 => {
                let dir = get_directory_name();
                clone_and_install_subdirs(repo_url, &dir, &["rust/async"]);
                return MenuAction::Continue;
            }
            2 => {
                let dir = get_directory_name();
                clone_and_install_subdirs(repo_url, &dir, &["rust/sync"]);
                return MenuAction::Continue;
            }
            3 => return MenuAction::Back,
            4 => return MenuAction::Cancel,
            _ => println!("Cancelled."),
        }
    }
}

fn clone_and_install_subdirs(repo_url: &str, repo_name: &str, subdirs: &[&str]) {
    println!(
        "Cloning selected directories from {}: {:?}",
        repo_url, subdirs
    );
    let _ = Command::new("git")
        .arg("clone")
        .arg("--filter=blob:none")
        .arg("--sparse")
        .arg(repo_url)
        .arg(repo_name)
        .status();
    let _ = Command::new("git")
        .arg("-C")
        .arg(repo_name)
        .arg("sparse-checkout")
        .arg("set")
        .args(subdirs)
        .status();

    let mut instructions = Vec::new();
    // Helper: get language and template name from subdir string
    fn split_lang_template(sub: &str) -> Option<(&str, &str)> {
        sub.split_once('/')
    }
    // Group subdirs by language
    let mut lang_map: std::collections::HashMap<&str, Vec<&str>> = std::collections::HashMap::new();
    for sub in subdirs {
        if let Some((lang, template)) = split_lang_template(sub) {
            lang_map.entry(lang).or_default().push(template);
        }
    }
    // Define all possible templates for each language
    let all_ts = ["kit", "web3js"];
    let all_rust = ["async", "sync"];
    let is_single_lang_all = lang_map.len() == 1
        && ((lang_map.contains_key("typescript") && lang_map["typescript"].len() == all_ts.len())
            || (lang_map.contains_key("rust") && lang_map["rust"].len() == all_rust.len()));
    if is_single_lang_all {
        // Move each subdir up to <repo_name>/<template>
        let (lang, templates) = lang_map.into_iter().next().unwrap();
        let templates_vec: Vec<_> = templates.clone();
        for template in &templates_vec {
            let src_path = Path::new(repo_name).join(lang).join(template);
            let dest_path = Path::new(repo_name).join(template);
            let _ = fs::remove_dir_all(&dest_path);
            let _ = fs::rename(&src_path, &dest_path);
        }
        // Remove now-empty parent language directory
        let lang_path = Path::new(repo_name).join(lang);
        let _ = fs::remove_dir_all(&lang_path);
        // Use <repo_name>/<template> for instructions
        for template in &templates_vec {
            let path = Path::new(repo_name).join(template);
            let mut showed_instructions = false;
            if path.join("Cargo.toml").exists() {
                instructions.push(format!(
                    "{sep}\nRust template: '{template}'\n{sep}\n  cd {dir}\n  cargo r\n",
                    sep = "-".repeat(30),
                    template = template,
                    dir = path.display()
                ));
                showed_instructions = true;
            }
            if path.join("package.json").exists() {
                let msg = format!("Running 'pnpm install' in {:?}...", path);
                let pb = ProgressBar::new_spinner();
                pb.set_message(msg.clone());
                pb.set_style(ProgressStyle::with_template("{spinner} {msg}").unwrap());
                pb.enable_steady_tick(Duration::from_millis(100));
                let status = Command::new("pnpm")
                    .arg("install")
                    .current_dir(&path)
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .status();
                pb.finish_and_clear();
                match status {
                    Ok(s) if s.success() => {
                        println!(
                            "{} {}",
                            console::Emoji("✅ ", "[ok]"),
                            style("pnpm install succeeded").green()
                        );
                    }
                    Ok(s) => {
                        println!(
                            "{} {} (status: {})",
                            console::Emoji("❌ ", "[fail]"),
                            style("pnpm install failed").red(),
                            s
                        );
                    }
                    Err(e) => {
                        println!(
                            "{} {} ({})",
                            console::Emoji("❌ ", "[fail]"),
                            style("pnpm install error").red(),
                            e
                        );
                    }
                }
                instructions.push(format!(
                    "{sep}\nTypescript template: '{template}'\n{sep}\n  cd {dir}\n  pnpm start\n",
                    sep = "-".repeat(30),
                    template = template,
                    dir = path.display()
                ));
                showed_instructions = true;
            }
            if !showed_instructions {
                println!(
                    "No recognized project type in '{}', skipping instructions.",
                    template
                );
            }
        }
    } else if subdirs.len() == 1 {
        let sub = subdirs[0];
        let src_path = Path::new(repo_name).join(sub);
        let dest_path = Path::new(repo_name);
        if let Ok(entries) = fs::read_dir(&src_path) {
            for entry in entries.flatten() {
                let entry_path = entry.path();
                let file_name = entry.file_name();
                let dest_file_path = dest_path.join(&file_name);
                let _ = fs::remove_dir_all(&dest_file_path);
                let _ = fs::rename(&entry_path, &dest_file_path);
            }
        }
        let _ = fs::remove_dir_all(&src_path);
        if let Some(parent) = Path::new(sub).parent() {
            let parent_path = Path::new(repo_name).join(parent);
            if parent_path
                .read_dir()
                .map(|mut i| i.next().is_none())
                .unwrap_or(false)
            {
                let _ = fs::remove_dir_all(&parent_path);
            }
        }
        let path = Path::new(repo_name);
        let mut showed_instructions = false;
        if path.join("Cargo.toml").exists() {
            instructions.push(format!(
                "{sep}\nRust template: '{sub}'\n{sep}\n  cd {dir}\n  cargo r\n",
                sep = "-".repeat(30),
                sub = sub,
                dir = path.display()
            ));
            showed_instructions = true;
        }
        if path.join("package.json").exists() {
            let msg = format!("Running 'pnpm install' in {:?}...", path);
            let pb = ProgressBar::new_spinner();
            pb.set_message(msg.clone());
            pb.set_style(ProgressStyle::with_template("{spinner} {msg}").unwrap());
            pb.enable_steady_tick(Duration::from_millis(100));
            let status = Command::new("pnpm")
                .arg("install")
                .current_dir(path)
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status();
            pb.finish_and_clear();
            match status {
                Ok(s) if s.success() => {
                    println!(
                        "{} {}",
                        console::Emoji("✅ ", "[ok]"),
                        style("pnpm install succeeded").green()
                    );
                }
                Ok(s) => {
                    println!(
                        "{} {} (status: {})",
                        console::Emoji("❌ ", "[fail]"),
                        style("pnpm install failed").red(),
                        s
                    );
                }
                Err(e) => {
                    println!(
                        "{} {} ({})",
                        console::Emoji("❌ ", "[fail]"),
                        style("pnpm install error").red(),
                        e
                    );
                }
            }
            instructions.push(format!(
                "{sep}\nTypescript template: '{sub}'\n{sep}\n  cd {dir}\n  pnpm start\n",
                sep = "-".repeat(30),
                sub = sub,
                dir = path.display()
            ));
            showed_instructions = true;
        }
        if !showed_instructions {
            println!(
                "No recognized project type in '{}', skipping instructions.",
                sub
            );
        }
    } else {
        for sub in subdirs {
            let path = Path::new(repo_name).join(sub);
            let mut showed_instructions = false;
            if path.join("Cargo.toml").exists() {
                instructions.push(format!(
                    "{sep}\nRust template: '{sub}'\n{sep}\n  cd {dir}\n  cargo r\n",
                    sep = "-".repeat(30),
                    sub = sub,
                    dir = path.display()
                ));
                showed_instructions = true;
            }
            if path.join("package.json").exists() {
                let msg = format!("Running 'pnpm install' in {:?}...", path);
                let pb = ProgressBar::new_spinner();
                pb.set_message(msg.clone());
                pb.set_style(ProgressStyle::with_template("{spinner} {msg}").unwrap());
                pb.enable_steady_tick(Duration::from_millis(100));
                let status = Command::new("pnpm")
                    .arg("install")
                    .current_dir(&path)
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .status();
                pb.finish_and_clear();
                match status {
                    Ok(s) if s.success() => {
                        println!(
                            "{} {}",
                            console::Emoji("✅ ", "[ok]"),
                            style("pnpm install succeeded").green()
                        );
                    }
                    Ok(s) => {
                        println!(
                            "{} {} (status: {})",
                            console::Emoji("❌ ", "[fail]"),
                            style("pnpm install failed").red(),
                            s
                        );
                    }
                    Err(e) => {
                        println!(
                            "{} {} ({})",
                            console::Emoji("❌ ", "[fail]"),
                            style("pnpm install error").red(),
                            e
                        );
                    }
                }
                instructions.push(format!(
                    "{sep}\nTypescript template: '{sub}'\n{sep}\n  cd {dir}\n  pnpm start\n",
                    sep = "-".repeat(30),
                    sub = sub,
                    dir = path.display()
                ));
                showed_instructions = true;
            }
            if !showed_instructions {
                println!(
                    "No recognized project type in '{}', skipping instructions.",
                    sub
                );
            }
        }
    }
    if !instructions.is_empty() {
        println!();
        println!("{}", style("====================").blue().bold());
        println!(
            "{}",
            style("How to run your installed templates:")
                .bold()
                .underlined()
        );
        println!("{}\n", style("====================").blue().bold());
        println!(
            "{} {}{}{}",
            style("1.").green().bold(),
            style("Run '").bold(),
            style("solana-test-validator").yellow().bold(),
            style("' in a separate terminal").bold()
        );
        println!("   {}\n", style("(If you don't have the Solana CLI installed, run 'solvibe install' to install all Solana dependencies.)").italic().dim());
        println!(
            "{} {}",
            style("2.").green().bold(),
            style("For each client template:").bold()
        );
        for msg in instructions {
            println!("{}", style(msg).cyan());
        }
        println!("{}", style("====================").blue().bold());
    }
}

fn clone_repo_into(repo_url: &str, dir: &str) {
    let status = Command::new("git")
        .arg("clone")
        .arg(repo_url)
        .arg(dir)
        .status();
    match status {
        Ok(s) if s.success() => println!("Repository cloned successfully into {}.", dir),
        Ok(s) => eprintln!("Git exited with status: {}", s),
        Err(e) => {
            eprintln!("Failed to run git: {}", e);
            return;
        }
    }

    // Handle different fullstack repos
    if repo_url.contains("anchor-web3js-nextjs") {
        let frontend_dir = Path::new(dir).join("frontend");
        let program_dir = Path::new(dir).join("program");

        // Run pnpm install in frontend
        if frontend_dir.exists() {
            println!("Running 'pnpm install' in {:?}...", frontend_dir);
            let pb = ProgressBar::new_spinner();
            pb.set_message("pnpm install (frontend)");
            pb.set_style(ProgressStyle::with_template("{spinner} {msg}").unwrap());
            pb.enable_steady_tick(Duration::from_millis(100));
            let status = Command::new("pnpm")
                .arg("install")
                .current_dir(&frontend_dir)
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status();
            pb.finish_and_clear();
            match status {
                Ok(s) if s.success() => println!("✅ pnpm install succeeded in frontend"),
                Ok(s) => eprintln!("❌ pnpm install failed in frontend (status: {})", s),
                Err(e) => eprintln!("❌ pnpm install error in frontend: {}", e),
            }
        } else {
            eprintln!("frontend directory not found in {}", dir);
        }

        // Run yarn in program
        if program_dir.exists() {
            println!("Running 'yarn' in {:?}...", program_dir);
            let pb = ProgressBar::new_spinner();
            pb.set_message("yarn (program)");
            pb.set_style(ProgressStyle::with_template("{spinner} {msg}").unwrap());
            pb.enable_steady_tick(Duration::from_millis(100));
            let status = Command::new("yarn")
                .current_dir(&program_dir)
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status();
            pb.finish_and_clear();
            match status {
                Ok(s) if s.success() => println!("✅ yarn succeeded in program"),
                Ok(s) => eprintln!("❌ yarn failed in program (status: {})", s),
                Err(e) => eprintln!("❌ yarn error in program: {}", e),
            }
        } else {
            eprintln!("program directory not found in {}", dir);
        }

        // Print instructions for running frontend and testing program
        println!();
        println!("{}", style("====================").blue().bold());
        println!(
            "{}",
            style("How to run your fullstack project:")
                .bold()
                .underlined()
        );
        println!("{}\n", style("====================").blue().bold());
        println!("{}", style("To run the frontend:").green().bold(),);
        println!("  cd {}/frontend", dir);
        println!("  pnpm dev\n");
        println!("{}", style("To test the Anchor program:").green().bold(),);
        println!("  cd {}/program", dir);
        println!("  anchor test\n");
        println!("{}", style("====================").blue().bold());
    } else if repo_url.contains("nextjs-anchor-codama") {
        // Handle Kit repo as monorepo - just run pnpm install at root
        let repo_dir = Path::new(dir);

        println!("Running 'pnpm install' in {:?}...", repo_dir);
        let pb = ProgressBar::new_spinner();
        pb.set_message("pnpm install (monorepo)");
        pb.set_style(ProgressStyle::with_template("{spinner} {msg}").unwrap());
        pb.enable_steady_tick(Duration::from_millis(100));
        let status = Command::new("pnpm")
            .arg("install")
            .current_dir(repo_dir)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
        pb.finish_and_clear();
        match status {
            Ok(s) if s.success() => println!("✅ pnpm install succeeded"),
            Ok(s) => eprintln!("❌ pnpm install failed (status: {})", s),
            Err(e) => eprintln!("❌ pnpm install error: {}", e),
        }

        // Print instructions for running Kit project
        println!();
        println!("{}", style("====================").blue().bold());
        println!(
            "{}",
            style("How to run your fullstack Kit project:")
                .bold()
                .underlined()
        );
        println!("{}\n", style("====================").blue().bold());
        println!("{}", style("To start the project:").green().bold());
        println!("  cd {}", dir);
        println!("  pnpm start");
        println!("\n  {}", style("(This will build the program, generate Codama clients, run tests, and start the frontend)").italic().dim());
        println!("{}", style("====================").blue().bold());
    }
}
