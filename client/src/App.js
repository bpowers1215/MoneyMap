import React, { Component } from 'react';
import logo from './logo.svg';
import Login from './_scenes/Users/Login';
import Alerts from './_components/alerts';
import './App.css';

class App extends Component {
  render() {
    return (
      <div className="App">
        <header className="App-header">
          <img src={logo} className="App-logo" alt="logo" />
          <h1 className="App-title">Welcome to Money Map</h1>
        </header>
        <Alerts />
        <Login />
      </div>
    );
  }
}

export default App;
