async function run() {
    // Import the create function from the new Helia library
    const { createHelia } = await import('helia');
    const { json } = await import('@helia/json');

    // Create a Helia node
    const helia = await createHelia();
    const j = json(helia);
    
    // we added three attributes, add as many as you want!
    const metadata = {
        name: "My First NFT",
        attributes: [
            {
                "trait_type": "Peace",
                "value": "10" 
            },
            {
                "trait_type": "Love",
                "value": "100"
            },
            {
                "trait_type": "Web3",
                "value": "1000"
            }
        ],
        // update the IPFS CID to be your image CID
        image: "https://ipfs.io/ipfs/QmNjLhetmjGnWh1EJi48iFxRAeXuZ3jSG8JKSMaWFPJZKS",
        description: "So much PLW3!"
    };

    // Add the metadata to IPFS
    const cid = await j.add(metadata);
    console.log('Metadata CID:', cid.toString());

    // Stop the Helia node
    await helia.stop();
}

run().catch(console.error);