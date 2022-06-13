use ini::Ini;
use ethers::solc::{Solc, CompilerOutput};
use ethers::solc::error::SolcError;
use ethers::providers::{Provider, Http, Middleware};
use ethers::abi::Abi;
use ethers::contract::Contract;
use ethers::types::{Address, H256};

#[derive(Debug)]
pub struct Contact {
    pub friendly_name: String,
    pub contract_address: Address,
    pub client_address: Address
}

fn get_ganache_url(ini_file: &str) -> String {
    let conf = Ini::load_from_file(ini_file)
        .expect("could not load ini file");
    
    let key = "Ganache URL";
    conf.section(None::<String>).unwrap().get(key)
        .expect(&format!("could not find key: {}", key))
        .to_string()
}

fn get_provider(ini_file: &str) -> Result<Provider<Http>, url::ParseError> {
    let url = get_ganache_url(ini_file);
    Provider::<Http>::try_from(url)
}

fn get_contacts(ini_file: &str) -> std::vec::Vec<Contact> {
    let mut contacts: std::vec::Vec<Contact> = std::vec::Vec::<Contact>::new();

    let conf = Ini::load_from_file(ini_file)
        .expect("could not load ini file");

    let contacts_section_ini = conf.section(Some("Contacts"))
        .expect("could not find section Contacts");
    
    for (key, value) in contacts_section_ini.iter() {
        let addresses: std::vec::Vec<&str> = value.split(" ").collect();
        assert_eq!(addresses.len(), 2, "Contact {} should have 2 addresses", key);

        let contract_address = addresses[0].parse::<Address>()
            .expect(&format!("Contact {}: could not parse contract address", key));
        let client_address = addresses[1].parse::<Address>()
            .expect(&format!("Contact {}: could not parse client address", key));

        contacts.push(Contact{
            friendly_name: key.to_string(),
            contract_address,
            client_address
        });
    }

    contacts
}

fn get_compiler_output(sol_file: &str) -> Result<CompilerOutput, SolcError> {
    Solc::default().compile_source(sol_file)
}

fn get_compiled_abi<'a, 'b>(compiled: &'a CompilerOutput, sol_file: &'b str) -> Option<&'a Abi> {
    compiled.get(sol_file, "Chatter")
        .expect("could not find contract Chatter")
        .abi
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn chatter_compiles() {

        // relative path, bad practice
        let compiled = get_compiler_output("./contract_src/Chatter.sol")
            .expect("could not generate compiler output");
        
        println!("{:?}", compiled.errors);
        assert!(!compiled.has_error())
    }

    #[test]
    fn ganache_url_parses_from_ini() {
        // relative path, bad practice
        let ganache_url = get_ganache_url("./ChangeMe.ini");
        assert_eq!(&ganache_url[..7], "http://", "ganache_url did not start with \"http://\"");
    }
    
    #[test]
    fn provider_instatiates() {
        // relative path, bad practice
        get_provider("./ChangeMe.ini")
            .expect("could not instantiate HTTP Provider");
    }

    #[test]
    fn has_parsable_contacts() {
        let contacts = get_contacts("./ChangeMe.ini");
        assert!(contacts.len() > 0);
    }

    #[tokio::test]
    async fn contacts_addresses_exist() {
        let provider = get_provider("./ChangeMe.ini")
            .expect("could not instantiate HTTP Provider");
        let contacts = get_contacts("./ChangeMe.ini");

        let accounts = provider.get_accounts()
            .await
            .expect("could not retrieve accounts");
        
        for contact in contacts {
            assert!(accounts.contains(&contact.contract_address), "{}'s contract address not found", contact.friendly_name);
            assert!(accounts.contains(&contact.client_address), "{}'s client address not found", contact.friendly_name);
        }
    }

    #[tokio::test]
    async fn contract_exists() {
        let provider = get_provider("./ChangeMe.ini")
            .expect("could not instantiate HTTP Provider");
        
        let address = "6C75C15717887faBfBC482c0d5Dce3659A94dA65".parse::<Address>()
            .expect("could not parse address");

        let sol_file = "./contract_src/Chatter.sol";
        let compiled = get_compiler_output(sol_file)
            .expect("could not generate compiler output");
        let abi = get_compiled_abi(&compiled, sol_file)
            .expect("abi is None");
    
        let contract = Contract::new(address, abi.to_owned(), provider);
        let contract_call = contract
            .method::<_, H256>("messageMe", "hi".to_owned())
            .expect("could not create contract call");
        
        // The following will try to send a message to a "Mutable" contract method and it should fail
        // even if the contract exists, as it does not have an address_from to pay the gas fee?
        let _tx_hash = contract_call.call().await
            .expect("contract could not be called");
    }

}