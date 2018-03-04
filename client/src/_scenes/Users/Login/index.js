import React, { Component } from 'react';
import { connect } from 'react-redux';
import LoginForm from './components/LoginForm';
import Alerts from '~/_components/alerts';
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
				<Alerts />
				<LoginForm />
			</div>
		);
	}
}

const Login = connect(mapStateToProps)(ConnectedLogin);
export default Login;
