const contractName = 'rhythm4nft.testnet'//'abel-test.testnet'//'dev-1629431119980-32207067362499';//dev-1621302059600-80550158376073

export default function getConfig(isServer = false) {
  let config = {
    networkId: "testnet",
    nodeUrl: "https://rpc.testnet.near.org",
    walletUrl: "https://wallet.testnet.near.org",
    helperUrl: "https://helper.testnet.near.org",
    contractName,
  };

  if (process.env.REACT_APP_ENV !== "prod") {
    config = {
      ...config,
      GAS: "200000000000000",
      DEFAULT_NEW_ACCOUNT_AMOUNT: "20",
      contractMethods: {
        changeMethods: ["nft_mint", "nft_transfer", "new_default_meta"],
        viewMethods: ["nft_metadata"],
      },
    };
  }
  if (process.env.REACT_APP_ENV === "prod") {
    config = {
      ...config,
      networkId: "mainnet",
      nodeUrl: "https://rpc.mainnet.near.org",
      walletUrl: "https://wallet.near.org",
      helperUrl: "https://helper.mainnet.near.org",
      contractName: "near",
    };
  }

  return config;
};
