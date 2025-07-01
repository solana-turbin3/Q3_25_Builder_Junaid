
#[cfg(test)] 
mod tests {
    // use solana_sdk;
    // use solana_sdk::{signature::{Keypair, Signer}, pubkey::Pubkey};

    use solana_client::rpc_client::RpcClient;
    use solana_sdk::signature::read_keypair_file;
    const RPC_URL: &str = "https://api.devnet.solana.com";

    // #[test]
    // fn keygen() {
    //     let kp = Keypair::new();
    //     println!("You've generated a new Solana wallet: {}", kp.pubkey().to_string());
    //     println!("");
    //     println!("To save your wallet, copy and paste the following into a JSON file:");
    //     println!("{:?}", kp.to_bytes());
    // }

    #[test]
    fn airdrop() {
        let keypair = read_keypair_file("./dev_wallet.json").expect("Couldn't find wallet file");

        // we'll establish a connection to Solana devnet using the const we defined above
        let client = RpcClient::new(RPC_URL);

        // We're going to claim 2 devnet SOL tokens (2 billion lamports)
        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
            Ok(sig) => {
                println!("Success! Check your TX here:");
                println!("https://explorer.solana.com/tx/{}?cluster=devnet", sig);
            }
            Err(err) => {
                println!("Airdrop failed: {}", err);
            }
        }
    }
    
    #[test]
    fn transfer_sol() {}



    // use bs58; use std::io::{self, BufRead};
    // #[test]
    // fn base58_to_wallet() {
    //     println!("Input your private key as a base58 string:");
    //     let stdin = io::stdin();
    //     let base58 = stdin.lock().lines().next().unwrap().unwrap();
    //     println!("Your wallet file format is:");
    //     let wallet = bs58::decode(base58).into_vec().unwrap();
    //     println!("{:?}", wallet);
    // }

    // #[test]
    // fn wallet_to_base58() {
    //     println!("Input your private key as a JSON byte array (e.g. [12,34,...]):");
    //     let stdin = io::stdin();
    //     let wallet = stdin
    //         .lock()
    //         .lines()
    //         .next()
    //         .unwrap()
    //         .unwrap()
    //         .trim_start_matches('[')
    //         .trim_end_matches(']')
    //         .split(',')
    //         .map(|s| s.trim().parse::<u8>().unwrap())
    //         .collect::<Vec<u8>>();
    //     println!("Your Base58-encoded private key is:");
    //     let base58 = bs58::encode(wallet).into_string();
    //     println!("{:?}", base58);
    // }

}
    
