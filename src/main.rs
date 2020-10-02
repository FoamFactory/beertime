use beertime::factory::Factory;

fn load(_factory: &mut Factory) {
    // equipment
}

fn main() {
    let mut factory = Factory::new("Loons Landing");
    load(&mut factory);

    println!("ok");
}
