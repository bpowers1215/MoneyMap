import React, { Component } from 'react';
import { connect } from 'react-redux';
import UserActions from '~/_data/users/actions';
import './styles.scss';

const mapDispatchToProps = dispatch => {
	return {
		login: ({email, password}) => dispatch(UserActions.login(email, password))
	};
};

class ConnectedLoginForm extends Component {
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
	}
	handleSubmit(event) {
		event.preventDefault();
		this.props.login(this.state);
	}
	render() {
		return (
			<div className="container">
				<form className="form-signin" onSubmit={this.handleSubmit}>
					<h2 className="form-signin-heading">Please sign in</h2>
					<label htmlFor="inputEmail" className="sr-only">Email address</label>
					<input type="email" name="email" id="inputEmail" className="form-control" placeholder="Email address" required autoFocus onChange={this.handleChange} />
					<label htmlFor="inputPassword" className="sr-only">Password</label>
					<input type="password" name="password" id="inputPassword" className="form-control" placeholder="Password" required onChange={this.handleChange} />
					<button className="btn btn-lg btn-primary btn-block" type="submit">Sign in</button>
				</form>
			</div>
		);
	}
}

const LoginForm = connect(null, mapDispatchToProps)(ConnectedLoginForm);
export default LoginForm;
