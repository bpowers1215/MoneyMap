import React, { Component } from 'react';
import { Link } from 'react-router-dom';
import { connect } from 'react-redux'
import './styles.scss';

const mapStateToProps = state => ({
});

class ConnectedWelcome extends Component {
	constructor(props) {
		super(props);
		this.state = {};
	}
	render() {
		return (
			<div className="container">
				<h2 className="form-signin-heading">Welcome. Learn all about Money Map here.</h2>
				Ready to <Link to={'/login'} >Login</Link>?
			</div>
		);
	}
}

const Welcome = connect(mapStateToProps)(ConnectedWelcome);
export default Welcome;
