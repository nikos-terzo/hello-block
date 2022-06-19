use ethers::contract::Abigen;

fn main() {
    Abigen::new("ChatterContract", "/workspaces/hello-block/contract_src/Chatter.abi.json")
        .expect("could not construct Abigen")
        .generate()
        .expect("could not generate Abigen")
        .write_to_file("src/chatter/contract.rs")
        .expect("could not write to file");
}
