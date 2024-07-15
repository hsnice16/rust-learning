use object_oriented_concepts::AveragedCollection;

fn main() {
    let mut averaged_collection = AveragedCollection::new();
    averaged_collection.add(12);
    averaged_collection.add(13);

    println!("Averaged collection list: {averaged_collection:#?}");
    println!("Average: {}", averaged_collection.average());
}
