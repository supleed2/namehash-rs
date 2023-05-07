use std::io::prelude::*;

use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Get the namehash of a single domain
    Domain {
        /// Domain to get namehash of
        domain: String,
    },
    /// Get the namehashes of many domains at once
    File {
        /// Path to input file, domains to hash with 1 per line
        input: std::path::PathBuf,
        /// File to save hashes to, stdout if not given
        #[arg(short, long, value_name = "FILE")]
        output: Option<std::path::PathBuf>,
    },
}

fn main() -> std::io::Result<()> {
    match Cli::parse().command {
        Commands::Domain { domain } => {
            println!("{domain}: 0x{}", hex::encode(namehash(&domain)));
            Ok(())
        }
        Commands::File { input, output } => match output {
            None => {
                let input = std::fs::File::open(input)?;
                for line in std::io::BufReader::new(input).lines() {
                    match line {
                        Ok(domain) => {
                            println!("{domain}: 0x{}", hex::encode(namehash(&domain)))
                        }
                        Err(error) => eprintln!("Error: {error}"),
                    }
                }
                Ok(())
            }
            Some(output) => {
                let input = std::fs::File::open(input)?;
                let mut outstr = String::new();
                for line in std::io::BufReader::new(input).lines() {
                    match line {
                        Ok(domain) => outstr
                            .push_str(&format!("{domain}: 0x{}\n", hex::encode(namehash(&domain)))),
                        Err(error) => eprintln!("Error: {error}"),
                    }
                }
                if let Some(folder) = std::path::Path::new(&output).parent() {
                    std::fs::create_dir_all(folder).unwrap();
                }
                write!(std::fs::File::create(output)?, "{}", outstr)
            }
        },
    }
}

fn keccak256(bytes: &[u8]) -> [u8; 32] {
    use tiny_keccak::{Hasher, Keccak};
    let mut hasher = Keccak::v256();
    hasher.update(bytes);
    let mut hash = [0u8; 32];
    hasher.finalize(&mut hash);
    hash
}

fn namehash(name: &str) -> Vec<u8> {
    if name.is_empty() {
        return vec![0u8; 32];
    }
    let mut hash = vec![0u8; 32];
    for label in name.rsplit('.') {
        hash.append(&mut keccak256(label.as_bytes()).to_vec());
        hash = keccak256(hash.as_slice()).to_vec();
    }
    hash
}

#[cfg(test)]
mod test {
    use super::namehash;

    #[test]
    fn test_namehash() {
        // Test cases, same than used @ EIP137 `https://github.com/ethereum/EIPs/blob/master/EIPS/eip-137.md`
        let cases = vec![
            (
                "",
                &[
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                ],
            ),
            (
                "crypto",
                &[
                    0x0f, 0x4a, 0x10, 0xa4, 0xf4, 0x6c, 0x28, 0x8c, 0xea, 0x36, 0x5f, 0xcf, 0x45,
                    0xcc, 0xcf, 0x0e, 0x9d, 0x90, 0x1b, 0x94, 0x5b, 0x98, 0x29, 0xcc, 0xdb, 0x54,
                    0xc1, 0x0d, 0xc3, 0xcb, 0x7a, 0x6f,
                ],
            ),
            (
                "eth",
                &[
                    0x93, 0xcd, 0xeb, 0x70, 0x8b, 0x75, 0x45, 0xdc, 0x66, 0x8e, 0xb9, 0x28, 0x1,
                    0x76, 0x16, 0x9d, 0x1c, 0x33, 0xcf, 0xd8, 0xed, 0x6f, 0x4, 0x69, 0xa, 0xb,
                    0xcc, 0x88, 0xa9, 0x3f, 0xc4, 0xae,
                ],
            ),
            (
                "foo.eth",
                &[
                    0xde, 0x9b, 0x9, 0xfd, 0x7c, 0x5f, 0x90, 0x1e, 0x23, 0xa3, 0xf1, 0x9f, 0xec,
                    0xc5, 0x48, 0x28, 0xe9, 0xc8, 0x48, 0x53, 0x98, 0x1, 0xe8, 0x65, 0x91, 0xbd,
                    0x98, 0x1, 0xb0, 0x19, 0xf8, 0x4f,
                ],
            ),
        ];

        for (name, expected_namehash) in cases {
            let namehash: &[u8] = &namehash(name);
            assert_eq!(namehash, expected_namehash);
        }
    }
}
