use clap::{Parser, Subcommand};
use colored::*;
use prettytable::{Table, row};
use rand::seq::SliceRandom;
use std::collections::HashMap;
use std::io::{self, Write};

#[derive(Parser)]
#[clap(author = "Connor Field", version = "1.0", about = "OSRS Random Generator")]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[clap(about = "Generate a random boss from various categories")]
    Boss,
    #[clap(about = "Generate a random skill to train")]
    Skill,
    #[clap(about = "Display help information")]
    Help,
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Boss) => generate_boss(),
        Some(Commands::Skill) => generate_skill(),
        Some(Commands::Help) => show_help(),
        None => interactive_menu(),
    }
}

fn interactive_menu() {
    loop {
        clear_screen();
        println!("\n{}\n", "OSRS Random Generator".bold().underline().cyan());
        println!("Please choose an option:");
        println!("1. Boss Chooser");
        println!("2. Skill Chooser");
        println!("3. Exit");
        print!("Enter your choice (1, 2, or 3): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => generate_boss(),
            "2" => generate_skill(),
            "3" => return,
            _ => println!("{}", "Invalid choice, please try again.".red()),
        }

        if !continue_prompt() {
            return;
        }
    }
}

fn show_help() {
    println!("OSRS Random Generator Help:\n");
    println!("1. Boss Chooser - Randomly select a boss from various categories.");
    println!("2. Skill Chooser - Randomly select a skill to train.");
    println!("3. Exit - Exit the application.\n");
    println!("You can use the interactive menu or pass commands directly as arguments.");
}

fn continue_prompt() -> bool {
    println!("{}", "Would you like to go back to the menu? (yes/no)".yellow());
    let mut answer = String::new();
    io::stdin().read_line(&mut answer).unwrap();
    answer.trim().eq_ignore_ascii_case("yes")
}

fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn load_bosses() -> HashMap<&'static str, Vec<&'static str>> {
    HashMap::from([
        ("World Bosses", vec![
            "Barrows", "Scurrius", "Giant Mole", "Deranged Archaeologist", "DKs", "Sarachnis",
            "Perilous Moons", "Kalphite Queen", "Corporeal Beast", "Zulrah", "Vorkath", "Phantom Muspah",
            "Nightmare", "Duke Sucellus", "The Leviathan", "The Whisperer", "Vardorvis", "Obor",
            "Bryophyta", "The Mimic", "Hespori", "Skotizo"
        ]),
        ("God Wars", vec!["Kree'arra", "Zilyana", "Graador", "K'ril", "Nex"]),
        ("Wilderness Bosses", vec![
            "Chaos Fanatic", "Crazy Archaeologist", "Scorpia", "King Black Dragon", "Calvar'ion",
            "Chaos Elemental", "Vet'ion", "Venenatis", "Callisto"
        ]),
        ("Slayer Only Bosses", vec![
            "Grotesque Guardians", "Abyssal Sire", "Kraken", "Cerberus", "Thermonuclear Smoke Devil",
            "Alchemical Hydra"
        ]),
        ("Minigame Bosses", vec!["Gauntlet", "TzTok-Jad", "TzKal-Zuk", "Sol Heredit"]),
        ("Skilling Bosses", vec!["Tempoross", "Wintertodt", "Zalcano"]),
        ("Raids", vec!["Chambers of Xeric", "Tombs of Amascut", "Theatre of Blood"]),
    ])
}

fn generate_skill() {
    let mut rng = rand::thread_rng();
    let skills = ["Attack", "Strength", "Defence", "Ranged", "Prayer", "Magic", "Hitpoints",
                  "Runecraft", "Crafting", "Mining", "Smithing", "Fishing", "Cooking", "Firemaking",
                  "Woodcutting", "Agility", "Herblore", "Thieving", "Fletching", "Slayer", "Farming",
                  "Construction", "Hunter"];
    let skill = skills.choose(&mut rng).unwrap();

    println!();
    let mut table = Table::new();
    table.add_row(row!["Randomly selected skill to train"]);
    table.add_row(row![skill.bold().green()]);
    table.printstd();
}

fn generate_boss() {
    let categories = load_bosses();
    let keys: Vec<&str> = categories.keys().cloned().collect();
    println!("\nAvailable Categories:");
    for (index, key) in keys.iter().enumerate() {
        println!("{}. {}", index + 1, key);
    }
    println!("Enter the number of any category you wish to exclude, or 0 to include all:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let exclusion = input.trim().parse::<usize>().unwrap_or(0);

    let mut rng = rand::thread_rng();
    let filtered_keys: Vec<&str> = if exclusion > 0 && exclusion <= keys.len() {
        keys.into_iter().enumerate().filter(|&(i, _)| i + 1 != exclusion).map(|(_, k)| k).collect()
    } else {
        keys
    };

    let category = filtered_keys.choose(&mut rng).expect("No categories available");
    let bosses = categories.get(category).expect("Category not found");
    let boss = bosses.choose(&mut rng).expect("No bosses found in category");

    println!();
    let mut table = Table::new();
    table.add_row(row!["Category", "Boss"]);
    table.add_row(row![category.bold().yellow(), boss.bold().green()]);
    table.printstd();
}
