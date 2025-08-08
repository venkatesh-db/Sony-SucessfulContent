

use clap::{Subcommand, Args};

use ed25519_dalek::{Keypair, Signature, Signer, Verifier};
use rand::rngs::OsRng;

#[derive(Debug, Args)] // <-- add this
pub struct WalletCmd {
    #[command(subcommand)]
    pub cmd: WalletSubCmd,
}

#[derive(Debug, Subcommand)]
pub enum WalletSubCmd {
    Gen(GenArgs),
    Sign(SignArgs),
    Verify(VerifyArgs),
}

#[derive(Debug, Args)]
pub struct GenArgs {}

#[derive(Debug, Args)]
pub struct SignArgs {
    pub msg: String,
}

#[derive(Debug, Args)]
pub struct VerifyArgs {
    pub msg: String,
    pub sig_hex: String,
}

pub fn run(cmd: WalletCmd) {
    let mut csprng = OsRng {};
    let keypair = Keypair::generate(&mut csprng);

    match cmd.cmd {
        WalletSubCmd::Gen(_) => {
            println!("Public: {}", hex::encode(keypair.public.to_bytes()))
        }
        WalletSubCmd::Sign(args) => {
            let sig: Signature = keypair.sign(args.msg.as_bytes());
            println!("Signature (hex): {}", hex::encode(sig.to_bytes()));
        }
        WalletSubCmd::Verify(args) => {
            let sig = Signature::from_bytes(&hex::decode(args.sig_hex).unwrap()).unwrap();
            let ok = keypair.public.verify(args.msg.as_bytes(), &sig).is_ok();
            println!("Verify OK? {}", ok);
        }
    }
}
