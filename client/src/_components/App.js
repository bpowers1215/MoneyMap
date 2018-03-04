import React, { Component } from 'react';
import { Route } from 'react-router-dom';
import { Link } from 'react-router-dom';
import Welcome from '~/_scenes/Welcome';
import Login from '~/_scenes/Users/Login';
import Account from '~/_scenes/Users/Account';
import './App.scss';

class App extends Component {
	render() {
		return (
			<div className="App">
				<nav className="navbar is-transparent">
					<div className="navbar-brand">
						<a className="navbar-item" href="https://bulma.io">
							<img src="https://bulma.io/images/bulma-logo.png" alt="Bulma: a modern CSS framework based on Flexbox" width="112" height="28"/>
						</a>
						<div class="navbar-burger burger" data-target="navbarExampleTransparentExample">
							<span></span>
							<span></span>
							<span></span>
						</div>
					</div>

					<div className="navbar-menu">
						<div className="navbar-start"></div>

						<div className="navbar-end">
							<div className="navbar-item">
								<div className="field">
									<p className="control">
										<a className="button is-outlined is-primary" href="login">
											<span>Login</span>
										</a>
									</p>
								</div>
							</div>
						</div>
					</div>
				</nav>

				<section className="hero is-primary is-medium">
					<div className="hero-body">
						<div className="container">
						<h1 className="title">
							Money Map
						</h1>
						<h2 className="subtitle">
							a finance management solution
						</h2>
						</div>
					</div>
				</section>
				<Route exact={true} path="/" component={Welcome} />
				<Route exact={true} path="/login" component={Login} />
				<Route exact={true} path="/account" component={Account} />
			</div>
		);
	}
}

export default App;
