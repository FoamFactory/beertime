use beertime::beer::Beer;
use beertime::equipment::Equipment;
use beertime::equipment_group::EquipmentGroup;
use beertime::factory::Factory;
use beertime::interval::Interval;
use beertime::recipy::Recipy;
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
            format!("Fermentor 15G G10{}", i),
            System::G10,
            EquipmentGroup::Fermentor,
            Volume::GallonUS(15.0),
        ));
    }
    for i in 0..2 {
        eqs.push(Equipment::new(
            format!("CO2 Tank 5Lb G10{}", i),
            System::G10,
            EquipmentGroup::CO2Tank,
            Volume::Lb(5.0),
        ));
    }
    for i in 0..2 {
        eqs.push(Equipment::new(
            format!("Keg 5G {}", i),
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
            Recipy::new(
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
            Recipy::new(
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
            Recipy::new(
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
            Recipy::new(
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
            Recipy::new(
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
            Recipy::new(
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
            Recipy::new(
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
            Recipy::new(
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
            Recipy::new(
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
            Recipy::new(
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
            Recipy::new(
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
            Recipy::new(
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

fn wishlist(_factory: &Factory) -> Vec<(&'static str, Volume)> {
    vec![
        ("Bier", Volume::GallonUS(90.0)),
        ("Anti-Scurvy Elixir", Volume::GallonUS(90.0)),
        ("Autumn's Early Arrival Blonde", Volume::GallonUS(60.0)),
    ]
}

fn load(factory: &mut Factory) {
    load_equipment(factory);
    load_recipies(factory);
}

fn main() {
    let mut factory = Factory::new("Loons Landing");
    load(&mut factory);
    let wishlist = wishlist(&factory);
    let batches_needed = factory.calculate_batches(wishlist);
    assert_eq!(batches_needed.len(), 24);
    //FIXME: group theses steps together; there is too much hash/vector building and sorting going on
    println!("\nbatches needed: {:?}", batches_needed);
    let most_needed_steps = factory.calculate_bottleneck_step(batches_needed);
    println!("\nbottleneck step: {:?}", most_needed_steps);
    let most_needed_equipment = factory.calculate_bottleneck_equipment(most_needed_steps);
    println!("\nbottleneck equipment: {:?}", most_needed_equipment);
    let most_bottlenecked_equipment = factory.calculate_bottleneck(most_needed_equipment);
    println!("\nbottleneck : {:?}", most_bottlenecked_equipment);
    //todo plan around bottleneck
    //todo generate plan list
    //generate gantt chart
    //calculate oee's

    println!("ok");
}
