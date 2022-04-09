#[macro_use]
extern crate rocket;
use rocket::response::content::Json;
use rocket::State;

mod blockchain;
use blockchain::blockchain::*;
use blockchain::master_chain::*;

#[launch]
fn rocket() -> _ {
    let mut blocks = Blockchain::new();
    let mut master_blocks = Master::new();

    let mut i = 0.0f64;

    // Add some blocks for testing

    loop {
        i = i + 1.0;
        if blocks.blocks.len() == 10 {
            master_blocks.validate_chain();
            blocks.validate_chain();
            master_blocks.add_master_block(blocks.blocks);
            blocks.blocks = vec![];
            blocks.genesus();
        }

        blocks.add_block(Transaction {
            sender: "test".to_string(),
            reciever: "net".to_string(),
            amount: i * 5.0,
        });
        if i == 100.0 {
            break;
        }
    }

    // Example Actions

    // print all the blocks

    // for block in &blocks.blocks {
    //     print!(
    //         "id:{}\n hash: {},\n previous_hash: {},\n Transaction: {:?}\n\n",
    //         block.id, block.block_hash, block.previous_hash, block.transaction
    //     );
    // }

    // print the master blockchain

    for master_block in &master_blocks.master_blocks {
        print!(
            "id: {}\n hash: {},\n previous_hash: {}, \n Blocks: {:?}\n\n",
            master_block.id,
            master_block.block_hash,
            master_block.previous_hash,
            master_block.block_data
        )
    }

    // Finding a block by known hash

    // let my_block = blocks.find_block_by_hash(
    //     "aa89390f927d1c05d70252caf9963e2c82c1fe153c906afe9fb51f0e3cd8c203".to_string(),
    // );

    // println!("{:?}", my_block);

    let my_block = master_blocks.find_block_by_hash(
        "ea678fa3238eb37fdf93a4da83bfba13969ff79e8e563925242813d9635dd94b".to_string(),
    );

    println!("{:?}", my_block);

    //  server
    rocket::build()
        .manage(blocks)
        .manage(master_blocks)
        .mount("/", routes![index])
        .mount("/blocks", routes![get_block])
        .mount("/master", routes![get_master_block_block])
}

// Get all blocks
#[get("/")]
fn index(blocks: &State<Blockchain>) -> Json<String> {
    let serialized = serde_json::to_string(&blocks.blocks).unwrap();
    Json(serialized)
}

// Get single block
#[get("/<hash>")]
fn get_block(hash: String, blocks: &State<Blockchain>) -> Json<String> {
    let block: Block = blocks.find_block_by_hash(hash);
    let serialized = serde_json::to_string(&block).unwrap();
    Json(serialized)
}

// Find Transaction

#[get("/<hash>")]
fn get_master_block_block(hash: String, master_blocks: &State<Master>) -> Json<String> {
    let block: Block = master_blocks.find_block_by_hash(hash);
    let serialized = serde_json::to_string(&block).unwrap();
    Json(serialized)
}