import React from 'react';
import ReactDOM from 'react-dom';
import './index.css';
import App from './App';
import getConfig from './config.js';
import * as nearAPI from 'near-api-js';
import reportWebVitals from './reportWebVitals';
import { createRoot } from 'react-dom/client';
import { Buffer } from "buffer"; global.Buffer = Buffer;

// initializing contract
async function initContract() {
  const nearConfig = getConfig(process.env.NODE_ENV || 'testnet');
  const near = await nearAPI.connect({
    deps: {
      keyStore: new nearAPI.keyStores.BrowserLocalStorageKeyStore()
    },
    ...nearConfig
  })

  // needed to access wallet
  const walletConnection = new nearAPI.WalletConnection(near);

  // load in account data
  let currentUser;
  if (walletConnection.getAccountId()) {
      currentUser = {
        accountId: walletConnection.getAccountId(),
        balance: (await walletConnection.account().state()).amount
      }
  }

  // initializing our contract APIs by contract name and configuration
  const contract = await new nearAPI.Contract(walletConnection.account(), nearConfig.contractName,{
    viewMethods: ["get_bulletin", "get_bulletins"],
    changeMethods: ["new_bulletinpost"],
    sender: walletConnection.getAccountId()
  })

  return { contract, currentUser, nearConfig, walletConnection };

}

window.nearInitPromise = initContract()
.then(({ contract, currentUser, nearConfig, walletConnection}) => {
  const container = document.getElementById('root');
  const root = createRoot(container);
  root.render(<App contract={contract}
        currentUser={currentUser}
        nearConfig={nearConfig}
        wallet={walletConnection}/>)
});

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();
