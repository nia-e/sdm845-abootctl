use gpt::disk;
use gpt::partition::Partition;
use std::collections::BTreeMap;
use std::io;
use std::path::Path;

fn get_partitions_for_dev(dev_path: &Path) -> Result<BTreeMap<u32, Partition>, io::Error> {
    //println!("Testing path {}", dev_path.display());
    let size = disk::LogicalBlockSize::Lb4096;
    let header = gpt::header::read_header(dev_path, size);
    match header {
        Ok(header) => gpt::partition::read_partitions(dev_path, &header, size),
        Err(e) => panic!("{}", e),
    }
}

pub fn get_boot_partitions() -> (Partition, Partition, String) {
    //Open relevant GPT stuff

    let blockdevs = block_utils::get_block_devices();
    assert!(!blockdevs.is_err());

    let blockdevs = blockdevs.unwrap();

    let dev_path =
        blockdevs.into_iter().find(
            |dev_path| match get_partitions_for_dev(dev_path.as_path()) {
                Ok(dev) => dev.values().any(|p| p.name == "boot_a"),
                Err(_e) => false,
            },
        );
    if dev_path.is_none() {
        panic!("Failed to find boot_a, does this device have slots?");
    }
    let dev_path = dev_path.unwrap();

    let all_partitions = get_partitions_for_dev(dev_path.as_path()).unwrap();
    let boot_a = all_partitions
        .values()
        .find(|x| x.name == "boot_a")
        .unwrap()
        .clone();
    let boot_b = all_partitions
        .values()
        .find(|x| x.name == "boot_b")
        .unwrap()
        .clone();

    return (
        boot_a,
        boot_b,
        dev_path.as_path().to_str().unwrap().to_string(),
    );
}

pub fn set_boot_partition(new_partition: Partition, path: &str) {
    println!("Path: {}", path);
    let path = Path::new(&path);
    let config = gpt::GptConfig::new()
        .logical_block_size(disk::LogicalBlockSize::Lb4096)
        .writable(true);

    let mut disk = config.open(path).unwrap();
    let part_table = disk.partitions();

    let mut new_part_table = part_table.clone();

    if !part_table
        .values()
        .any(|partition| partition.name == new_partition.name)
    {
        panic!("Partition {} not found!", new_partition.name);
    }

    for (key, partition) in part_table.into_iter() {
        if partition.name == new_partition.name {
            println!("Partition name {} matches updated partition {}", partition.name, new_partition.name);
            new_part_table.insert(*key, new_partition.clone());
            break;
        }
    }

    println!("NOT WRITING TO DISK!!!");
    println!("{:?}", new_part_table);

    match disk.update_partitions(new_part_table) {
        Ok(_) => println!("Updated partition table"),
        Err(e) => panic!("Error while updating partitions: {}", e),
    }

    match disk.write_inplace() {
        Ok(_) => println!("Changes successfully written to disk"),
        Err(e) => panic!("Error while writing changes to disk: {}", e),
    }
}
