const range = function* (total = 0, step = 1, from = 0) {
	for (let i = 0; i < total; yield from + i++ * step) { /* Necessary for yield to work */ }
};

const ethers = await import("https://esm.sh/ethers@6.3.0");
const polygonscan = new ethers.EtherscanProvider(137, "<POLYGONSDCAN_API_KEY>");
const udInterface = await polygonscan.getContract("0x0301cc5242A1F039799E8F806302Dc2140421971") || Deno.exit(1);
const alchemy = new ethers.AlchemyProvider("matic", "<ALCHEMY_API_KEY>");
const udContract = new ethers.Contract("0xa9a6A3626993D487d2Dbda3173cf58cA1a9D9e9f", udInterface.interface.fragments, alchemy);

for (const domain of range(1000)) {
	const noTld = domain.toString().padStart(3, "0") + ".";
	for (const tld of ["888", "anime", "bitcoin", "blockchain", "coin", "crypto", "dao", "hi", "klever", "kresus", "manga", "nft", "polygon", "wallet", "x", "zil"]) {
		const domainStr = noTld + tld;
		const cmd = await new Deno.Command(".\\namehash.exe", { args: ["domain", domainStr] }).output();
		const hash = new TextDecoder().decode(cmd.stdout).substring(6 + tld.length, 72 + tld.length);
		try {
			const owner = await udContract.ownerOf(hash);
			console.log(`${domainStr.padEnd(14)}: ${owner}`);
		} catch (e) {
			if (e.toString().startsWith(`Error: execution reverted: "ERC721: invalid token ID"`)) {
				console.error(`${domainStr.padEnd(14)}: Domain is not minted yet`);
			} else {
				console.error(`${domainStr.padEnd(14)}: ${e}`);
			}
		}
	}
}
