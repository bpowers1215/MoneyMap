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
			<section className="section">
				<div className="container">
					<h2 className="title is-2">Welcome.</h2>
					<p>Learn all about Money Map here.</p>
					Ready to <Link to={'/login'} >Login</Link>?
				</div>
			</section>
		);
	}
}

const Welcome = connect(mapStateToProps)(ConnectedWelcome);
export default Welcome;
