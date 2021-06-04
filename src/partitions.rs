use std::path::Path;
use gpt::partition::{Partition};
use std::collections::BTreeMap;
use std::io;

fn get_partitions_for_dev(dev_path: &Path) -> Result<BTreeMap<u32, Partition>, io::Error> {
	println!("Testing path {}", dev_path.display());
	let size = gpt::disk::LogicalBlockSize::Lb4096;
	let header = gpt::header::read_header(dev_path, size);
	match header {
		Ok(header) => gpt::partition::read_partitions(dev_path, &header, size),
		Err(e) => Err(e),
	}
}

pub fn get_boot_partitions() -> (Partition, Partition) {
	//Open relevant GPT stuff

	let blockdevs = block_utils::get_block_devices();
	assert!(!blockdevs.is_err());

	let blockdevs = blockdevs.unwrap();

	let dev_path = blockdevs.into_iter()
		.find(|dev_path| { match get_partitions_for_dev(dev_path.as_path()) {
			Ok(dev) => dev.values().find(|p| p.name == "boot_a").is_some(),
			Err(e) => false,
		}});
	if dev_path.is_none() {
		panic!("Failed to find boot_a, does this device have slots?");
	}
	let dev_path = dev_path.unwrap();

	let all_partitions =  get_partitions_for_dev(dev_path.as_path()).unwrap();
	let boot_a = all_partitions.values().find(|x| x.name == "boot_a").unwrap().clone();
	let boot_b = all_partitions.values().find(|x| x.name == "boot_b").unwrap().clone();

	// for dev_path in blockdevs {
	// 	let header = gpt::header::read_header(dev_path.as_path(), size).unwrap();
	// 	let mut partitions = gpt::partition::read_partitions(dev_path.as_path(), &header, size).unwrap();
	// 	let res = partitions.values().try_find(|x| x.name == "boot_a");
	// 	if res.is_err() {
	// 		continue;
	// 	}
	// }



	// let boot_a = partitions.values().find(|x| x.name == "boot_a").unwrap();


	return (boot_a, boot_b)
}
