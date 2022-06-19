pub use chattercontract_mod::*;
#[allow(clippy::too_many_arguments)]
mod chattercontract_mod {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "ChatterContract was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static CHATTERCONTRACT_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\"}],\"name\":\"HasMessage\",\"type\":\"event\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\"}],\"name\":\"getMessages\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\"}],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"text\",\"type\":\"string\"}],\"name\":\"messageMe\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"}]") . expect ("invalid abi")
        });
    #[derive(Clone)]
    pub struct ChatterContract<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for ChatterContract<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for ChatterContract<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ChatterContract))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> ChatterContract<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract = ethers::contract::Contract::new(
                address.into(),
                CHATTERCONTRACT_ABI.clone(),
                client,
            );
            Self(contract)
        }
        #[doc = "Calls the contract's `getMessages` (0xb2f71322) function"]
        pub fn get_messages(
            &self,
            from: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([178, 247, 19, 34], from)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `messageMe` (0xaea924ce) function"]
        pub fn message_me(&self, text: String) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([174, 169, 36, 206], text)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `HasMessage` event"]
        pub fn has_message_filter(&self) -> ethers::contract::builders::Event<M, HasMessageFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, HasMessageFilter> {
            self.0.event_with_filter(Default::default())
        }
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "HasMessage", abi = "HasMessage(address)")]
    pub struct HasMessageFilter {
        pub from: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getMessages`function with signature `getMessages(address)` and selector `[178, 247, 19, 34]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getMessages", abi = "getMessages(address)")]
    pub struct GetMessagesCall {
        pub from: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `messageMe`function with signature `messageMe(string)` and selector `[174, 169, 36, 206]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "messageMe", abi = "messageMe(string)")]
    pub struct MessageMeCall {
        pub text: String,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ChatterContractCalls {
        GetMessages(GetMessagesCall),
        MessageMe(MessageMeCall),
    }
    impl ethers::core::abi::AbiDecode for ChatterContractCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <GetMessagesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ChatterContractCalls::GetMessages(decoded));
            }
            if let Ok(decoded) =
                <MessageMeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ChatterContractCalls::MessageMe(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ChatterContractCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ChatterContractCalls::GetMessages(element) => element.encode(),
                ChatterContractCalls::MessageMe(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ChatterContractCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ChatterContractCalls::GetMessages(element) => element.fmt(f),
                ChatterContractCalls::MessageMe(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<GetMessagesCall> for ChatterContractCalls {
        fn from(var: GetMessagesCall) -> Self {
            ChatterContractCalls::GetMessages(var)
        }
    }
    impl ::std::convert::From<MessageMeCall> for ChatterContractCalls {
        fn from(var: MessageMeCall) -> Self {
            ChatterContractCalls::MessageMe(var)
        }
    }
}
