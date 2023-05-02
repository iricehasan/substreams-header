mod pb;

#[substreams::handlers::map]
fn map_hello_world(blk: pb::cosmos::Block) -> Result<pb::cosmos::Header, substreams::errors::Error> {
    let header = blk.header.unwrap();
    Ok(header)
}