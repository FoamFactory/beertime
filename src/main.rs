use beertime::equipment::Equipment;
use beertime::equipment_group::EquipmentGroup;
use beertime::factory::Factory;
use beertime::system::System;
use beertime::volume::Volume;

fn load(factory: &mut Factory) {
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
        factory.equipments.push(eq);
    }

    // equipment
}

fn main() {
    let mut factory = Factory::new("Loons Landing");
    load(&mut factory);

    println!("ok");
}
