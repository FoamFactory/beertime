use std::collections::HashMap;

use clap::Parser;

use beertime::beer::Beer;
use beertime::config::Config;
use beertime::equipment::Equipment;
use beertime::equipment_group::EquipmentGroup;
use beertime::factory::Factory;
use beertime::interval::Interval;
use beertime::plan::Plan;
use beertime::recipe::Recipe;
use beertime::steps::Steps;
use beertime::style::Style;
use beertime::system::System;
use beertime::volume::Volume;

fn load_equipment(factory: &mut Factory) {
    let mut eqs = vec![
        Equipment::new(
            "Mash Tun 15G G10".to_string(),
            System::G10,
            EquipmentGroup::MashTun,
            Volume::GallonUS(15.0),
        ),
        Equipment::new(
            "Hot Liquor Tank 15G G10".to_string(),
            System::G10,
            EquipmentGroup::HotLiquorTank,
            Volume::GallonUS(15.0),
        ),
        Equipment::new(
            "Kettle 15G G10".to_string(),
            System::G10,
            EquipmentGroup::Kettle,
            Volume::GallonUS(15.0),
        ),
    ];
    for i in 0..7 {
        eqs.push(Equipment::new(
            format!("Fermentor 15G G10 {}", i + 1),
            System::G10,
            EquipmentGroup::Fermentor,
            Volume::GallonUS(15.0),
        ));
    }
    for i in 0..2 {
        eqs.push(Equipment::new(
            format!("CO2 Tank 5Lb G10 {}", i + 1),
            System::G10,
            EquipmentGroup::CO2Tank,
            Volume::Lb(5.0),
        ));
    }
    for i in 0..14 {
        eqs.push(Equipment::new(
            format!("Keg 5G {}", i + 1),
            System::G10,
            EquipmentGroup::Keg,
            Volume::GallonUS(5.0),
        ));
    }
    for eq in eqs {
        factory.equipments.insert(eq.name.to_string(), eq);
    }
}

fn load_recipies(factory: &mut Factory) {
    let beers = vec![
        Beer::new(
            "Dobroy Nochi",
            Style::ImperialStout,
            Recipe::new(
                System::G10,
                Volume::GallonUS(5.0),
                Steps::new(
                    Some(Interval::Hours(12)),
                    Some(Interval::Days(11)),
                    None,
                    Some(Interval::Weeks(14)),
                    Some(Interval::Months(4)),
                    Some(Interval::Days(2)),
                ),
            ),
        ),
        Beer::new(
            "Damned Squirrel",
            Style::BrownAle,
            Recipe::new(
                System::G10,
                Volume::GallonUS(10.0),
                Steps::new(
                    Some(Interval::Hours(6)),
                    Some(Interval::Days(7)),
                    None,
                    Some(Interval::Weeks(2)),
                    Some(Interval::Months(1)),
                    Some(Interval::Days(2)),
                ),
            ),
        ),
        Beer::new(
            "The Patriot",
            Style::AmberLager,
            Recipe::new(
                System::G10,
                Volume::GallonUS(10.0),
                Steps::new(
                    Some(Interval::Hours(6)),
                    Some(Interval::Days(11)),
                    Some(Interval::Days(7)),
                    Some(Interval::Months(1)),
                    Some(Interval::Months(4)),
                    Some(Interval::Days(2)),
                ),
            ),
        ),
        Beer::new(
            "Kung Fu Kicker",
            Style::SpecialtyStout,
            Recipe::new(
                System::G10,
                Volume::GallonUS(5.0),
                Steps::new(
                    Some(Interval::Hours(12)),
                    Some(Interval::Months(1)),
                    None,
                    Some(Interval::Months(4)),
                    Some(Interval::Months(6)),
                    Some(Interval::Days(2)),
                ),
            ),
        ),
        Beer::new(
            "Anti-Scurvy Elixir",
            Style::IPA,
            Recipe::new(
                System::G10,
                Volume::GallonUS(10.0),
                Steps::new(
                    Some(Interval::Hours(6)),
                    Some(Interval::Days(7)),
                    None,
                    Some(Interval::Weeks(5)),
                    Some(Interval::Months(1)),
                    Some(Interval::Days(2)),
                ),
            ),
        ),
        Beer::new(
            "Autumn's Early Arrival Blonde",
            Style::BlondeAle,
            Recipe::new(
                System::G10,
                Volume::GallonUS(10.0),
                Steps::new(
                    Some(Interval::Hours(6)),
                    Some(Interval::Days(9)),
                    None,
                    Some(Interval::Months(1)),
                    Some(Interval::Months(1)),
                    Some(Interval::Days(2)),
                ),
            ),
        ),
        Beer::new(
            "Blues Don't Bother Me",
            Style::FruitBeer,
            Recipe::new(
                System::G10,
                Volume::GallonUS(10.0),
                Steps::new(
                    Some(Interval::Hours(6)),
                    Some(Interval::Days(4)),
                    None,
                    Some(Interval::Weeks(6)),
                    Some(Interval::Months(1)),
                    Some(Interval::Days(2)),
                ),
            ),
        ),
        Beer::new(
            "36th St.",
            Style::SmokedAle,
            Recipe::new(
                System::G10,
                Volume::GallonUS(10.0),
                Steps::new(
                    Some(Interval::Hours(6)),
                    Some(Interval::Days(8)),
                    None,
                    Some(Interval::Months(1)),
                    Some(Interval::Months(3)),
                    Some(Interval::Days(2)),
                ),
            ),
        ),
        Beer::new(
            "Ironclad",
            Style::CaliforniaCommon,
            Recipe::new(
                System::G10,
                Volume::GallonUS(10.0),
                Steps::new(
                    Some(Interval::Hours(6)),
                    Some(Interval::Days(10)),
                    None,
                    Some(Interval::Weeks(6)),
                    Some(Interval::Weeks(2)),
                    Some(Interval::Days(2)),
                ),
            ),
        ),
        Beer::new(
            "Golden Ticket",
            Style::Kellerbier,
            Recipe::new(
                System::G10,
                Volume::GallonUS(10.0),
                Steps::new(
                    Some(Interval::Hours(6)),
                    Some(Interval::Days(9)),
                    Some(Interval::Days(4)),
                    Some(Interval::Weeks(6)),
                    Some(Interval::Months(1)),
                    Some(Interval::Days(2)),
                ),
            ),
        ),
        Beer::new(
            "Bier",
            Style::Pilsner,
            Recipe::new(
                System::G10,
                Volume::GallonUS(10.0),
                Steps::new(
                    Some(Interval::Hours(6)),
                    Some(Interval::Days(7)),
                    Some(Interval::Days(3)),
                    Some(Interval::Weeks(2)),
                    Some(Interval::Months(1)),
                    Some(Interval::Days(2)),
                ),
            ),
        ),
        Beer::new(
            "Red Sunset",
            Style::IrishRedAle,
            Recipe::new(
                System::G10,
                Volume::GallonUS(10.0),
                Steps::new(
                    Some(Interval::Hours(6)),
                    Some(Interval::Days(5)),
                    None,
                    Some(Interval::Days(9)),
                    Some(Interval::Weeks(2)),
                    Some(Interval::Days(2)),
                ),
            ),
        ),
    ];
    for beer in beers {
        factory.beers.insert(beer.name.to_string(), beer);
    }
}

fn wishlist(factory: &Factory) -> HashMap<&'static str, (&Beer, Volume)> {
    let config = vec![
        ("Bier", Volume::GallonUS(30.0)),
        ("Anti-Scurvy Elixir", Volume::GallonUS(70.0)),
        ("Autumn's Early Arrival Blonde", Volume::GallonUS(90.0)),
    ];
    let mut wishlist = HashMap::with_capacity(config.len());
    for (name, volume) in config {
        wishlist.insert(name, (factory.beers.get(name).unwrap(), volume));
    }
    wishlist
}

fn load(factory: &mut Factory) {
    load_equipment(factory);
    load_recipies(factory);
}

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
    /// File name of the factory definition toml file
    #[clap(short)]
    factory_definition_file: String,
}

fn main() {
    // As input, we need a factory definition .json file.
    let args = Args::parse();
    let def_file_path = args.factory_definition_file;
    match Config::read_config(def_file_path) {
        Ok(conf) => println!("Factory Name: {}", conf.factory.name),
        _ => panic!("Unable to read configuration file. Does it exist and in proper format?")
    }
    // let mut factory = Factory::new("Loons Landing");
    // load(&mut factory);
    // let wishlist = wishlist(&factory);
    // let batches_needed = factory.calculate_batches(wishlist);
    // assert_eq!(batches_needed.len(), 19);
    // let most_needed_steps = factory.calculate_bottleneck_step(&batches_needed);
    // let most_needed_equipment =
    //     factory.calculate_bottleneck_equipment(most_needed_steps.as_slice());
    // let _most_bottlenecked_equipment =
    //     factory.calculate_bottleneck(most_needed_equipment.as_slice());
    // //println!("\nbottleneck : {:?}", most_bottlenecked_equipment);
    // let now = chrono::offset::Utc::now();
    // let solution = Plan::plan(&factory, &batches_needed, now);
    // let pla = Plan::pla_basic(solution.as_slice(), Plan::sort_by_batch);
    // println!("{}", pla);
    // @TODO: Generate plan list
    // @TODO: calculate oee's
}
