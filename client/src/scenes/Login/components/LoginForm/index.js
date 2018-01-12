import React, { Component } from 'react';

class LoginForm extends Component {
	render() {
		return (
			<div className="login-form">
				<h2>Login</h2>
				<ul>
					<li><input type="text" name="username" /></li>
					<li><input type="password" name="password" /></li>
				</ul>
			</div>
		);
	}
}

export default LoginForm;
