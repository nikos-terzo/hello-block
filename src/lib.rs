use ini::Ini;
use ethers::solc::{Solc, CompilerOutput};
use ethers::solc::error::SolcError;
use ethers::etherscan::Client;
use ethers::providers::{Provider, Http, Middleware};
use ethers::signers::{LocalWallet, WalletError};
use ethers::middleware::SignerMiddleware;
use ethers::abi::Abi;
use ethers::contract::{Contract, ContractFactory, ContractError};
use ethers::types::{Address, H256, BlockId};

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

fn get_wallet(ini_file: &str) -> Result<LocalWallet, WalletError>  {
    let conf = Ini::load_from_file(ini_file)
        .expect("could not load ini file");
    
    let key = "private_key";
    conf.section(None::<String>).unwrap().get(key)
        .expect(&format!("could not find key: {}", key))
        .parse::<LocalWallet>()
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

fn provider_get_contract(sol_file: &str, contact: &Contact, provider: Provider<Http>) -> Contract<Provider<Http>> {
    let compiled = get_compiler_output(sol_file)
        .expect("could not generate compiler output");
    let abi = get_compiled_abi(&compiled, sol_file)
        .expect("abi is None");

    Contract::new(contact.contract_address, abi.to_owned(), provider)
}

async fn upload_contract(sol_file: &str, client: std::sync::Arc<SignerMiddleware<Provider<Http>, LocalWallet>>) -> Result<Contract<SignerMiddleware<Provider<Http>, LocalWallet>>, ContractError<SignerMiddleware<Provider<Http>, LocalWallet>>> {
    
    let compiled = get_compiler_output(sol_file)
        .expect("could not generate compiler output");
    let contract = compiled.get(sol_file, "Chatter")
        .expect("could not find contract Chatter");

    let factory = ContractFactory::new(
        contract.abi.expect("contract.abi is None").clone(),
        contract.bin.expect("contract.bin is None").clone(),
        client
        );

    factory
        .deploy(())
        .expect("could not create deployment transaction")
        .confirmations(0usize)
        .send()
        .await
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn chatter_compiles() {

        // relative path, bad practice
        let compiled = get_compiler_output("./contract_src/Chatter.sol")
            .expect("could not generate compiler output");
        
        assert!(!compiled.has_error(), "errors: {:?}", compiled.errors);
    }

    #[test]
    fn ganache_url_parses_from_ini() {
        // relative path, bad practice
        let ganache_url = get_ganache_url("./Chatter1-conf.ini");
        assert_eq!(&ganache_url[..7], "http://", "ganache_url did not start with \"http://\"");
    }
    
    #[test]
    fn provider_instatiates() {
        // relative path, bad practice
        get_provider("./Chatter1-conf.ini")
            .expect("could not instantiate HTTP Provider");
    }

    #[test]
    fn has_parsable_contacts() {
        let contacts = get_contacts("./Chatter1-conf.ini");
        assert!(contacts.len() > 0);
    }

    #[tokio::test]
    async fn contacts_addresses_exist() {
        let provider = get_provider("./Chatter1-conf.ini")
            .expect("could not instantiate HTTP Provider");
        let contacts = get_contacts("./Chatter1-conf.ini");

        let accounts = provider.get_accounts()
            .await
            .expect("could not retrieve accounts");
        for contact in contacts {
            let code = provider.get_code(contact.contract_address, None::<BlockId>)
                .await
                .expect(&format!("{}'s contract address not found", contact.friendly_name));
            assert!(accounts.contains(&contact.client_address), "{}'s client address not found", contact.friendly_name);
            assert_ne!(code.0.len(), 0, "{}'s code not found", contact.friendly_name);
        }
    }

    #[tokio::test] #[ignore]    // call from provider requires constant function not existing in contract
    async fn contract_exists_from_provider() {
        let provider = get_provider("./Chatter1-conf.ini")
            .expect("could not instantiate HTTP Provider");
        
        let contacts = get_contacts("./Chatter1-conf.ini");

        for contact in contacts {
            let contract = provider_get_contract("./contract_src/Chatter.sol", &contact, provider.clone());

            let contract_call = contract
                .method::<_, H256>("messageMe", "hi".to_owned())    // provider can only call constant functions, not this
                .expect(&format!("could not create contract call for {}", contact.friendly_name));

            // The following will try to send a message to a "Mutable" contract method and it should fail
            // even if the contract exists, as it does not have an address_from to pay the gas fee.
            // The contract is created using provider, not a client. Maybe call instead of send saves the day?
            // The above are all speculations based on my understanding of evm and contracts
            let _tx_hash = contract_call.call().await
                .expect(&format!("{}'s contract could not be called", contact.friendly_name));
        }
    }

    #[tokio::test] #[ignore]
    async fn contracts_upload() {
        let provider = get_provider("./Chatter1-conf.ini")
            .expect("could not instantiate HTTP Provider");
        
        let client1 = SignerMiddleware::new(
            provider.clone(),
            get_wallet("Chatter1-conf.ini")
                .expect("could not parse private key")
            );
        let client1 = std::sync::Arc::new(client1);
        let contract1 = upload_contract("./contract_src/Chatter.sol", std::sync::Arc::clone(&client1));

        let client2 = SignerMiddleware::new(
            provider,
            get_wallet("Chatter2-conf.ini")
                .expect("could not parse private key")
            );
        let client2 = std::sync::Arc::new(client2);
        let contract2 = upload_contract("./contract_src/Chatter.sol", std::sync::Arc::clone(&client2));

        let contract1 = contract1.await.expect("could not upload contract2");
        println!("Chatter1: Share the below to your friends, for them to chat with you!");
        println!("contract_address client_address: {} {}", hex::encode(contract1.address()), hex::encode(client1.address()));
        let contract2 = contract2.await.expect("could not upload contract2");
        println!("Chatter2: Share the below to your friends, for them to chat with you!");
        println!("contract_address client_address: {} {}", hex::encode(contract2.address()), hex::encode(client2.address()));
    }

}