use std::error::Error;


fn main() -> Result<(), Box<dyn Error>> {
    tonic_build::configure().compile_well_known_types(true);
    tonic_build::compile_protos("CamerAIProtos/Node.proto")?;

    Ok(())
}
