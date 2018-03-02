import React, { Component } from 'react';
import { Route } from 'react-router-dom';
import logo from '~/logo.svg';
import Welcome from '~/_scenes/Welcome';
import Login from '~/_scenes/Users/Login';
import Account from '~/_scenes/Users/Account';
import Alerts from '~/_components/alerts';
import './App.css';

class App extends Component {
	render() {
		return (
			<div className="App">
				<header className="App-header">
					<img src={logo} className="App-logo" alt="logo" />
					<h1 className="App-title">Money Map</h1>
				</header>
				<Alerts />
				<Route exact={true} path="/" component={Welcome} />
				<Route exact={true} path="/login" component={Login} />
				<Route exact={true} path="/account" component={Account} />
			</div>
		);
	}
}

export default App;
