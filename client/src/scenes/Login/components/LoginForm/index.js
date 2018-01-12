import React, { Component } from 'react';
import './styles.scss';

class LoginForm extends Component {
	constructor(props) {
		super(props);
		this.state = {};
	
		this.handleChange = this.handleChange.bind(this);
		this.handleSubmit = this.handleSubmit.bind(this);
	}
	handleChange(event) {
		let newState = {};
		newState[event.target.name] = event.target.value;
		this.setState(newState);
		console.log(this.state);
		console.log(newState);
	}
	handleSubmit(event) {
		event.preventDefault();
		console.log(this.state);
		this.props.loginSubmit(this.state);
	}
	render() {
		return (
			<div className="container">
				<form className="form-signin" onSubmit={this.handleSubmit}>
					<h2 className="form-signin-heading">Please sign in</h2>
					<label htmlFor="inputEmail" className="sr-only">Email address</label>
					<input type="email" name="username" id="inputEmail" className="form-control" placeholder="Email address" required autoFocus  onChange={this.handleChange} />
					<label htmlFor="inputPassword" className="sr-only">Password</label>
					<input type="password" id="inputPassword" className="form-control" placeholder="Password" required />
					<button className="btn btn-lg btn-primary btn-block" type="submit">Sign in</button>
				</form>
			</div>
		);
	}
}

export default LoginForm;
