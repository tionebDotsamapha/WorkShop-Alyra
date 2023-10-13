#![cfg_attr(not(feature = "std"), no_std, no_main)]
#[ink::contract]
mod ws_alyra {
    // use scale::{Decode, Encode};
    #[derive(Debug)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        JsonSerializationError,
    }
    use ink::env::{debug_println};
    use ink_prelude::{
        format,
        string::{String, ToString},
        vec::Vec,
    };  
    use serde::Deserialize;
    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    
    #[derive(Debug, Deserialize)]
    struct MexcPriceData {
        last: String,
    }
    
    #[derive(Debug, Deserialize)]
    struct MexcData {
        data: Vec<MexcPriceData>,
    }

    #[derive(serde::Deserialize, Debug)]
    struct KucoPriceData {
        data: KucoData,
    }
    #[derive(serde::Deserialize, Debug)]
    struct KucoData {
        price: String,
    }
    #[derive(Deserialize, Debug)]
    struct GateData {
        last: String,
    }
    #[ink(storage)]
    pub struct WsAlyra {
        /// Stores a single `bool` value on the storage.
        value: bool,
    }

    impl WsAlyra {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }
        
        #[ink(message)]
        pub fn get_price_token(&self, token: String,) -> String {
            let mut prices: Vec<i64> = Vec::new();
            
            
            prices = oracle_price(
                "https://www.mexc.com/open/api/v2/market/ticker?symbol=".to_string(),
                token.clone(),
                prices
            );
            prices = oracle_price(
                "https://api.kucoin.com/api/v1/market/orderbook/level1?symbol=".to_string(),
                token.clone(),
                prices
            );            
            prices = oracle_price(
                "https://data.gateapi.io/api2/1/ticker/".to_string(),
                token.clone(),
                prices
            );
            // debug_println!("Planck : {}",convert_str_flto_planck("125.2306".to_string()));
            // debug_println!("unPlanck : {}",convert_planck_to_str_fl(125230600000000));

            fn convert_str_flto_planck(input_string : String)
            -> i64 {
                let mut planck:i64 = 0;
                if let Some(dot_position) = input_string.find('.'){
                    let modified_string = input_string.replace(".", "");
                    let result: Result<i64, _> = modified_string.parse();
                    match result {
                        Ok(number) => {
                        planck = number * 10i64.pow((12-(input_string.len()-dot_position-1)).try_into().unwrap());
                        }
                        Err(e) => {
                            debug_println!("Erreur de conversion : {:?}", e);
                        }
                    }
                }
                planck
            }
            
            fn convert_planck_to_str_fl(num: i64) -> String {
                let num_divided = num / 10i64.pow(12);
                let num_remainder = num % 10i64.pow(12);
            
                let mut number_str = num_divided.to_string();
            
                if num_remainder > 0 {
                    let remainder_str = num_remainder.to_string();
                    let trimmed_remainder = remainder_str.trim_end_matches('0');
                    if !trimmed_remainder.is_empty() {
                        number_str.push('.');
                        number_str.push_str(trimmed_remainder);
                    }
                }
            
                debug_println!("formatted: {:?}", number_str.clone());
                number_str
            }
            
            fn oracle_price(url : String, token : String, mut prices: Vec<i64>)
            -> Vec<i64> {
                let the_url : String;
                match (&url.contains("mexc"), &url.contains("kucoin"), &url.contains("gate")) {
                    (true, _, _) => {
                       the_url = format!("{}{}_USDT",url,token.to_uppercase());
                    }
                    (_, true, _) => {
                        the_url = format!("{}{}-USDT",url,token.to_uppercase());
                    }
                    (_, _, true) => {
                        the_url = format!("{}{}_USDT",url,token.to_uppercase());
                    }
                    _ => {
                        debug_println!("This URL is not managed : {}", url);
                        return prices.clone();
                    }
                }
                
                let ticker: String = match String::from_utf8(pink::http_get!(the_url.clone()).body) {
                    Ok(s) => s,
                    Err(e) => {
                        debug_println!("Error converting to String: {:?}", e);
                        return prices.clone(); // Gérer l'erreur de conversion
                    }
                };
                
                match (&url.contains("mexc"), &url.contains("kucoin"), &url.contains("gate")) {
                    (true, _, _) => {
                        let price_result : Result<MexcData, _> = pink_json::from_str(&ticker).map_err(|_| Error::JsonSerializationError);
                        if let Ok(price_data) = price_result {
                            if let Some(price) = price_data.data.last() {
                                let price_str = price.last.clone();
                                let price:i64 = convert_str_flto_planck(price_str.clone());
                                prices.push(price);
                                debug_println!("price_str : {:?}",price_str);
                            }
                        } else {
                            // Gérez l'erreur ici si la désérialisation a échoué
                            debug_println!("Error deserializing JSON {}", ticker);
                        }
                    }
                    (_, true, _) => {
                        let price_result : Result<KucoPriceData, _> = pink_json::from_str(&ticker).map_err(|_| Error::JsonSerializationError);
                        if let Ok(price_data) = price_result {
                            let price_str = price_data.data.price;
                            let price:i64 = convert_str_flto_planck(price_str);
                            prices.push(price);
                        }
                    }
                    (_, _, true) => {
                        let price_result : Result<GateData, _> = pink_json::from_str(&ticker).map_err(|_| Error::JsonSerializationError);
                        debug_println!("tester : {:?}", the_url);
                        if let Ok(price_data) = price_result {
                            let price_str = price_data.last;
                            let price:i64 = convert_str_flto_planck(price_str);
                            prices.push(price);
                        }
                    }
                    _ => {
                        debug_println!("This URL is not managed : {}", url);
                        return prices.clone();
                    }
                }

                return prices;
            }

            debug_println!("prices : {:?}", prices);
            let average_price: i64 = prices.iter().sum::<i64>()/ prices.len() as i64;
            debug_println!("Moyenne : {}", average_price);
            let result : String = format!("1 {} = {} USDT", token, convert_planck_to_str_fl(average_price) );
            result
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let ws_alyra = WsAlyra::default();
            assert_eq!(ws_alyra.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            pink_extension_runtime::mock_ext::mock_all_ext();
            let mut ws_alyra = WsAlyra::new(false);
            assert_eq!(ws_alyra.get(), false);
            ws_alyra.flip();
            assert_eq!(ws_alyra.get(), true);
            let price = ws_alyra.get_price_token("pha".to_string());
            debug_println!("price : {:?}", price);
        }
    }

    /// This is how you'd write end-to-end (E2E) or integration tests for ink! contracts.
    ///
    /// When running these you need to make sure that you:
    /// - Compile the tests with the `e2e-tests` feature flag enabled (`--features e2e-tests`)
    /// - Are running a Substrate node which contains `pallet-contracts` in the background
    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// A helper function used for calling contract messages.
        use ink_e2e::build_message;

        /// The End-to-End test `Result` type.
        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        /// We test that we can upload and instantiate the contract using its default constructor.
        #[ink_e2e::test]
        async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = WsAlyraRef::default();

            // When
            let contract_account_id = client
                .instantiate("ws_alyra", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // Then
            let get = build_message::<WsAlyraRef>(contract_account_id.clone())
                .call(|ws_alyra| ws_alyra.get());
            let get_result = client.call_dry_run(&ink_e2e::alice(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), false));

            Ok(())
        }

        /// We test that we can read and write a value from the on-chain contract contract.
        #[ink_e2e::test]
        async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = WsAlyraRef::new(false);
            let contract_account_id = client
                .instantiate("ws_alyra", &ink_e2e::bob(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let get = build_message::<WsAlyraRef>(contract_account_id.clone())
                .call(|ws_alyra| ws_alyra.get());
            let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), false));

            // When
            let flip = build_message::<WsAlyraRef>(contract_account_id.clone())
                .call(|ws_alyra| ws_alyra.flip());
            let _flip_result = client
                .call(&ink_e2e::bob(), flip, 0, None)
                .await
                .expect("flip failed");

            // Then
            let get = build_message::<WsAlyraRef>(contract_account_id.clone())
                .call(|ws_alyra| ws_alyra.get());
            let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), true));

            Ok(())
        }
    }
}
