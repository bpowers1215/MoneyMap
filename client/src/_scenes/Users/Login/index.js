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
				<section className="hero is-primary">
					<div className="hero-body">
						<div className="container">
							<h1 className="title">
								Login
							</h1>
						</div>
					</div>
				</section>
				<Alerts />
				<LoginForm />
			</div>
		);
	}
}

const Login = connect(mapStateToProps)(ConnectedLogin);
export default Login;
