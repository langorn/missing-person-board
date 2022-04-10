import 'regenerator-runtime/runtime';
import React from 'react';
import PropTypes from 'prop-types';

import logo from './logo.svg';
import './App.css';
import CreateBulletin from './components/CreateBulletin';
import BulletinList from './components/BulletinList';


const App = ({ contract, currentUser, nearConfig, wallet}) => {

    const signIn = async () => {
     await wallet.requestSignIn(
        nearConfig.contractName,
        'NEAR toDo List'
      );
    };

    const signOut = () => {
      wallet.signOut();
      window.location.replace(window.location.origin + window.location.pathname);
    }

    return (
      <>
      <h1>Near</h1>
      {
        currentUser 
        ? <div>
          <h2>
            Account ID : { currentUser.accountId }
            {" "}
            <button onClick={signOut}> Log out </button>
            <CreateBulletin contract={contract} />
            <BulletinList contract={contract}/>
          </h2>
        </div>
        :
        <div>
          Sign In 
          {" "}
          <button onClick={signIn}>Login</button>
        </div>
      }
      
      </>


    );




};
// function App() {
//   return (
//     <div className="App">
//       <header className="App-header">
//         <img src={logo} className="App-logo" alt="logo" />
//         <p>
//           Edit <code>src/App.js</code> and save to reload.
//         </p>
//         <a
//           className="App-link"
//           href="https://reactjs.org"
//           target="_blank"
//           rel="noopener noreferrer"
//         >
//           Learn React
//         </a>
//       </header>
//     </div>
//   );
// }
// App.propTypes = {
//   contract: PropTypes.shape({
//     create: PropTypes.func.isRequired,
//     get: PropTypes.func.isRequired,
//     update: PropTypes.func.isRequired,
//     del: PropTypes.func.isRequired,
//   }).isRequired,
//   currentUser: PropTypes.shape({
//     accountId: PropTypes.string.isRequired,
//     balance: PropTypes.string.isRequired
//   }),
//   nearConfig: PropTypes.shape({
//     contractName: PropTypes.string.isRequired
//   }).isRequired,
//   wallet: PropTypes.shape({
//     requestSignIn: PropTypes.func.isRequired,
//     signOut: PropTypes.func.isRequired
//   }).isRequired
// };
export default App;
