import React, { Component } from 'react';
import { connect } from 'react-redux';
import LoginForm from './components/LoginForm';
import LoginFeedback from './components/LoginFeedback';
import './styles.scss';

const mapStateToProps = state => ({
});

class ConnectedLogin extends Component {
	constructor(props) {
		super(props);
		this.state = {};
	}
	render() {
		return (
			<div>
				<LoginForm />
				<LoginFeedback />
			</div>
		);
	}
}

const Login = connect(mapStateToProps)(ConnectedLogin);
export default Login;
