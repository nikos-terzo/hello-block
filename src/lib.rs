use ini::Ini;

fn get_ganache_host(ini_file: &str) -> String {
    let conf = Ini::load_from_file(ini_file)
        .expect("could not load ini file");
    
    let key = "GanacheHost";
    let ganache_host = conf.section(None::<String>).unwrap().get(key)
        .expect(&format!("could not find key: {}", key));
    ganache_host.to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn chatter_compiles() {
        use ethers::solc::Solc;

        // relative path, bad practice
        let compiled = Solc::default().compile_source("./contract_src/Chatter.sol")
            .expect("could not generate compiler output");
        
        println!("{:?}", compiled.errors);
        assert!(!compiled.has_error())
    }

    #[test]
    fn ganache_host_parses_from_ini() {
        use crate::get_ganache_host;

        // relative path, bad practice
        let ganache_host = get_ganache_host("./ChangeMe.ini");
        assert_eq!(&ganache_host[..7], "http://", "ganache_host did not start with \"http://\"");
    }
    
    #[test]
    fn provider_instatiates() {
        use crate::get_ganache_host;
        use ethers::providers::{Provider, Http};
        
        // relative path, bad practice
        let ganache_host = get_ganache_host("./ChangeMe.ini");
        let _provider = Provider::<Http>::try_from(ganache_host)
            .expect("could not instantiate HTTP Provider");
    }

}