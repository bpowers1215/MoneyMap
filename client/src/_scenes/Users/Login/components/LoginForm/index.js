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
					<div className="field">
						<label className="label">Email Address</label>
						<div className="control has-icons-left has-icons-right">
							<input type="email" name="email" id="inputEmail" className="input" placeholder="Email address" required autoFocus onChange={this.handleChange} />
							<span className="icon is-small is-left">
								<i className="fas fa-envelope"></i>
							</span>
						</div>
					</div>

					<div className="field">
						<label className="label">Email</label>
						<div className="control has-icons-left has-icons-right">
							<input type="password" name="password" id="inputPassword" className="input" placeholder="Password" required onChange={this.handleChange} />
							<span className="icon is-small is-left">
								<i className="fas fa-lock"></i>
							</span>
						</div>
					</div>

					<div className="field is-grouped">
						<div className="control">
							<button className="button is-info" type="submit">Log in</button>
						</div>
					</div>
				</form>
			</div>
		);
	}
}

const LoginForm = connect(null, mapDispatchToProps)(ConnectedLoginForm);
export default LoginForm;
