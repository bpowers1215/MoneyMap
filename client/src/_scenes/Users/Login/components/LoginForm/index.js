import React, { Component } from 'react';
import { connect } from 'react-redux';
import UserActions from '~/_data/users/actions';
import { Field } from '~/_components/form';
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
			<form className="form-signin" onSubmit={this.handleSubmit}>
				<Field
					type="email"
					name="email"
					fieldId="inputEmail"
					label="Email"
					controlClasses="control has-icons-left"
					placeholder="Email Address"
					onChange={this.handleChange}>
					<span className="icon is-small is-left">
						<i className="fas fa-envelope"></i>
					</span>
				</Field>

				<Field
					type="password"
					name="password"
					fieldId="inputPassword"
					label="Password"
					controlClasses="control has-icons-left"
					placeholder="Password"
					onChange={this.handleChange}>
					<span className="icon is-small is-left">
						<i className="fas fa-lock"></i>
					</span>
				</Field>

				<div className="field is-grouped">
					<div className="control">
						<button className="button is-info" type="submit">Log in</button>
					</div>
				</div>
			</form>
		);
	}
}

const LoginForm = connect(null, mapDispatchToProps)(ConnectedLoginForm);
export default LoginForm;
