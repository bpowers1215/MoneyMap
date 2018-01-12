import React, { Component } from 'react';
import LoginForm from './components/LoginForm';
import LoginFeedback from './components/LoginFeedback';
import './styles.scss';

class Login extends Component {
	constructor(props) {
		super(props);
		this.state = {};
	
		this.loginSubmit = this.loginSubmit.bind(this);
	}
	loginSubmit(login) {
		this.setState(login);
	}
	render() {
		return (
			<div>
				<LoginForm loginSubmit={this.loginSubmit} />
				<LoginFeedback data={this.state} />
			</div>
		);
	}
}

export default Login;
