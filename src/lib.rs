
#[cfg(test)]
mod tests {
    #[test]
    fn chatter_compiles() {
        use ethers::solc::Solc;

        // relative path, bad practice, I know
        let compiled = Solc::default().compile_source("./contract_src/Chatter.sol")
            .expect("could not generate compiler output");
        
        println!("{:?}", compiled.errors);
        assert!(!compiled.has_error())
    }
}